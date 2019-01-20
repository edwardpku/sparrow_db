
use sparrow_db_model::proto::operation::{GetRequest, GetResponse, PutRequest, PutResponse};

pub trait storage_engine {
    fn get(&self, req: &GetRequest) -> Result<GetResponse, String>;
    fn put(&self, req: &PutRequest) -> Result<PutResponse, String>;
}
