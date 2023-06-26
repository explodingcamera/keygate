mod prisma;
use std::future::Future;

use client::QueryError;
pub use prisma::*;
pub use prisma_client_rust as client;

pub async fn connect(url: &str) -> Result<PrismaClient, client::NewClientError> {
    PrismaClient::_builder().with_url(url.into()).build().await
}

impl PrismaClient {
    pub async fn tx<TErr, TRet, TFut, TFn>(&self, tx: TFn) -> Result<TRet, TErr>
    where
        TFut: Future<Output = Result<TRet, TErr>>,
        TFn: FnOnce(PrismaClient) -> TFut,
        TErr: From<QueryError>,
    {
        self._transaction().run(tx).await
    }
}
