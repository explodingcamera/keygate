mod prisma;
pub use prisma::*;
use prisma_client_rust::NewClientError;

pub use PrismaClient;

pub async fn connect(url: &str) -> Result<PrismaClient, NewClientError> {
    PrismaClient::_builder().with_url(url.into()).build().await
}
