use vkteams_bot_cli::cli::Cli;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Unable to load .env file");
    Cli::default().match_input().await;
}
