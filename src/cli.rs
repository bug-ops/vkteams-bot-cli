use crate::cli::types::*;
use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use core::default::Default;
use vkteams_bot::types::*;

pub mod types;
pub struct Cli {
    pub bot: Bot,
    pub matches: Opts,
}

impl Default for Cli {
    fn default() -> Self {
        Self {
            bot: Bot::default(),
            matches: Opts::parse(),
        }
    }
}

impl Cli {
    /// Match result and print it
    pub fn match_result<T>(&self, result: Result<T>)
    where
        T: serde::Serialize,
    {
        match result {
            Ok(r) => {
                println!("{}", serde_json::to_string(&r).unwrap().green());
                std::process::exit(exitcode::OK);
            }
            Err(e) => {
                println!("{:?}", e.to_string().red());
                std::process::exit(exitcode::DATAERR);
            }
        }
    }
    pub async fn match_input(&self) {
        match self.matches.clone().subcmd {
            SubCommand::SendText { user_id, message } => self.match_result(
                self.bot
                    .messages_send_text(
                        RequestMessagesSendText::new(ChatId(user_id))
                            .set_text((message, ParseMode::HTML))
                            .build(),
                    )
                    .await,
            ),
            SubCommand::SendFile { user_id, file_path } => self.match_result(
                self.bot
                    .messages_send_file(RequestMessagesSendFile::new(ChatId(user_id)), file_path)
                    .await,
            ),
            SubCommand::GetEvents => {
                self.match_result(self.bot.get_events().await);
            }
        }
    }
}
