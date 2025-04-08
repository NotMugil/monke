use crate::{Context, Error};
use rand::prelude::*;

/// A Magic 8 ball command that can predict the future
#[poise::command(prefix_command, slash_command)]
pub async fn ball(
    ctx: Context<'_>,
    #[description = "Your question for the magic 8-ball"] question: String,
) -> Result<(), Error> {
    let responses = [
        "Yes, definitely",
        "Ask again later",
        "Don’t count on it",
        "Outlook good",
        "Very doubtful",
        "Yes, but be careful",
        "Cannot predict now",
        "It’s certain",
        "Most likely",
        "Signs point to yes",
        "Don't hold your breath",
        "Absolutely",
        "Reply hazy, try again",
        "My sources say no",
        "Without a doubt",
        "Cannot say for sure",
        "As I see it, yes",
        "Concentrate and ask again",
        "Yes, in due time",
        "Don’t bet on it",
        "Outlook not so good",
        "Definitely not",
        "Yes, but proceed with caution",
        "Better not tell you now",
        "Looks like yes",
        "Cannot foresee that",
        "Try again later",
        "Not in a million years",
        "The future is unclear",
        "Most likely not",
    ];

    let response = responses.choose(&mut rand::rng()).unwrap();

    let reply = format!("**Question:** {}\n**Answer:** {}\n\n", question, response,);

    ctx.send(poise::CreateReply::default().content(reply))
        .await?;
    Ok(())
}
