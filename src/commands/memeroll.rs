use crate::commands::{Context, Error};
use poise::serenity_prelude::{ButtonStyle, CreateActionRow, CreateButton};
use rand::seq::SliceRandom;

/// Posts a meme randomly from #image_post_memes
#[poise::command(slash_command, prefix_command)]
pub async fn memeroll(
    ctx: Context<'_>,
    #[description = "Caption"] caption: Option<String>,
) -> Result<(), Error> {
    let meme_msgs_lock = ctx.data().meme_msgs.read().await;
    let selected_message = &*meme_msgs_lock
        .choose(&mut rand::thread_rng())
        .expect("TODO handle meme_msgs being empty");

    let image_url;
    if selected_message.attachments.len() > 0 {
        image_url = &selected_message.attachments[0].url;
    } else {
        image_url = &selected_message.content;
    }

    let mut buttons = Vec::with_capacity(2);
    if let Some(c) = caption {
        buttons.push(
            CreateButton::new("caption")
                .style(ButtonStyle::Secondary)
                .label(c)
                .disabled(true),
        )
    }

    let timestamp = selected_message.timestamp.format("%Y-%m-%d %H:%M");
    buttons.push(
        CreateButton::new_link(selected_message.link())
            .label(format!("{}, {}", selected_message.author.name, timestamp)),
    );

    let reply = poise::CreateReply::default()
        .content(image_url)
        .components(vec![CreateActionRow::Buttons(buttons)]);

    ctx.send(reply).await?;

    Ok(())
}
