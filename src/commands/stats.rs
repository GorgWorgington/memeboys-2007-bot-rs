use poise::serenity_prelude as serenity;
use crate::commands::{Context, Error};

// TODO: This is a hacky way to get the number of memes a user has posted
// Instead, we should be storing this data in a database, and updating every time a meme is posted
fn get_meme_count(user: &serenity::User, meme_messages: &Vec<serenity::Message>) -> u32 {
    let mut meme_message_count = 0;
    for message in meme_messages.iter() {
        if message.author.id == user.id {
            meme_message_count += 1;
        }
    }
    meme_message_count
}

/// Posts a meme randomly from #image_post_memes
#[poise::command(slash_command, prefix_command)]
pub async fn stats(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    if let Some(u) = user.as_ref() {
        let meme_message_count = get_meme_count(u, &ctx.data().meme_msgs);
        ctx.say(format!("> **Meme count**\n> {}: {} memes posted", u, meme_message_count)).await?;
        return Ok(());
    } else {
        return Ok(()); 
    }
}
