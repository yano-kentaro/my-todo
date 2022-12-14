//! # main.rs

// ===========================================================|0
//                        ライブラリのインポート
// ==================================================|2022/12/15
use anyhow::Context;
use axum::{
    extract::Extension,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env,
    net::SocketAddr,
    sync::{Arc, RwLock},
};
use thiserror::Error;

// ===========================================================|0
//                        リポジトリエラーの定義
// ==================================================|2022/12/15
#[derive(Debug, Error)]
enum RepositoryError {
    #[error("NotFound, id is {0}")]
    UserNotFound(i32),
}

pub trait TodoRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    fn create(&self, payload: CreateTodo) -> Todo;
    fn find(&self, id: i32) -> Option<Todo>;
    fn all(&self) -> Vec<Todo>;
    fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo>;
    fn delete(&self, id: i32) -> anyhow::Result<()>;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Todo {
    id: i32,
    text: String,
    completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateTodo {
    text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateTodo {
    text: Option<String>,
    completed: Option<bool>,
}

impl Todo {
    pub fn new(id: i32, text: String) -> Self {
        Self {
            id,
            text,
            completed: false,
        }
    }
}

type TodoDatas = HashMap<i32, Todo>;

#[derive(Debug, Clone)]
pub struct TodoRepositoryForMemory {
    store: Arc<RwLock<TodoDatas>>,
}

impl TodoRepositoryForMemory {
    pub fn new() -> Self {
        TodoRepositoryForMemory {
            store: Arc::default(),
        }
    }
}

impl TodoRepository for TodoRepositoryForMemory {
    fn create(&self, payload: CreateTodo) -> Todo {
        todo!();
    }

    fn find(&self, id: i32) -> Option<Todo> {
        todo!();
    }

    fn all(&self) -> Vec<Todo> {
        todo!();
    }

    fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo> {
        todo!();
    }

    fn delete(&self, id: i32) -> anyhow::Result<()> {
        todo!();
    }
}

#[tokio::main]
async fn main() {
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

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

pub async fn create_todo<T: TodoRepository>(
    Json(payload): Json<CreateTodo>,
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse {
    let todo = repository.create(payload);
    (StatusCode::CREATED, Json(todo))
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
