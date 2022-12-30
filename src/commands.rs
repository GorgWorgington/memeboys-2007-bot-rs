mod register;
mod coinflip;
mod react;
pub use self::register::*;
pub use self::coinflip::*;
pub use self::react::*;

pub struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
