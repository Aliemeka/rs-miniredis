use crate::state::{SharedState, State, Value};
use std::time::{Duration, Instant};
use tokio::io::split as tokio_split;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::sleep as tokio_sleep;

const DEFAULT_EXPIRTY: u64 = 60; // default expiration time in seconds

pub async fn run_app() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Mini-Redis with Active TTL running on 127.0.0.1:6379");

    // initialize state
    let state = SharedState::new(State::new());

    tokio::spawn(auto_expire(state.clone()));

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let state = state.clone();

        tokio::spawn(async move {
            handle_client(stream, state).await;
        });
    }
}

async fn auto_expire(state: SharedState) {
    loop {
        {
            let mut db = state.db.write().unwrap();
            let now = Instant::now();
            db.retain(|_, (_, expire_time)| *expire_time > now); // Only retain entries that haven't expired
            drop(db);
        }
        tokio_sleep(Duration::from_secs(1)).await;
    }
}

async fn handle_client(stream: TcpStream, state: SharedState) {
    // handle client connection and commands
    let (reader, mut writer) = tokio_split(stream);
    let buff_reader = BufReader::new(reader);
    let mut lines = buff_reader.lines();
    while let Some(line) = lines.next_line().await.unwrap() {
        // use line here
        let mut parts = line.split_whitespace();
        if parts.clone().count() == 0 {
            writer.write_all(b"Error: Empty command\r\n").await.unwrap();
            continue;
        }
        let command = parts.next().unwrap_or("").to_uppercase();

        let response = match command.as_str() {
            "SET" => {
                if parts.clone().count() < 2 {
                    "Error: SET command requires at least 2 arguments\r\n".to_string();
                }
                let key = parts.next().unwrap_or("").to_string();
                let value = parts.next().unwrap_or("").to_string();
                let value_type = if value.contains(',') {
                    Value::VecStr(value.split(',').map(|s| s.to_string()).collect())
                } else {
                    Value::String(value)
                };
                let expire_time = parts
                    .next()
                    .unwrap_or("")
                    .to_string()
                    .parse()
                    .unwrap_or(DEFAULT_EXPIRTY);
                state.set(key, value_type, expire_time);
                "OK\r\n".to_string()
            }
            "GET" => {
                let key = parts.next().unwrap_or("");

                if let Some(value) = state.get(key).await {
                    let (value, value_type) = match value {
                        Value::String(s) => (s, "String"),
                        Value::VecStr(v) => (v.join(","), "VecStr"),
                    };
                    format!("{value}, type: {value_type}\r\n")
                } else {
                    "Nil\r\n".to_string()
                }
            }
            "DEL" | "DELETE" => {
                let key = parts.next().unwrap_or("");
                state.delete(key).await;
                "OK\r\n".to_string()
            }
            _ => "Unknown command\r\n".to_string(),
        };
        writer.write_all(response.as_bytes()).await.unwrap();
    }
}
