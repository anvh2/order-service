// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_ORDER_SERVICE_CAPTURE: ::grpcio::Method<super::order_service::CaptureRequest, super::order_service::CaptureResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/order.v1.OrderService/Capture",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct OrderServiceClient {
    client: ::grpcio::Client,
}

impl OrderServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        OrderServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn capture_opt(&self, req: &super::order_service::CaptureRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::order_service::CaptureResponse> {
        self.client.unary_call(&METHOD_ORDER_SERVICE_CAPTURE, req, opt)
    }

    pub fn capture(&self, req: &super::order_service::CaptureRequest) -> ::grpcio::Result<super::order_service::CaptureResponse> {
        self.capture_opt(req, ::grpcio::CallOption::default())
    }

    pub fn capture_async_opt(&self, req: &super::order_service::CaptureRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::order_service::CaptureResponse>> {
        self.client.unary_call_async(&METHOD_ORDER_SERVICE_CAPTURE, req, opt)
    }

    pub fn capture_async(&self, req: &super::order_service::CaptureRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::order_service::CaptureResponse>> {
        self.capture_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait OrderService {
    fn capture(&mut self, ctx: ::grpcio::RpcContext, req: super::order_service::CaptureRequest, sink: ::grpcio::UnarySink<super::order_service::CaptureResponse>);
}

pub fn create_order_service<S: OrderService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_ORDER_SERVICE_CAPTURE, move |ctx, req, resp| {
        instance.capture(ctx, req, resp)
    });
    builder.build()
}
