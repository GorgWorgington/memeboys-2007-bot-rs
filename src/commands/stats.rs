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
    let user = match user.as_ref() {
        Some(u) => u,
        None => ctx.author(),
    };

    let meme_msgs_lock = ctx.data().meme_msgs.read().await;

    let meme_message_count = get_meme_count(user, &*meme_msgs_lock);
    ctx.say(format!("> **Meme count**\n> {}: {} memes posted", user, meme_message_count)).await?;
    Ok(())
}
