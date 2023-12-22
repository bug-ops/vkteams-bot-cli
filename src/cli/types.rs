use clap::{Parser, Subcommand};
/// VKTeams CLI - Interacts with VK Teams API
#[derive(Parser, Clone, Debug)]
#[command(author="Andrei G.", version="0.1.0", about="vkteams-bot-cli tool", long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub subcmd: SubCommand,
}
/// Subcommands for VKTeams CLI
#[derive(Subcommand, Debug, Clone)]
pub enum SubCommand {
    /// ## Example
    /// ```bash
    /// vkteams-bot-cli send-text --user-id text@example --message "Hello"
    /// ```
    SendText {
        #[arg(short, long, required = true, value_name = "USER_ID")]
        user_id: String,
        #[arg(short, long, required = true, value_name = "MESSAGE")]
        message: String,
    },
    /// ## Example
    /// ```bash
    /// vkteams-bot-cli send-file --user-id text@example.com --file-path ./file.txt
    /// ```
    SendFile {
        #[arg(short, long, required = true, value_name = "USER_ID")]
        user_id: String,
        #[arg(short, long, required = true, value_name = "FILE_PATH")]
        file_path: String,
    },
    GetEvents,
}
