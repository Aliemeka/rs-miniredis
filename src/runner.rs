use crate::state::{SharedState, State, Value};
use std::time::{Duration, Instant};
use tokio::io::split as tokio_split;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::sleep as tokio_sleep;

const DEFAULT_EXPIRTY: u64 = 60; // default expiration time in seconds

pub async fn run_server(addr: &str, state: State) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Mini-Redis with Active TTL running on {}", addr);

    let state = SharedState::new(state);

    tokio::spawn(auto_expire(state.clone()));

    loop {
        let (stream, addr) = listener.accept().await.unwrap();
        println!("New client connected: {}", addr);
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
                    if value.contains(':') {
                        Value::Hash(
                            value
                                .split(',')
                                .filter_map(|pair| {
                                    let mut kv = pair.splitn(2, ':');
                                    if let (Some(k), Some(v)) = (kv.next(), kv.next()) {
                                        Some((k.to_string(), v.to_string()))
                                    } else {
                                        None
                                    }
                                })
                                .collect(),
                        )
                    } else {
                        Value::VecStr(value.split(',').map(|s| s.to_string()).collect())
                    }
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
                if parts.clone().count() != 1 {
                    "Error: GET command requires exactly 1 argument\r\n".to_string();
                }
                let key = parts.next().unwrap_or("");

                if let Some(value) = state.get(key).await {
                    let (value, value_type) = match value {
                        Value::String(s) => (s, "String"),
                        Value::VecStr(v) => (v.join(","), "VecStr"),
                        Value::Hash(h) => (
                            h.iter()
                                .map(|(k, v)| format!("{}:{}", k, v))
                                .collect::<Vec<String>>()
                                .join(","),
                            "Hash",
                        ),
                    };
                    format!("{value}, type: {value_type}\r\n")
                } else {
                    "Nil\r\n".to_string()
                }
            }
            "UPDATE" => {
                if parts.clone().count() < 2 {
                    "Error: UPDATE command requires at least 2 arguments\r\n".to_string();
                }
                let key = parts.next().unwrap_or("");
                if let Some(_) = state.get(key).await {
                    let new_val = parts.next().unwrap_or("").to_string();
                    let new_value = if new_val.contains(',') {
                        if new_val.contains(':') {
                            Value::Hash(
                                new_val
                                    .split(',')
                                    .filter_map(|pair| {
                                        let mut kv = pair.splitn(2, ':');
                                        if let (Some(k), Some(v)) = (kv.next(), kv.next()) {
                                            Some((k.to_string(), v.to_string()))
                                        } else {
                                            None
                                        }
                                    })
                                    .collect(),
                            )
                        } else {
                            Value::VecStr(new_val.split(',').map(|s| s.to_string()).collect())
                        }
                    } else {
                        Value::String(new_val)
                    };
                    let expire_time = parts
                        .next()
                        .unwrap_or("")
                        .to_string()
                        .parse()
                        .unwrap_or(DEFAULT_EXPIRTY);
                    state.set(key.to_string(), new_value, expire_time);
                    "OK\r\n".to_string()
                } else {
                    "Error: Key does not exist\r\n".to_string()
                }
            }
            "DEL" | "DELETE" => {
                let key = parts.next().unwrap_or("");
                state.delete(key).await;
                "OK\r\n".to_string()
            }
            "EXISTS" => {
                let key = parts.next().unwrap_or("");
                if state.get(key).await.is_some() {
                    "YES\r\n".to_string()
                } else {
                    "NO\r\n".to_string()
                }
            }
            "RENAME" => {
                let old_key = parts.next().unwrap_or("");
                let new_key = parts.next().unwrap_or("");
                if let Some(value) = state.get(old_key).await {
                    state.set(new_key.to_string(), value, DEFAULT_EXPIRTY);
                    state.delete(old_key).await;
                    "OK\r\n".to_string()
                } else {
                    "Error: Key does not exist\r\n".to_string()
                }
            }
            "TYPE" => {
                let key = parts.next().unwrap_or("");
                if let Some(value) = state.get(key).await {
                    let value_type = match value {
                        Value::String(_) => "String",
                        Value::VecStr(_) => "VecStr",
                        Value::Hash(_) => "Hash",
                    };
                    format!("{value_type}\r\n")
                } else {
                    "Nil\r\n".to_string()
                }
            }
            "CLEARALL" => {
                let mut db = state.db.write().unwrap();
                db.clear();
                "OK\r\n".to_string()
            }
            "PING" => "PONG\r\n".to_string(),
            _ => "Unknown command\r\n".to_string(),
        };
        writer.write_all(response.as_bytes()).await.unwrap();
    }
}
