use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures_channel::oneshot;
use futures_executor::block_on;
use grpcio::{ResourceQuota, ChannelBuilder, Environment, ServerBuilder};

use crate::{cache, proto, handler};

use super::base;

pub struct Server {
    name: String,
    host: String,
    port: u16,
}

impl Server {
    pub fn new(name: String, host: String, port: u16) -> Self {
        Self {name, host, port}
    }
}

impl base::Server for Server {
    fn start(&mut self) {
        let env = Arc::new(Environment::new(1));

        let cache = cache::OrderStoreImpl::new();
        let handler = handler::OrderService::new(cache);
        let service = proto::order::create_order_service(handler);

        let quota = ResourceQuota::new(Some(&self.name)).resize_memory(1024*1024);
        let builder = ChannelBuilder::new(env.clone()).set_resource_quota(quota);

        let mut server = ServerBuilder::new(env)
                    .register_service(service)
                    .bind(&self.host, self.port)
                    .channel_args(builder.build_args())
                    .build()
                    .unwrap();

        server.start();

        // graceful shutdown
        let (tx, rx) = oneshot::channel();

        thread::spawn(move || {
            let _ = io::stdin().read(&mut [0]).unwrap();
            tx.send(())
        });

        let _ = block_on(rx);
        let _ = block_on(server.shutdown());
    }


    fn stop(&mut self) {

    }
}