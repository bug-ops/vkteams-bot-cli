use vkteams_bot_cli::cli::Cli;

#[tokio::main]
async fn main() {
    Cli::default().match_input().await;
}
