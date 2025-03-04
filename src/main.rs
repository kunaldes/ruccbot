mod choices;
mod slap;
mod time;

use poise::{builtins::HelpConfiguration, serenity_prelude as serenity};

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays this help text.
#[poise::command(prefix_command)]
async fn help(
    ctx: Context<'_>,
    #[description = "Command to get help for"]
    #[rest]
    command: Option<String>,
) -> Result<(), Error> {
    let extra_text_at_bottom = "\
Type `.help command` for more info on a command.";
    poise::builtins::help(
        ctx,
        command.as_deref(),
        HelpConfiguration {
            extra_text_at_bottom,
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                choices::choose(),
                choices::order(),
                time::time(),
                slap::slap(),
                help(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(".".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
