use clap::{Parser, Subcommand};
/// VKTeams CLI - Interacts with VK Teams API
#[derive(Parser, Clone, Debug)]
#[command(author="Andrei G.", version="0.1.1", about="vkteams-bot-cli tool", long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub subcmd: SubCommand,
}
/// Subcommands for VKTeams CLI
#[derive(Subcommand, Debug, Clone)]
pub enum SubCommand {
    /// Send text message text <MESSAGE> to user with <USER_ID>
    SendText {
        #[arg(short, long, required = true, value_name = "USER_ID")]
        user_id: String,
        #[arg(short, long, required = true, value_name = "MESSAGE")]
        message: String,
    },
    /// Send file from <FILE_PATH> to user with <USER_ID>
    SendFile {
        #[arg(short, long, required = true, value_name = "USER_ID")]
        user_id: String,
        #[arg(short, long, required = true, value_name = "FILE_PATH")]
        file_path: String,
    },
    /// Get events once or listen with optional <LISTEN> flag
    GetEvents {
        #[arg(short, long, required = false, value_name = "LISTEN")]
        listen: Option<bool>,
    },
}
