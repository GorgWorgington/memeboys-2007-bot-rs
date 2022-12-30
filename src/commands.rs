mod age;
mod register;
mod coinflip;
mod memeroll;
use poise::serenity_prelude::ChannelId;
use poise::serenity_prelude::Message;

pub use self::age::*;
pub use self::register::*;
pub use self::coinflip::*;
pub use self::memeroll::*;

// User data, which is stored and accessible in all command invocations
pub struct Data {
  meme_channel_id: ChannelId,
  meme_msgs: Vec<Message>,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
