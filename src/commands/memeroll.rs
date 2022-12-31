use crate::commands::{Context, Error};
use rand::seq::SliceRandom;


/// Posts a meme randomly from #image_post_memes
#[poise::command(slash_command, prefix_command)]
pub async fn memeroll(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let selected_message = ctx.data().meme_msgs.choose(&mut rand::thread_rng()).expect("TODO handle meme_msgs being empty");
    
    let image_url;
    if selected_message.attachments.len() <= 0 {
        image_url = &selected_message.attachments[0].url;
    } else {
        image_url = &selected_message.content;
    }
    ctx.say(image_url).await?;
    Ok(())
}
