use std::{
	collections::HashMap,
	sync::{Arc, RwLock},
};
use serde::{Deserialize, Serialize};
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
		let mut store = self.store.write().unwrap();
		let id = store.len() as i32 + 1;
		let todo = Todo::new(id, payload.text);
		store.insert(id, todo.clone());
		todo
	}

	fn find(&self, id: i32) -> Option<Todo> {
		let store = self.store.read().unwrap();
		store.get(&id).cloned()
	}

	fn all(&self) -> Vec<Todo> {
		let store = self.store.read().unwrap();
		store.values().cloned().collect()
	}

	fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo> {
		let mut store = self.store.write().unwrap();
		let todo = store.get_mut(&id).ok_or(RepositoryError::UserNotFound(id))?;
		if let Some(text) = payload.text {
			todo.text = text;
		}
		if let Some(completed) = payload.completed {
			todo.completed = completed;
		}
		Ok(todo.clone())
	}

	fn delete(&self, id: i32) -> anyhow::Result<()> {
		let mut store = self.store.write().unwrap();
		store.remove(&id).ok_or(RepositoryError::UserNotFound(id))?;
		Ok(())
	}
}