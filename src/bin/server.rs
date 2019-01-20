extern crate futures;
extern crate grpcio;
extern crate rocksdb;
extern crate sparrow_db_model;

#[macro_use]
extern crate log;

#[path = "../storage/mod.rs"]
mod storage;

use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::sync::oneshot;
use futures::Future;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

use sparrow_db_model::proto::operation::{
    DeleteRequest, DeleteResponse, Exception, Exception_Type, GetRequest, GetResponse, PutRequest,
    PutResponse, ScanRequest, ScanResponse,
};
use sparrow_db_model::proto::operation_grpc::{self, SparrowDb};
use storage::rocksdb_engine::{rocksdb_engine, rocksdb_engine_config};
use storage::storage_engine::storage_engine;

#[derive(Clone)]
struct SparrowDBService {
    storage: rocksdb_engine,
}

impl SparrowDb for SparrowDBService {
    fn get(&mut self, ctx: RpcContext, req: GetRequest, sink: UnarySink<GetResponse>) {
        let resp = self.storage.get(&req);
        let f = sink
            .success(resp.unwrap())
            .map_err(move |e| error!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }

    fn put(&mut self, ctx: RpcContext, req: PutRequest, sink: UnarySink<PutResponse>) {
        let resp = self.storage.put(&req);
        let f = sink
            .success(resp.unwrap())
            .map_err(move |e| error!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }

    fn delete(&mut self, ctx: RpcContext, req: DeleteRequest, sink: UnarySink<DeleteResponse>) {}

    fn scan(&mut self, ctx: RpcContext, req: ScanRequest, sink: UnarySink<ScanResponse>) {}
}

fn main() {
    let env = Arc::new(Environment::new(1));

    let config: rocksdb_engine_config = rocksdb_engine_config {
        path: String::from("/tmp/rocks"),
    };
    let engine = rocksdb_engine::new(&config);
    let service = operation_grpc::create_sparrow_db(SparrowDBService { storage: engine });
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
