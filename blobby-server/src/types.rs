use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag(String);

/// Struct that contains the metadata for the file sent
#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub file_name: String,
    pub extension: String,
    pub tags: Vec<Tag>,
    #[serde(with = "ts_milliseconds")]
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Blob {
    pub metadata: Metadata,
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}

/// Struct to build metadata
pub struct MetadataBuilder {
    file_name: String,
    extension: String,
    tags: Option<Vec<Tag>>,
    timestamp: Option<DateTime<Utc>>,
}

impl MetadataBuilder {
    /// Create a new builder, filename and extension are required
    pub fn new<S: Into<String>>(file_name: S, extension: S) -> Self {
        Self {
            file_name: file_name.into(),
            extension: extension.into(),
            tags: None,
            timestamp: None,
        }
    }

    /// Set specific tags
    pub fn set_tags(&mut self, tags: Vec<Tag>) {
        self.tags = Some(tags);
    }

    /// Set a timestamp
    pub fn set_timestamp(&mut self, timestamp: DateTime<Utc>) {
        self.timestamp = Some(timestamp);
    }

    /// Build the metadata
    pub fn build(self) -> Metadata {
        Metadata {
            file_name: self.file_name,
            extension: self.extension,
            tags: self.tags.unwrap_or_default(),
            timestamp: self.timestamp.unwrap_or_else(|| Utc::now()),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_deserialize_metadata() {
        let data = include_bytes!("../test-resources/metadata.msgpack");
        let metadata: super::Metadata = rmp_serde::from_slice(data).unwrap();
        dbg!(metadata);
    }

    #[test]
    fn test_deserialize_blob() {
        let data = include_bytes!("../test-resources/blob.msgpack");
        let blob: super::Blob = rmp_serde::from_slice(data).unwrap();
        dbg!(blob);
    }
}
