extern crate grpcio;
extern crate grpcio_proto;
extern crate sparrow_db_model;

use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};
use sparrow_db_model::proto::operation::{GetRequest, PutRequest};
use sparrow_db_model::proto::operation_grpc::SparrowDbClient;

fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");
    let client = SparrowDbClient::new(ch);

    let mut put_req = PutRequest::new();
    put_req.set_key(Vec::from("key1"));
    put_req.set_value(Vec::from("value1"));
    match client.put(&put_req) {
        Err(e) => println!("Error {}", e),
        _ => (),
    }

    let mut get_req = GetRequest::new();
    get_req.set_key(Vec::from("key1"));
    match client.get(&get_req) {
        Ok(value) => println!("OK {}", std::str::from_utf8(value.get_value()).unwrap()),
        Err(e) => println!("Error {}", e),
    };
}
