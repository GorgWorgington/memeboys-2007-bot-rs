mod register;
mod coinflip;
mod memeroll;
mod react;
mod stats;
use poise::serenity_prelude::ChannelId;
use poise::serenity_prelude::Message;
use poise::serenity_prelude::RwLock;

pub use self::register::*;
pub use self::coinflip::*;
pub use self::memeroll::*;
pub use self::react::*;
pub use self::stats::*;

// User data, which is stored and accessible in all command invocations
pub struct Data {
  pub meme_channel_id: ChannelId,
  pub meme_msgs: RwLock<Vec<Message>>,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
