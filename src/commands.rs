mod coinflip;
mod memeroll;
mod react;
mod register;
mod stats;
use poise::serenity_prelude::prelude::RwLock;
use poise::serenity_prelude::ChannelId;
use poise::serenity_prelude::Message;

pub use self::coinflip::*;
pub use self::memeroll::*;
pub use self::react::*;
pub use self::register::*;
pub use self::stats::*;

// User data, which is stored and accessible in all command invocations
pub struct Data {
    pub meme_channel_id: ChannelId,
    pub meme_msgs: RwLock<Vec<Message>>,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
