extern crate rocksdb;

use rocksdb::DB;
use rocksdb::rocksdb_options::WriteOptions;
use sparrow_db_model::proto::operation::{
    DeleteRequest, DeleteResponse, Exception, Exception_Type, GetRequest, GetResponse, PutRequest,
    PutResponse, ScanRequest, ScanResponse,
};
use std::sync::{Arc, Mutex};

use super::storage_engine::storage_engine;

#[derive(Clone)]
pub struct rocksdb_engine {
    pub db: Arc<DB>,
}

pub struct rocksdb_engine_config {
    pub path: String,
}

impl rocksdb_engine {
    pub fn new(config: &rocksdb_engine_config) -> Self {
        Self {
            db: Arc::new(DB::open_default(&config.path).unwrap()),
        }
    }
}

impl storage_engine for rocksdb_engine {
    fn get(&self, req: &GetRequest) -> Result<GetResponse, String> {
        let mut resp = GetResponse::new();
        match self.db.get(req.get_key()) {
            Ok(Some(value)) => {
                println!("retrived value {}", value.to_utf8().unwrap());
                resp.set_value(value.to_vec());
                Ok(resp)
            },
            Ok(None) => Ok(resp),
            Err(e) => Err(e.to_string()),
        }
    }

    fn put(&self, req: &PutRequest) -> Result<PutResponse, String> {
        let resp = PutResponse::new();
        match self.db.put_opt(req.get_key(), req.get_value(), &WriteOptions::new()) {
            Ok(()) => Ok(resp),
            Err(e) => Err(e.to_string()),
        }
    }
}
