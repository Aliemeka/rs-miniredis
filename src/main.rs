mod runner;
mod state;

#[tokio::main]
async fn main() {
    runner::run_app().await;
}
