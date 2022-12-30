mod age;
mod register;
mod coinflip;
pub use self::age::*;
pub use self::register::*;
pub use self::coinflip::*;

pub struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
