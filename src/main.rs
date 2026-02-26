use clap::Parser;
use rs_miniredis::{KeyState, run_server};

#[derive(Parser)]
#[command(
    name = "rs-miniredis",
    about = "A lightweight in-memory key-value store"
)]
struct Cli {
    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    #[arg(short, long, default_value_t = 6379)]
    port: u16,
}

#[tokio::main]
async fn main() {
    let state = KeyState::new();
    if let Err(e) = run_server("127.0.0.1:6379", state).await {
        eprintln!("Server error: {}", e);
        std::process::exit(1);
    }
}
