# Rust MiniRedis

A lightweight, blazingly-fast in-memory key-value store inspired by Redis, built with Rust and Tokio.

## Features

- In-memory key-value storage
- Automatic key expiration (TTL)
- Support for string, list (comma-separated), and hash (key:value pairs) values
- Async TCP server via Tokio
- Commands: `SET`, `GET`, `UPDATE`, `DEL`, `EXISTS`, `RENAME`, `TYPE`, `CLEARALL`, `PING`

## Installation

Install directly from [crates.io](https://crates.io/crates/rs-miniredis):

```bash
cargo install rs-miniredis
```

## Running the Server

```bash
rs-miniredis
```

The server starts on `127.0.0.1:6379` by default.

To specify a custom host and port:

```bash
rs-miniredis --host 0.0.0.0 --port 6379
```

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)

## Contributing

Clone the repository

```bash
git clone https://github.com/Aliemeka/rs-miniredis.git
```

## Running the Server after cloning

```bash
cargo run
```

You can also pass host and port flags:

```bash
cargo run -- --host 0.0.0.0 --port 6379
```

The server starts on `127.0.0.1:6379` by default.

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
UPDATE name Bob
EXISTS name
TYPE name
RENAME name username
DEL username
CLEARALL
PING
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

# SET a hash value (comma-separated key:value pairs)
echo "SET user name:Alice,age:30,city:Lagos" | nc 127.0.0.1 6379

# UPDATE a key's value (can change type)
echo "UPDATE name Bob" | nc 127.0.0.1 6379

# UPDATE a string to a list
echo "UPDATE name red,green,blue" | nc 127.0.0.1 6379

# UPDATE a key with a new TTL
echo "UPDATE name Alice 300" | nc 127.0.0.1 6379

# Check if a key exists
echo "EXISTS name" | nc 127.0.0.1 6379

# Get the type of a stored value
echo "TYPE name" | nc 127.0.0.1 6379

# Rename a key
echo "RENAME name username" | nc 127.0.0.1 6379

# Clear all keys
echo "CLEARALL" | nc 127.0.0.1 6379

# Ping the server
echo "PING" | nc 127.0.0.1 6379

# DELETE a key
echo "DEL name" | nc 127.0.0.1 6379
```

## Commands

| Command           | Syntax                        | Description                                                             |
| ----------------- | ----------------------------- | ----------------------------------------------------------------------- |
| `SET`             | `SET <key> <value> [ttl]`     | Store a value. Optional TTL in seconds (default: 60).                   |
| `GET`             | `GET <key>`                   | Retrieve a value by key. Returns `Nil` if not found or expired.         |
| `UPDATE`          | `UPDATE <key> <value> [ttl]`  | Update an existing key. The new value can be a different type entirely. |
| `DEL` or `DELETE` | `DEL <key>` or `DELETE <key>` | Delete a key.                                                           |
| `EXISTS`          | `EXISTS <key>`                | Returns `YES` if the key exists and has not expired, otherwise `NO`.    |
| `RENAME`          | `RENAME <old_key> <new_key>`  | Rename a key. Returns an error if the key does not exist.               |
| `TYPE`            | `TYPE <key>`                  | Returns the value type: `String`, `VecStr`, or `Hash`.                  |
| `CLEARALL`        | `CLEARALL`                    | Delete all keys from the store.                                         |
| `PING`            | `PING`                        | Health check. Returns `PONG`.                                           |

### List Values

Pass a comma-separated string as the value to store a list:

```
SET fruits apple,banana,mango
GET fruits
# => apple,banana,mango, type: VecStr
```

### Hash Values

Pass comma-separated `key:value` pairs to store a hash map:

```
SET user name:Alice,age:30,city:Lagos
GET user
# => name:Alice,age:30,city:Lagos, type: Hash
```

If any pair contains a `:`, the entire value is treated as a hash. Otherwise it falls back to a list.

### Updating Values

`UPDATE` works like `SET` but only succeeds if the key already exists. Crucially, the new value can be a completely different type from the old one:

```
SET score 42
# score is a String

UPDATE score math:90,english:85,science:78
# score is now a Hash

UPDATE score 90,85,78
# score is now a VecStr
```

## Add to your project

```bash
cargo add rs-miniredis
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
- [clap](https://docs.rs/clap) — CLI
