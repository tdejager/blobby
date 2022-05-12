
use crate::types::Blob;

use crate::file_handler::FileBlobHandler;

use axum::{extract::Extension, http::StatusCode, routing::post, Router};
use axum_msgpack::MsgPack;
use blob_traits::SaveBlob;
use std::sync::Arc;

mod blob_traits;
mod file_handler;
mod types;
mod hashmap_handler;

/// Handles the anyhow error type
fn handle_anyhow_error(err: anyhow::Error) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Something went wrong: {}", err),
    )
}

/// Post a blob to the server to save
async fn post_blob(
    MsgPack(blob): MsgPack<Blob>,
    Extension(blob_handler): Extension<Arc<dyn SaveBlob + Send + Sync>>,
) -> Result<String, (StatusCode, String)> {
    blob_handler
        .save_blob(blob)
        .await
        .map(|uuid| uuid.to_string())
        .map_err(|e| handle_anyhow_error(e))
}


/// Creates the router and registers the routes
fn app(blob_handler: Arc<dyn SaveBlob + Send + Sync>) -> Router {
    Router::new()
        .route("/blob", post(post_blob))
        .layer(Extension(blob_handler))
}

#[tokio::main]
async fn main() {
    // build our application with a single route

    // run it with hyper on localhost:3030
    axum::Server::bind(&"0.0.0.0:3030".parse().unwrap())
        .serve(app(Arc::new(FileBlobHandler::default())).into_make_service())
        .await
        .expect("Blobby crashed with an error");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::MetadataBuilder;
    use axum::http;
    use std::str::FromStr;
    use tower::ServiceExt;
    use crate::hashmap_handler::HashmapHandler;

    #[tokio::test]
    async fn test_post_blob() {
        // Create a blob function
        let blob = Blob {
            metadata: MetadataBuilder::new("test", "txt").build(),
            data: vec![1, 2, 3],
        };
        // Create the app
        let app = app(Arc::new(HashmapHandler::default()));

        // Serialize data into a byte array
        let vec = rmp_serde::to_vec(&blob).unwrap();

        // Create request and sent it to the server
        let response = app
            .oneshot(
                http::Request::builder()
                    .method(axum::http::Method::POST)
                    .uri("/blob")
                    .header(
                        http::header::CONTENT_TYPE,
                        http::HeaderValue::from_static("application/msgpack"),
                    )
                    .body(axum::body::Body::from(vec))
                    .unwrap(),
            )
            .await
            .unwrap();

        // Check that we get a 200 response
        assert_eq!(response.status(), http::StatusCode::OK);
        // Check that we got a uuid back
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let uuid = String::from_utf8_lossy(body.as_ref());
        // Check if we can parse it as a uuid
        uuid::Uuid::from_str(uuid.as_ref()).expect("Should be able to parse this as a uuid");
    }
}
