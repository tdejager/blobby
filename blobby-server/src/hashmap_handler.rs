use std::collections::HashMap;
use uuid::Uuid;
use crate::{Blob, SaveBlob};
use async_trait::async_trait;
use tokio::sync::Mutex;

pub struct HashmapHandler {
    data: Mutex<HashMap<Uuid, Blob>>
}

impl Default for HashmapHandler {
    fn default() -> Self {
        Self {
            data: Mutex::new(HashMap::default())
        }
    }
}

#[async_trait]
impl SaveBlob for HashmapHandler {
    async fn save_blob(&self, blob: Blob) -> anyhow::Result<Uuid> {
        let uuid = Uuid::new_v4();
        self.data.lock().await.insert(uuid, blob);
        Ok(uuid)
    }
}