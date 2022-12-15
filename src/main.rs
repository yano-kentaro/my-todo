//! # main.rs

// ===========================================================|0
//                        ライブラリのインポート
// ==================================================|2022/12/15
mod repositories;
mod handlers;

use crate::repositories::{TodoRepository, TodoRepositoryForMemory};
use handlers::{create_todo};

use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use std::{
    env,
    net::SocketAddr,
    sync::Arc,
};

#[tokio::main]
async fn main() {
    // Logging
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    // Server
    let repository = TodoRepositoryForMemory::new();
    let app = create_app(repository);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

fn create_app<T: TodoRepository>(repository: T) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/todos", post(create_todo::<T>))
        .layer(Extension(Arc::new(repository)))
}

// // テストの定義
// #[cfg(test)]
// mod test {
//     use super::*;
//     use axum::{
//         body::Body,
//         http::{header, Method, Request},
//     };
//     use tower::ServiceExt;

//     #[tokio::test]
//     async fn should_return_hello_world() {
//         let req = Request::builder()
//             .method(Method::GET)
//             .uri("/")
//             .body(Body::empty())
//             .unwrap();

//         let app = create_app();
//         let res = app.oneshot(req).await.unwrap();

//         let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
//         let body = String::from_utf8(bytes.to_vec()).unwrap();

//         assert_eq!(body, "Hello, World!");
//     }

//     #[tokio::test]
//     async fn should_return_user_data() {
//         let req = Request::builder()
//             .method(Method::POST)
//             .uri("/users")
//             .header(header::CONTENT_TYPE, "application/json")
//             .body(Body::from(r#"{"username": "test"}"#))
//             .unwrap();

//         let app = create_app();
//         let res = app.oneshot(req).await.unwrap();

//         let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
//         let body = String::from_utf8(bytes.to_vec()).unwrap();

//         assert_eq!(body, r#"{"id":1337,"username":"test"}"#);
//     }
// }
