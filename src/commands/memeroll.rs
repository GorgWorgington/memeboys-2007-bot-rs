use crate::commands::{Context, Error};
use rand::seq::SliceRandom;


/// Posts a meme randomly from #image_post_memes
#[poise::command(slash_command, prefix_command)]
pub async fn memeroll(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let selected_message = ctx.data().meme_msgs.choose(&mut rand::thread_rng()).unwrap();
    
    // ctx.say(response).await?;
    Ok(())
}
