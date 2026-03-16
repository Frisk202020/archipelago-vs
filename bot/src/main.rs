use std::{fs, time::Duration};

use poise::serenity_prelude as serenity;
use serenity::{all::{CreatePollAnswer, GuildId}, json};
use serde::Deserialize; 

struct Data {}

#[derive(Deserialize)]
struct Config {
    server_id: u64,
    token: String
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command)]
async fn poll(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let poll = serenity::CreatePoll::new()
        .question("Archipelago ce week-end ?")
        .answers(
            vec!(
                "Vendredi soir",
                "Samedi aprem",
                "Dimanche aprem"
            )
            .into_iter()
            .map(|x| CreatePollAnswer::new().text(x))
            .collect()
        )
        .duration(Duration::from_secs(3600 * 24 * 5))
        .allow_multiselect();
    let message = serenity::CreateMessage::new().poll(poll);
    ctx.say("Perdu").await?;
    ctx.channel_id().send_message(ctx.http(), message).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let config_file = fs::read_to_string("config.json").unwrap();
    let config = json::from_str::<Config>(config_file).unwrap();
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), poll()],
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(ctx.http.clone(), &framework.options().commands, GuildId::new(config.server_id)).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(config.token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}