use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    VecStr(Vec<String>),
    Hash(HashMap<String, String>),
    // Future types can be added here (e.g., List, Set, etc.)
}

type Db = Arc<RwLock<HashMap<String, (Value, Instant)>>>; // In-memory database with expiration time

// State struct to hold the in-memory database
pub struct State {
    pub db: Db,
}

impl State {
    pub fn new() -> Self {
        let db: Db = Arc::new(RwLock::new(HashMap::new()));
        State { db }
    }

    pub fn set(&self, key: String, value: Value, expire_time: u64) {
        let mut db = self.db.write().unwrap();
        db.insert(
            key,
            (value, Instant::now() + Duration::from_secs(expire_time)),
        );
    }

    pub async fn get(&self, key: &str) -> Option<Value> {
        let db = self.db.read().unwrap();
        if let Some((value, expire_time)) = db.get(key) {
            if Instant::now() < *expire_time {
                let value = match value {
                    Value::String(s) => Some(Value::String(s.clone())),
                    Value::VecStr(v) => Some(Value::VecStr(v.clone())),
                    Value::Hash(h) => Some(Value::Hash(h.clone())),
                };
                return value;
            }
        }
        None
    }

    pub async fn delete(&self, key: &str) {
        let mut db = self.db.write().unwrap();
        db.remove(key);
    }
}

pub type SharedState = Arc<State>;

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
