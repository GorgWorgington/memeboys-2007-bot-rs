use std::{fs};
use commands::Data;
use poise::{serenity_prelude::{self as serenity, ChannelId, RwLock }, futures_util::StreamExt,};

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
              commands::stats(),
              commands::memeroll(),
              commands::react(),
              ],
              event_handler: |_ctx, event, _framework, data| {
                Box::pin(async move {
                    match event {
                        poise::Event::Message { new_message } => {
                            if new_message.channel_id == data.meme_channel_id
                            && (new_message.embeds.len() > 0 || new_message.attachments.len() > 0) {
                                let mut meme_msgs_lock = data.meme_msgs.write().await;
                                (*meme_msgs_lock).push(new_message.to_owned());
                                println!("Added a meme");
                            }
                        },
                        _ => (),
                    }
                    Ok(())
                })
              },
            ..Default::default()
        })
        .token(token)
        .intents(intents)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                ctx.invisible().await;
                let meme_channel_id = ChannelId(274323160143233025);
                let mut meme_msgs = Vec::new();

                let mut count = 0;

                let mut messages = meme_channel_id.messages_iter(&ctx).boxed();
                while let Some(message_result) = messages.next().await {
                    match message_result {
                        Ok(message) => {
                            if message.embeds.len() > 0 || message.attachments.len() > 0 {
                                meme_msgs.push(message);
                                count += 1;
                                println!("added {} memes", count);
                                // if count > 10 { break };
                            }
                        },
                        Err(error) => eprintln!("Uh oh! Error: {}", error),
                    }
                }
                println!("found {} memes", count);

                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                let data = Data {
                  meme_channel_id,
                  meme_msgs: RwLock::new(meme_msgs),
                };

                Ok(data)
            })
        });

    framework.run().await.unwrap();
}
