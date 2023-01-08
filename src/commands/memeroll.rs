use crate::commands::{Context, Error};
use rand::seq::SliceRandom;
use poise::serenity_prelude::{self as serenity};


/// Posts a meme randomly from #image_post_memes
#[poise::command(slash_command, prefix_command)]
pub async fn memeroll(
    ctx: Context<'_>,
    #[description = "Caption"] caption: Option<String>,
) -> Result<(), Error> {
    let meme_msgs_lock = ctx.data().meme_msgs.read().await;
    let selected_message = &*meme_msgs_lock.choose(&mut rand::thread_rng()).expect("TODO handle meme_msgs being empty");
    
    let image_url;
    if selected_message.attachments.len() > 0 {
        image_url = &selected_message.attachments[0].url;
    } else {
        image_url = &selected_message.content;
    }

    ctx.send(| builder | {
        builder
            .content(image_url)
            .components(|components| {
                components
                    .create_action_row(|action_row| {
                        let mut ret = action_row;
                        if !caption.is_none() {
                            ret = ret
                                .create_button(|button| {
                                    button
                                        .style(serenity::ButtonStyle::Secondary)
                                        .label(caption.unwrap())
                                        .custom_id("caption")
                                        .disabled(true)
                                })
                            }
                        let timestamp = selected_message.timestamp.format("%Y-%m-%d %H:%M");
                        ret = ret.create_button(|button| {
                            button
                                .style(serenity::ButtonStyle::Link)
                                .label(format!("{}, {}", selected_message.author.name, timestamp))
                                .url(selected_message.link())
                        });
                        ret
                    })
            })
    }).await?;
    Ok(())
}
