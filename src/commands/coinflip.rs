use crate::{Context, Error};

/// Flip a coin to get either head or tails
#[poise::command(prefix_command, slash_command)]
pub async fn coinflip(ctx: Context<'_>) -> Result<(), Error> {
    let result = if rand::random::<bool>() {
        "Heads"
    } else {
        "Tails"
    };

    ctx.send(poise::CreateReply::default().content(format!("You flipped: {}", result)))
        .await?;
    Ok(())
}
