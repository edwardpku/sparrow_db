
use sparrow_db_model::proto::operation::{
    DeleteRequest, DeleteResponse, Exception, Exception_Type, GetRequest, GetResponse, PutRequest,
    PutResponse, ScanRequest, ScanResponse,
};

pub trait storage_engine {
    fn get(&self, req: &GetRequest) -> Result<GetResponse, String>;
    fn put(&self, req: &PutRequest) -> Result<PutResponse, String>;
}
