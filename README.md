# Rust MiniRedis

A lightweight, blazingly-fast in-memory key-value store inspired by Redis, built with Rust and Tokio.

## Features

- In-memory key-value storage
- Automatic key expiration (TTL)
- Support for string and list (comma-separated) values
- Async TCP server via Tokio
- Commands: `SET`, `GET`, `DEL`

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)

## Running the Server

```bash
cargo run
```

The server starts on `127.0.0.1:6379`.

## Querying the Server with NetCat

### Mac

```bash
nc 127.0.0.1 6379
```

### Linux

```bash
nc -q 0 127.0.0.1 6379
```

Once connected, type commands directly:

```
SET name Alice
GET name
DEL name
```

### One-liner commands (Mac & Linux)

```bash
# SET a key
echo "SET name Alice" | nc 127.0.0.1 6379

# SET a key with a custom TTL (in seconds)
echo "SET name Alice 120" | nc 127.0.0.1 6379

# GET a key
echo "GET name" | nc 127.0.0.1 6379

# SET a list value (comma-separated)
echo "SET colors red,green,blue" | nc 127.0.0.1 6379

# DELETE a key
echo "DEL name" | nc 127.0.0.1 6379
```

## Commands

| Command | Syntax                    | Description                                                     |
| ------- | ------------------------- | --------------------------------------------------------------- |
| `SET`   | `SET <key> <value> [ttl]` | Store a value. Optional TTL in seconds (default: 60).           |
| `GET`   | `GET <key>`               | Retrieve a value by key. Returns `Nil` if not found or expired. |
| `DEL`   | `DEL <key>`               | Delete a key.                                                   |

### List Values

Pass a comma-separated string as the value to store a list:

```
SET fruits apple,banana,mango
GET fruits
# => apple,banana,mango, type: VecStr
```

## Project Structure

```
src/
├── main.rs      # Entry point
├── runner.rs    # TCP server, client handler, command parsing
└── state.rs     # In-memory store with TTL logic
```

## Dependencies

- [tokio](https://tokio.rs/) — Async runtime
