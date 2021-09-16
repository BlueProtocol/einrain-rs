# Einrain-rs: A Discord Bot for Blue Protocol

Einrain is a Discord bot made for the game Blue Protocol and the [Blue Protocol Database's](https://blue-protocol-db.com/) Discord server. Although it's still in its infancy, we hope it will come to be an invaluable tool for players of Blue Protocol.

### Features

- Slash Commands
- `/class <class_type>`: Returns all information associated with the selected class
- `/skills <class_type>`: Returns all skills of the selected class
- `/help <command>`: Returns information on the given command or all commands
- `~register <global>`: Registers slash commands with the guild or global (Owner only)
- `~unregister <global>`: Unregisters slash commands with the guild or global (Owner only)

### Setup

If you haven't already, install Rust and Cargo with [rustup](https://rustup.rs).

Clone or download this repo.

Create a `config.toml` file in the root of the bot's directory, formatted like
```toml
token = '<bot token here>' # Token for the Discord bot
owner_id = <bot owners id here> # Discord ID of bot owner for registering and unregistering slash commands
log_file = 'log.txt' # Unused
colour = 12345678 # Unused
```

Create a Discord Bot Application [here](https://discord.com/developers/applications). Go to the "Bot" tab on the left. Copy and paste the bot's token into your `config.toml` as indicated above.

Find the Discord user you want to designate as the "owner". This is the user who will be able to register slash commands with Discord after the bot is running and in a server. Right click on their username and click "Copy ID". Copy and paste the owner's id into your `config.toml` as indicated above.

Go back to the root of the bot's directory and execute the following command to run the bot normally:
```
cargo run --release
```

Go back to the Discord Bot Application website and the "OAuth2" tab. Select the scope "bot" in the URL Generator and copy the URL at the bottom. Go to the URL and add the bot to the server you want.

If you go to the server and attempt to execute a slash command at this point, you won't be able to. Slash commands must be registered with Discord before they are able to be used. The owner must enter the server with the bot and execute `~register`. Commands should then be available in the slash commands menu soon, if they aren't already. Tip: Slash commands can be accessed by typing "/" into chat.

All done!

### Running with Docker

First we compile the bot and build the Docker container by executing:
```
docker build -t einrain .
```

To run it, we execute:
```
docker run -it --rm --name einrain-running einrain
```
