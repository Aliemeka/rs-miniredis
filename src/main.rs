use clap::Parser;
use rs_miniredis::{KeyStore, run_server};

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
    let cli = Cli::parse();
    let addr = format!("{}:{}", cli.host, cli.port);
    let state = KeyStore::new();
    if let Err(e) = run_server(&addr, state).await {
        eprintln!("Server error: {}", e);
        std::process::exit(1);
    }
}
