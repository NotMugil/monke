use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Display a user's profile picture
#[poise::command(prefix_command, slash_command)]
pub async fn pfp(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let requester = ctx.author();
    let avatar_url = u.avatar_url().unwrap_or_else(|| u.default_avatar_url());
    let embed = serenity::CreateEmbed::new()
        .title(format!("{}'s Profile Picture", u.name))
        .color(0x3498db)
        .image(avatar_url)
        .footer(
            serenity::CreateEmbedFooter::new(format!("Requested by: {}", requester.name.clone()))
                .icon_url(requester.avatar_url().unwrap_or_default()),
        );

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
