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

extern crate futures;
extern crate grpcio;
extern crate sparrow_db_model;
extern crate rocksdb;

#[macro_use]
extern crate log;

#[path = "../log_util.rs"]
mod log_util;

use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::sync::oneshot;
use futures::Future;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

use sparrow_db_model::proto::operation::{GetRequest, GetResponse, PutRequest, PutResponse, DeleteRequest, DeleteResponse, ScanRequest, ScanResponse, Exception, Exception_Type};
use sparrow_db_model::proto::operation_grpc::{self, SparrowDb};

use rocksdb::{DBIterator, SeekKey, Writable, WriteBatch, DB};

#[derive(Clone)]
struct SparrowDBService;


impl SparrowDb for SparrowDBService {
    fn get(&mut self, ctx: RpcContext, req: GetRequest, sink: UnarySink<GetResponse>) {
        let db = DB::open_default("/tmp/rocks").unwrap();
        let mut resp = GetResponse::new();
        match db.get(req.get_key()) {
            Ok(Some(value)) => {
                println!("retrived value {}", value.to_utf8().unwrap());
                resp.set_value(value.to_vec());
            },
            Ok(None) => println!("value not found"),
            Err(e) => println!("operational problem encountered: {}", e),
        }
        let f = sink
            .success(resp)
            .map_err(move |e| error!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }

    fn put(&mut self, ctx: RpcContext, req: PutRequest, sink: UnarySink<PutResponse>) {
        let db = DB::open_default("/tmp/rocks").unwrap();
        let mut resp = PutResponse::new();
        match db.put(req.get_key(), req.get_value()) {
            Ok(()) => println!("retrieved!"),
            Err(e) => {
                let mut error = Exception::new();
                error.set_field_type(Exception_Type::RetryableException);
                error.set_message(e);
                resp.set_exception(error);
            },
        }
        let f = sink
            .success(resp)
            .map_err(move |e| error!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }

    fn delete(&mut self, ctx: RpcContext, req: DeleteRequest, sink: UnarySink<DeleteResponse>) {
    }

    fn scan(&mut self, ctx: RpcContext, req: ScanRequest, sink: UnarySink<ScanResponse>) {
    }
}

fn main() {
    let _guard = log_util::init_log(None);
    let env = Arc::new(Environment::new(1));

    let service = operation_grpc::create_sparrow_db(SparrowDBService);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 50_051)
        .build()
        .unwrap();
    server.start();

    for &(ref host, port) in server.bind_addrs() {
        info!("listening on {}:{}", host, port);
    }

    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        info!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = rx.wait();
    let _ = server.shutdown().wait();
}
