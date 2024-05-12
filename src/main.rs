#[macro_use]
extern crate log;

pub mod cli;
use cli::Cli;

#[tokio::main]
async fn main() {
    Cli::default().match_input().await;
}
