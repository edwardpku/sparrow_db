// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

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

const METHOD_SPARROW_DB_GET: ::grpcio::Method<super::operation::GetRequest, super::operation::GetResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/sparrow_db_model.SparrowDB/Get",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SPARROW_DB_PUT: ::grpcio::Method<super::operation::PutRequest, super::operation::PutResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/sparrow_db_model.SparrowDB/Put",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SPARROW_DB_DELETE: ::grpcio::Method<super::operation::DeleteRequest, super::operation::DeleteResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/sparrow_db_model.SparrowDB/Delete",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SPARROW_DB_SCAN: ::grpcio::Method<super::operation::ScanRequest, super::operation::ScanResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/sparrow_db_model.SparrowDB/Scan",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct SparrowDbClient {
    client: ::grpcio::Client,
}

impl SparrowDbClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        SparrowDbClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn get_opt(&self, req: &super::operation::GetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operation::GetResponse> {
        self.client.unary_call(&METHOD_SPARROW_DB_GET, req, opt)
    }

    pub fn get(&self, req: &super::operation::GetRequest) -> ::grpcio::Result<super::operation::GetResponse> {
        self.get_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_async_opt(&self, req: &super::operation::GetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operation::GetResponse>> {
        self.client.unary_call_async(&METHOD_SPARROW_DB_GET, req, opt)
    }

    pub fn get_async(&self, req: &super::operation::GetRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operation::GetResponse>> {
        self.get_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn put_opt(&self, req: &super::operation::PutRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operation::PutResponse> {
        self.client.unary_call(&METHOD_SPARROW_DB_PUT, req, opt)
    }

    pub fn put(&self, req: &super::operation::PutRequest) -> ::grpcio::Result<super::operation::PutResponse> {
        self.put_opt(req, ::grpcio::CallOption::default())
    }

    pub fn put_async_opt(&self, req: &super::operation::PutRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operation::PutResponse>> {
        self.client.unary_call_async(&METHOD_SPARROW_DB_PUT, req, opt)
    }

    pub fn put_async(&self, req: &super::operation::PutRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operation::PutResponse>> {
        self.put_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_opt(&self, req: &super::operation::DeleteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operation::DeleteResponse> {
        self.client.unary_call(&METHOD_SPARROW_DB_DELETE, req, opt)
    }

    pub fn delete(&self, req: &super::operation::DeleteRequest) -> ::grpcio::Result<super::operation::DeleteResponse> {
        self.delete_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_async_opt(&self, req: &super::operation::DeleteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operation::DeleteResponse>> {
        self.client.unary_call_async(&METHOD_SPARROW_DB_DELETE, req, opt)
    }

    pub fn delete_async(&self, req: &super::operation::DeleteRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operation::DeleteResponse>> {
        self.delete_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn scan_opt(&self, req: &super::operation::ScanRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operation::ScanResponse> {
        self.client.unary_call(&METHOD_SPARROW_DB_SCAN, req, opt)
    }

    pub fn scan(&self, req: &super::operation::ScanRequest) -> ::grpcio::Result<super::operation::ScanResponse> {
        self.scan_opt(req, ::grpcio::CallOption::default())
    }

    pub fn scan_async_opt(&self, req: &super::operation::ScanRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operation::ScanResponse>> {
        self.client.unary_call_async(&METHOD_SPARROW_DB_SCAN, req, opt)
    }

    pub fn scan_async(&self, req: &super::operation::ScanRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operation::ScanResponse>> {
        self.scan_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait SparrowDb {
    fn get(&mut self, ctx: ::grpcio::RpcContext, req: super::operation::GetRequest, sink: ::grpcio::UnarySink<super::operation::GetResponse>);
    fn put(&mut self, ctx: ::grpcio::RpcContext, req: super::operation::PutRequest, sink: ::grpcio::UnarySink<super::operation::PutResponse>);
    fn delete(&mut self, ctx: ::grpcio::RpcContext, req: super::operation::DeleteRequest, sink: ::grpcio::UnarySink<super::operation::DeleteResponse>);
    fn scan(&mut self, ctx: ::grpcio::RpcContext, req: super::operation::ScanRequest, sink: ::grpcio::UnarySink<super::operation::ScanResponse>);
}

pub fn create_sparrow_db<S: SparrowDb + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SPARROW_DB_GET, move |ctx, req, resp| {
        instance.get(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SPARROW_DB_PUT, move |ctx, req, resp| {
        instance.put(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SPARROW_DB_DELETE, move |ctx, req, resp| {
        instance.delete(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SPARROW_DB_SCAN, move |ctx, req, resp| {
        instance.scan(ctx, req, resp)
    });
    builder.build()
}
