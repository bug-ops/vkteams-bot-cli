mod types;
use crate::cli::types::*;
use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use core::default::Default;
use vkteams_bot::types::*;
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
/// Implementation for CLI
impl Cli {
    /// Match input with subcommands
    pub async fn match_input(&self) {
        // Match subcommands
        match self.matches.clone().subcmd {
            // Subcommand for send text message
            SubCommand::SendText { user_id, message } => {
                match_result(
                    &self
                        .bot
                        .messages_send_text(
                            RequestMessagesSendText::new(ChatId(user_id))
                                .set_text((message, ParseMode::HTML))
                                .build(),
                        )
                        .await,
                );
            }
            // Subcommand for send file
            SubCommand::SendFile { user_id, file_path } => {
                match_result(
                    &self
                        .bot
                        .messages_send_file(
                            RequestMessagesSendFile::new(ChatId(user_id)),
                            file_path,
                        )
                        .await,
                );
            }
            // Subcommand for get events
            SubCommand::GetEvents { listen } => {
                match listen {
                    Some(true) => {
                        self.bot.event_listener(match_result).await;
                    }
                    _ => {
                        match_result(&self.bot.get_events().await);
                    }
                };
            }
        }
        std::process::exit(exitcode::OK);
    }
}

/// Match result and print it
pub fn match_result<T>(result: &Result<T>)
where
    T: serde::Serialize,
{
    match result {
        Ok(r) => {
            println!("{}", serde_json::to_string(&r).unwrap().green());
        }
        Err(e) => {
            println!("{}", e.to_string().red());
            std::process::exit(exitcode::DATAERR);
        }
    }
}
