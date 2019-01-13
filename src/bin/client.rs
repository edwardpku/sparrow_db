// Copyright 2017 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate grpcio;
extern crate grpcio_proto;
extern crate sparrow_db_model;

use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};
use sparrow_db_model::proto::operation::{GetRequest, GetResponse, PutRequest, PutResponse};
use sparrow_db_model::proto::operation_grpc::SparrowDbClient;

fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");
    let client = SparrowDbClient::new(ch);

    let mut put_req = PutRequest::new();
    put_req.set_key(Vec::from("key1"));
    put_req.set_value(Vec::from("value1"));
    client.put(&put_req);

    let mut get_req = GetRequest::new();
    get_req.set_key(Vec::from("key1"));
    match client.get(&get_req) {
        Ok(value) => println!("OK {}", std::str::from_utf8(value.get_value()).unwrap()),
        Err(e) => println!("Error {}", e)
    };
}
