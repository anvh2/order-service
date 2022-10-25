extern crate protoc_grpcio;

fn main() {
    let root = "src/proto";
    protoc_grpcio::compile_grpc_protos(
        &["order/order_service.proto"], 
        &[root], 
        "src/proto/order", 
        None,
    ).unwrap()
}
