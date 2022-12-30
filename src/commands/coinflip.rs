use crate::commands::{Context, Error};
use rand::seq::SliceRandom;


/// Flips a coin
#[poise::command(slash_command, prefix_command)]
pub async fn coinflip(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let coin_flip_options = vec!["Heads", "Tails"];
    let coin_flip_result = coin_flip_options.choose(&mut rand::thread_rng()).unwrap();
    let response = format!("> {}", coin_flip_result);
    ctx.say(response).await?;
    Ok(())
}
