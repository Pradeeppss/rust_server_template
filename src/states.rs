use sqlx::{Pool, Sqlite};

use crate::models::Task;
use std::sync::{Arc, Mutex};



#[derive(Clone)]
pub struct AppState {
    pub tasks: Arc<Mutex<Vec<Task>>>,
    pub next_id: Arc<Mutex<u32>>,
    pub pool: Pool<Sqlite>
}


impl AppState {
    pub fn new(pool:Pool<Sqlite>) -> Self {
        let seeds: Vec<Task> = vec![
            Task {id: 1, title: "learn axum".to_string(), done: false},
            Task {id: 2, title: "learn rust".to_string(), done: false},
            Task {id: 3, title: "learn linux".to_string(), done: false},
            Task {id: 4, title: "learn js".to_string(), done: true},
        ];

        Self { 
            tasks: Arc::new(Mutex::new(seeds)),
            next_id: Arc::new(Mutex::new(5)),
            pool 
        }
    }
}