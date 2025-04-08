use crate::{Context, Error};

/// A command to return the GitHub repository URL of the bot
#[poise::command(prefix_command, slash_command)]
pub async fn repo(ctx: Context<'_>) -> Result<(), Error> {
    let repo_url = "https://github.com/NotMugil/rust-discord-bot";

    let reply = format!("Check out the bot's GitHub repository here: {}", repo_url);

    ctx.send(poise::CreateReply::default().content(reply).ephemeral(true))
        .await?;

    Ok(())
}
