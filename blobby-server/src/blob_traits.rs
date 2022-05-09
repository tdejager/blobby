use async_trait::async_trait;

use crate::types::Blob;

#[async_trait]
pub trait SaveBlob {
    // Save the blob to some datasource
    async fn save_blob(&self, blob: Blob) -> anyhow::Result<uuid::Uuid>;
}

#[async_trait]
pub trait QueryBlob {
    /// Use this method to query a blob by name
    async fn query_blob_by_name<S: AsRef<str>>(&self, file_name: S) -> anyhow::Result<Vec<Blob>>;
    /// Use this method to query a blob by its tags
    async fn query_blob_by_tags<S: AsRef<str>>(&self, name: S) -> anyhow::Result<Vec<Blob>>;
}
