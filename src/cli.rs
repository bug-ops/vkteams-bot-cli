use clap::{Parser, Subcommand};
use colored::Colorize;
use reqwest::Client;
use std::path::PathBuf;
use vkteams_bot::api::types::*;
/// VKTeams CLI - Interacts with VK Teams API
pub struct Cli {
    /// bot instance
    pub bot: Bot,
    /// matches from clap
    pub matches: Opts,
}
/// Default implementation for bot with API V1
impl Default for Cli {
    fn default() -> Self {
        Self {
            bot: Bot::default(),
            matches: Opts::parse(),
        }
    }
}
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
    /// Download file with <FILE_ID> into <FILE_PATH>
    GetFile {
        #[arg(short = 'f', long, required = true, value_name = "FILE_ID")]
        file_id: String,
        #[arg(short = 'p', long, required = true, value_name = "FILE_PATH")]
        file_path: String,
    },
    /// Get events once or listen with optional <LISTEN> flag
    GetEvents {
        #[arg(short, long, required = false, value_name = "LISTEN")]
        listen: Option<bool>,
    },
}
/// Implementation for CLI
impl Cli {
    /// Match input with subcommands
    pub async fn match_input(&self) {
        // Match subcommands
        match self.matches.subcmd.to_owned() {
            // Subcommand for send text message
            SubCommand::SendText { user_id, message } => {
                let parser = MessageTextParser::default()
                    .add(MessageTextFormat::Plain(message))
                    .to_owned();
                match self
                    .bot
                    .messages_send_text(ChatId(user_id), Some(parser), None, None, None, None)
                    .await
                {
                    Ok(r) => match_result(self.bot.clone(), r).await,
                    Err(e) => {
                        error!("ERROR messages/sendText: {}", e);
                        println!("{}", e.to_string().red());
                    }
                };
            }
            // Subcommand for send file
            SubCommand::SendFile { user_id, file_path } => {
                match self
                    .bot
                    .messages_send_file(ChatId(user_id), file_path, None, None, None, None, None)
                    .await
                {
                    Ok(r) => match_result(self.bot.clone(), r).await,
                    Err(e) => {
                        error!("ERROR messages/sendFile: {}", e);
                        println!("API messages/sendFile: {}", e.to_string().red());
                    }
                };
            }
            // Subcommand for get events
            SubCommand::GetEvents { listen } => {
                match listen {
                    Some(true) => {
                        self.bot.event_listener(match_result).await;
                    }
                    _ => match self.bot.events_get().await {
                        Ok(events) => {
                            match_result(self.bot.clone(), events).await;
                        }
                        Err(e) => {
                            error!("ERROR events/get: {}", e);
                            println!("API events/get: {}", e.to_string().red());
                        }
                    },
                };
            }
            // Subcommand for get file from id
            SubCommand::GetFile { file_id, file_path } => {
                let id = FileId(file_id);
                match self.bot.files_get_info(id).await {
                    // Download file data
                    Ok(file_info) => {
                        if !file_info.ok {
                            error!("Error: {}", file_info.description.to_owned().unwrap());
                            println!(
                                "API files/getInfo: {}",
                                file_info.description.unwrap().red()
                            );
                            return;
                        }
                        // file_info.
                        match file_info.download(Client::new()).await {
                            // Save file to the disk
                            Ok(file_data) => {
                                file_save(
                                    &file_info.file_name.to_owned().unwrap(),
                                    &file_path,
                                    file_data,
                                )
                                .await
                            }
                            Err(e) => {
                                error!("Error: {}", e);
                                println!("Error while download: {}", e.to_string().red());
                            }
                        }
                    }
                    Err(e) => {
                        error!("Error: {}", e);
                        println!("API files/getInfo: {}", e.to_string().red());
                    }
                };
            }
        }
    }
}

/// Match result and print it
pub async fn match_result<T>(_: Bot, result: T)
where
    T: serde::Serialize,
{
    println!("{}", serde_json::to_string(&result).unwrap().green());
}
/// Save file on disk
pub async fn file_save(file_name: &str, path: &str, file_data: Vec<u8>) {
    let mut file_path = PathBuf::from(path);
    file_path.push(file_name);
    match tokio::fs::write(&file_path, file_data).await {
        Ok(_) => {
            println!(
                "File saved to: `{}`",
                file_path.display().to_string().green()
            );
        }
        Err(e) => {
            error!("Error: {}", e);
            println!("File not saved: {}", e.to_string().red());
        }
    }
}
