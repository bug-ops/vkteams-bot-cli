<div>
<a href="https://docs.rs/vkteams-bot-cli/latest/vkteams_bot_cli/">
    <img src="https://img.shields.io/docsrs/vkteams-bot-cli/latest">
</a>
<a href="https://crates.io/crates/vkteams-bot-cli">
    <img src="https://img.shields.io/crates/v/vkteams-bot-cli">
</a>
 <a href="https://github.com/k05h31/vkteams-bot-cli/actions">
    <img src="https://github.com/k05h31/vkteams-bot-cli/workflows/Rust/badge.svg">
</a>
</div>

# VK Teams Bot API Cli

VK Teams Bot API terminal application.

## Table of Contents

- [Environment](#environment)
- [Usage](#usage)

## Environment

1. Begin with bot API following [instructions](https://teams.vk.com/botapi/?lang=en)
2. Set environment variables or save in `.env` file
```bash
# Unix-like
$ export VKTEAMS_BOT_API_TOKEN=<Your token here> #require
$ export VKTEAMS_BOT_API_URL=<Your base api url> #require
$ export VKTEAMS_PROXY=<Proxy> #optional

$ cargo install vkteams-bot-cli

# Windows
$ set VKTEAMS_BOT_API_TOKEN=<Your token here> #require
$ set VKTEAMS_BOT_API_URL=<Your base api url> #require
$ set VKTEAMS_PROXY=<Proxy> #optional

# Build from source
$ cargo install vkteams-bot-cli
```

## Usage

```bash
# Help
$ vkteams-bot-cli --help
# Listen for events
$ vkteams-bot-cli get-events -l true | grep "ALARM"
# Send text
$ vkteams-bot-cli send-text -u <USER_ID> -m "text message"
# Send file
$ vkteams-bot-cli send-file -u <USER_ID> -f <FILE_PATH>
# Download file
$ vkteams-bot-cli get-file -i <FILE_ID> -f <FILE_PATH>
```

## Commands
| Command | Description |
|---------|-------------|
| `get-events` | Listen for events (long polling) |
| `send-text` | Send text message |
| `send-file` | Send file to chat |
| `get-file`  | Download file from API |