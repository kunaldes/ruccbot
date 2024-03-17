use lazy_static::lazy_static;
use poise::serenity_prelude as serenity;
use rand::{seq::SliceRandom, Rng};
use regex::Regex;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^\s*(\d+)\s*-\s*(\d+)\s*$").unwrap();
}

fn split_input(input: String) -> Vec<String> {
    if input.contains(',') {
        input
            .split(',')
            .skip_while(|x| x.is_empty())
            .map(|x| x.trim().into())
            .collect()
    } else {
        input.split_whitespace().map(|x| x.into()).collect()
    }
}

fn do_choose(choices: String) -> Option<String> {
    if let Some(captures) = RE.captures(choices.as_str()) {
        let (_, [i1, i2]) = captures.extract();
        if let (Ok(num1), Ok(num2)) = (i1.parse::<i32>(), i2.parse::<i32>()) {
            if num1 <= num2 {
                return Some(rand::thread_rng().gen_range(num1..=num2).to_string());
            }
        }
    }
    let foo: Vec<String> = split_input(choices);
    foo.choose(&mut rand::thread_rng()).cloned()
}

#[poise::command(prefix_command, aliases("c"))]
async fn choose(ctx: Context<'_>, #[rest] choices: Option<String>) -> Result<(), Error> {
    if let Some(choice) = do_choose(choices.unwrap_or_default()) {
        ctx.say(choice).await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![choose()],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(String::from(".")),
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
