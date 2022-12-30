use std::fs;
use commands::Data;
use poise::serenity_prelude as serenity;

mod commands;
mod config;
use config::Config;


#[tokio::main]
async fn main() {

  //TODO improve error reporting (don't allow panics)
  let config_file = fs::read_to_string("config.json").expect("Failed to read config file");
  let config = Config::from_json(config_file).expect("Failed to deserialize config file");

  //Grab token from parsed config file
  let token = config.token();

  // Set gateway intents, which decides what events the bot will be notified about
  let intents =
    serenity::GatewayIntents::GUILD_MESSAGES |
    serenity::GatewayIntents::DIRECT_MESSAGES |
    serenity::GatewayIntents::MESSAGE_CONTENT;

  let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
              commands::register(),
              commands::coinflip(),
              commands::react(),
              ],
            ..Default::default()
        })
        .token(token)
        .intents(intents)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}
