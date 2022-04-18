use crate::types::Blob;

use crate::file_handler::FileBlobHandler;
use axum::{routing::post, Router};
use axum_msgpack::MsgPack;

mod blob_traits;
mod file_handler;
mod types;

/// Post a blob to the server to save
async fn post_blob(MsgPack(blob): MsgPack<Blob>) {
    dbg!(blob.metadata);
}

/// Creates the router and registers the routes
fn app() -> Router {
    let handler = FileBlobHandler::default();
    Router::new()
        .route("/blob", post(post_blob))
        .layer(axum::extract::Extension(handler))
}

#[tokio::main]
async fn main() {
    // build our application with a single route

    // run it with hyper on localhost:3030
    axum::Server::bind(&"0.0.0.0:3030".parse().unwrap())
        .serve(app().into_make_service())
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::MetadataBuilder;
    use axum::http;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_post_blob() {
        // Create a blob function
        let blob = Blob {
            metadata: MetadataBuilder::new("test", "txt").build(),
            data: vec![1, 2, 3],
        };
        // Create the app
        let app = app();

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
    }
}
