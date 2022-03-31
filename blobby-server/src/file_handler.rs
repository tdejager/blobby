use crate::blob_traits::SaveBlob;
use crate::types::Metadata;
use crate::Blob;
use anyhow::Result;
use async_trait::async_trait;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

/// Settings for the Blob handler
#[derive(Clone)]
pub struct FileSettings {
    pub data_folder: String,
    pub metadata_folder: String,
}

impl FileSettings {
    /// Resolve the path for the blob file
    pub fn resolve_blob_path(&self, metadata: &Metadata) -> String {
        format!(
            "{}/{}.{}",
            self.data_folder, metadata.file_name, metadata.extension
        )
    }

    /// Function resolve the metadata path
    pub fn resolve_metadata_path(&self, metadata: &Metadata) -> String {
        format!("{}/{}.json", self.metadata_folder, metadata.file_name)
    }
}

/// Default settings for the Blob handler
impl Default for FileSettings {
    fn default() -> Self {
        Self {
            data_folder: "/tmp".to_string(),
            metadata_folder: "/tmp".to_string(),
        }
    }
}

/// Struct that handles saving of blobs to disk
#[derive(Clone)]
pub struct FileBlobHandler {
    settings: FileSettings,
}

impl FileBlobHandler {
    /// Create a new BlobHandler
    pub fn new(settings: FileSettings) -> Self {
        Self { settings }
    }
}

#[async_trait]
impl SaveBlob for FileBlobHandler {
    /// Save blob to filesystem
    async fn save_blob(&self, blob: Blob) -> Result<()> {
        // Write the blob
        let mut blob_file = File::create(self.settings.resolve_blob_path(&blob.metadata)).await?;
        blob_file.write_all(&blob.data).await?;

        // Write the metadata
        let json = serde_json::to_value(&blob.metadata).unwrap();
        let mut metadata_file =
            File::create(self.settings.resolve_metadata_path(&blob.metadata)).await?;
        // Save the json file
        metadata_file.write_all(json.to_string().as_bytes()).await?;
        Ok(())
    }
}

impl Default for FileBlobHandler {
    fn default() -> Self {
        Self::new(FileSettings::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::blob_traits::SaveBlob;
    use crate::file_handler::{FileBlobHandler, FileSettings};
    use crate::types::{Blob, MetadataBuilder};

    #[tokio::test]
    pub async fn test_save() {
        // Setup
        let builder = MetadataBuilder::new("bloep", "txt");
        let file_settings = FileSettings::default();
        let blob_handler = FileBlobHandler::new(FileSettings::default());
        let blob = Blob {
            metadata: builder.build(),
            data: vec![],
        };

        // Save file
        blob_handler.save_blob(blob).await.unwrap();

        // Check if files have been created
        assert!(
            std::path::Path::new(&format!("{}/{}", file_settings.data_folder, "bloep.txt"))
                .exists()
        );
        // and if metadata exists
        assert!(std::path::Path::new(&format!(
            "{}/{}",
            file_settings.metadata_folder, "bloep.json"
        ))
        .exists());
    }
}
