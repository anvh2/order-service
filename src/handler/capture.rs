use crate::{proto, cache};

use futures::{TryFutureExt, FutureExt};
use grpcio::{RpcContext, UnarySink};

use super::OrderService;

impl<S> proto::order::OrderService for OrderService<S>
where
    S: cache::OrderStore + Send + Clone + 'static,
{
   fn capture(&mut self, ctx: RpcContext<'_>, req: proto::order::CaptureRequest, sink: UnarySink<proto::order::CaptureResponse>) {
        let _order = proto::order::Order { 
            order_id: 22102500000001,
            order_no: req.order_no,
            order_type: req.order_type, 
            order_time: 1666630662824,
            category: req.category, 
            order_status: 1,
            unknown_fields: todo!(),
            cached_size: todo!(),
        };
        self.store.set(_order);

        let mut resp = proto::order::CaptureResponse::new();
        resp.set_order(_order);

        let f = sink
            .success(resp)
            .map_err(move |e| e)
            .map(|_| ());

        ctx.spawn(f)
   }
}

