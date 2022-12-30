use crate::commands::{Context, Error};

/// Post a reaction image/video
#[poise::command(prefix_command, slash_command, subcommands(
    "beans",
    "okretard", 
    "bruh", 
    "nigra", 
    "circus", 
    "green", 
    "gawk", 
    "penismusic", 
    "timesup", 
    "acecringe", 
    "acewrong", 
    "acelogic1", 
    "acelogic2", 
    "spongechoonch", 
    "mrbeandisgust",
    "hollow"
))]
pub async fn react(_ctx: Context<'_>) -> Result<(), Error> { Ok(()) }

/// (Image) This n eating beans
#[poise::command(prefix_command, slash_command)]
pub async fn beans(ctx: Context<'_>) -> Result<(), Error> { 
    ctx.say("https://cdn.discordapp.com/attachments/274323160143233025/763292145623760937/Screenshot_20191020-125459.jpg").await?;
    Ok(())
}

/// (Image) Ok retard
#[poise::command(prefix_command, slash_command)]
pub async fn okretard(ctx: Context<'_>) -> Result<(), Error> { 
    ctx.say("https://bigmemes.funnyjunk.com/comments/Blank+_c0de2c928a8b08136f306785ac7c24d6.jpg").await?;
    Ok(())
}

/// (Video) Bruh sound effect
#[poise::command(prefix_command, slash_command)]
pub async fn bruh(ctx: Context<'_>) -> Result<(), Error> { 
    ctx.say("https://youtu.be/2ZIpFytCSVc").await?;
    Ok(()) 
}

/// (Video) Loud nigra
#[poise::command(prefix_command, slash_command)]
pub async fn nigra(ctx: Context<'_>) -> Result<(), Error> { 
    ctx.say("https://youtu.be/IFOvTWSYCb").await?;
    Ok(()) 
}

/// (Image) You are the entire circus
#[poise::command(prefix_command, slash_command)]
pub async fn circus(ctx: Context<'_>) -> Result<(), Error> { 
    ctx.say("https://i.kym-cdn.com/photos/images/newsfeed/001/847/311/ace.png").await?;
    Ok(()) 
}

/// (Video) That things green
#[poise::command(prefix_command, slash_command)]
pub async fn green(ctx: Context<'_>) -> Result<(), Error> { 
    ctx.say("https://www.youtube.com/watch?v=8Cx_wdIOyzI").await?;
    Ok(()) 
}

/// (Video) Squidward chokes on a fork 10 hours
#[poise::command(prefix_command, slash_command)]
pub async fn gawk(ctx: Context<'_>) -> Result<(), Error> { 
    ctx.say("https://www.youtube.com/watch?v=oeux7476ZSY").await?;
    Ok(()) 
}

/// (Video) Penis music
#[poise::command(prefix_command, slash_command)]
pub async fn penismusic(ctx: Context<'_>) -> Result<(), Error> { 
    ctx.say("https://youtu.be/c4KNd0Yv6d0").await?;
    Ok(()) 
}

/// (Video) Times up spongebob
#[poise::command(prefix_command, slash_command)]
pub async fn timesup(ctx: Context<'_>) -> Result<(), Error> { 
    ctx.say("https://youtu.be/GyRZoBUeLK8").await?;
    Ok(()) 
}

/// (Image) That's going in the cringe reel
#[poise::command(prefix_command, slash_command)]
pub async fn acecringe(ctx: Context<'_>) -> Result<(), Error> { 
    ctx.say("https://cdn.discordapp.com/attachments/763286832968368149/763301964804259860/AACringe.png").await?;
    Ok(()) 
}

/// (Image) Phoenix says wrong
#[poise::command(prefix_command, slash_command)]
pub async fn acewrong(ctx: Context<'_>) -> Result<(), Error> { 
    ctx.say("https://cdn.discordapp.com/attachments/763286832968368149/763302202264518687/AAWrong.png").await?;
    Ok(()) 
}

/// (Image) It would seem logic doesn't exist
#[poise::command(prefix_command, slash_command)]
pub async fn acelogic1(ctx: Context<'_>) -> Result<(), Error> { 
    ctx.say("https://cdn.discordapp.com/attachments/763286832968368149/763302488215650304/logic.png").await?;
    Ok(()) 
}

/// (Image) "Lodge-ick" ya say?
#[poise::command(prefix_command, slash_command)]
pub async fn acelogic2(ctx: Context<'_>) -> Result<(), Error> { 
    ctx.say("https://cdn.discordapp.com/attachments/763286832968368149/763302551952556042/logic2_blur.png").await?;
    Ok(()) 
}

/// (Image) Spongebob choonches
#[poise::command(prefix_command, slash_command)]
pub async fn spongechoonch(ctx: Context<'_>) -> Result<(), Error> { 
    ctx.say("https://cdn.discordapp.com/attachments/763286832968368149/763303345745559562/spongebob1.jpg").await?;
    Ok(()) 
}

/// (Image) Mr. Bean disgust
#[poise::command(prefix_command, slash_command)]
pub async fn mrbeandisgust(ctx: Context<'_>) -> Result<(), Error> { 
    ctx.say("https://cdn.discordapp.com/attachments/274322598702219275/763310061728432148/confusion.png").await?;
    Ok(()) 
}

/// (Image) Hollow Feelsman
#[poise::command(prefix_command, slash_command)]
pub async fn hollow(ctx: Context<'_>) -> Result<(), Error> { 
    ctx.say("https://cdn.discordapp.com/attachments/763286832968368149/763310346571218954/hollow_feelsman.jpg").await?;
    Ok(()) 
}
