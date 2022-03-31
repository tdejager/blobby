use async_trait::async_trait;

use crate::types::Blob;

#[async_trait]
pub trait SaveBlob {
    async fn save_blob(&self, blob: Blob) -> anyhow::Result<()>;
}
