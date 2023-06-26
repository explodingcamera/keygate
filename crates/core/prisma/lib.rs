mod prisma;
pub use prisma::*;
pub use prisma_client_rust as client;

pub async fn connect(url: &str) -> Result<PrismaClient, client::NewClientError> {
    PrismaClient::_builder().with_url(url.into()).build().await
}
