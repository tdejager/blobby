use async_trait::async_trait;

use crate::types::Blob;

#[async_trait]
pub trait SaveBlob {
    async fn save_blob(&self, blob: Blob) -> anyhow::Result<(uuid::Uuid)>;
}

#[async_trait]
pub trait QueryBlob {
    async fn query_blob_by_name<S: AsRef<str>>(&self, file_name: S) -> anyhow::Result<Vec<Blob>>;
    async fn query_blob_by_tags<S: AsRef<str>>(&self, name: S) -> anyhow::Result<Vec<Blob>>;
}
