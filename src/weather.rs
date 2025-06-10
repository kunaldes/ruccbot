use crate::{Context, Error};

/// Returns the weather.
#[poise::command(prefix_command, aliases("w"))]
pub async fn weather(
    ctx: Context<'_>,
    #[description = "Location to fetch the weather for."]
    #[rest]
    _location: Option<String>,
) -> Result<(), Error> {
    ctx.say("It's 69° somewhere.").await?;
    Ok(())
}
