#[macro_use]
extern crate log;
use pretty_env_logger::env_logger;

pub mod cli;
use cli::Cli;

#[tokio::main]
async fn main() {
    env_logger::init();
    Cli::default().match_input().await;
}
