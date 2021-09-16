use tracing::instrument;

#[tokio::main]
#[instrument]
async fn main() {
    einrain_core::start().await
}