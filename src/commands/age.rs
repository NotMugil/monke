use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Displays your or another user's account creation date
#[poise::command(prefix_command, slash_command)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let requester = ctx.author();
    let embed = serenity::CreateEmbed::new()
        .title("Account Information")
        .description(format!(
            "{}'s account was created at {}",
            u.name,
            u.created_at()
        ))
        .color(0x3498db)
        .thumbnail(u.avatar_url().unwrap_or_default())
        .footer(
            serenity::CreateEmbedFooter::new(format!("Requested by: {}", requester.name.clone()))
                .icon_url(requester.avatar_url().unwrap_or_default()),
        );

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
