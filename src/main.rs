use std::fs;
use serenity::{async_trait, model::{prelude::{Message, Ready}, user::OnlineStatus}, prelude::*};

mod config;
use config::Config;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn message(&self, ctx: Context, msg: Message) {
    if msg.content == "!ping" {
      if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
        println!("Error sending message: {:?}", why);
      }
    }
  }

  async fn ready(&self, _: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);
  }
}

#[tokio::main]
async fn main() {

  //TODO improve error reporting (don't allow panics)
  let config_file = fs::read_to_string("config.json").expect("Failed to read config file");
  let config = Config::from_json(config_file).expect("Failed to deserialize config file");

  //Grab token from parsed config file
  let token = config.token();

  // Set gateway intents, which decides what events the bot will be notified about
  let intents =
        GatewayIntents::GUILD_MESSAGES
      | GatewayIntents::DIRECT_MESSAGES
      | GatewayIntents::MESSAGE_CONTENT;

  // Create a new instance of the Client, logging in as a bot. This will
  // automatically prepend your bot token with "Bot ", which is a requirement
  // by Discord for bot users.
  let mut client =
      Client::builder(&token, intents)
      .event_handler(Handler)
      .presence(None, OnlineStatus::Invisible)
      .await.expect("Error creating client");

  // Finally, start a single shard, and start listening to events.
  //
  // Shards will automatically attempt to reconnect, and will perform
  // exponential backoff until it reconnects.
  if let Err(why) = client.start().await {
      println!("Client error: {:?}", why);
  }
}
