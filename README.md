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
$ export VKTEAMS_VKTEAMS_BOT_API_TOKEN=<Your token here> #require
$ export VKTEAMS_BOT_API_URL=<Your base api url> #require
$ export VKTEAMS_PROXY=<Proxy> #optional

# Windows
$ set VKTEAMS_VKTEAMS_BOT_API_TOKEN=<Your token here> #require
$ set VKTEAMS_BOT_API_URL=<Your base api url> #require
$ set VKTEAMS_PROXY=<Proxy> #optional
```
3. Put lines in you `Cargo.toml` file
```toml
[dependencies]
vkteams_bot_cli = "0.1"
```

## Usage