pub mod pb;

mod abi;
mod config;

pub use config::AppConfig;

pub use abi::Tpl;
use futures::Stream;
use pb::{
    meta_data_server::{MetaData, MetaDataServer},
    Content, MaterializeRequest,
};
use std::pin::Pin;
use tonic::{async_trait, Request, Response, Status, Streaming};

#[allow(unused)]
pub struct MetadataService {
    config: AppConfig,
}

type ServiceResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<Content, Status>> + Send>>;

#[async_trait]
impl MetaData for MetadataService {
    type MaterializeStream = ResponseStream;

    async fn materialize(
        &self,
        request: Request<Streaming<MaterializeRequest>>,
    ) -> ServiceResult<Self::MaterializeStream> {
        let query = request.into_inner();
        self.materialize(query).await
    }
}

impl MetadataService {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }

    pub fn into_server(self) -> MetaDataServer<Self> {
        MetaDataServer::new(self)
    }
}
