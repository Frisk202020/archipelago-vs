mod commands;
mod util;
mod interaction;

use std::fs;
use poise::serenity_prelude as serenity;
use ::serenity::all::{Context, FullEvent};
use serenity::{all::GuildId, json};
use serde::Deserialize;

use crate::{commands::{add_game, finish, list_games, pick_random_game, poll, remove_game, start_session}, util::{Error, RequestData}}; 

#[derive(Deserialize)]
struct Config {
    server_id: u64,
    token: String
}

async fn event_handler(
    ctx: &Context, event: &FullEvent 
) -> Result<(), Error> {
    match event {
        FullEvent::InteractionCreate { interaction } => {
            interaction::handle(ctx.http.clone(), interaction).await
        },
        _ => Ok(())
    }
}

#[tokio::main]
async fn main() {
    let config_file = fs::read_to_string("data/config.json").unwrap();
    let config = json::from_str::<Config>(config_file).unwrap();
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                poll(), pick_random_game(), add_game(), 
                remove_game(), list_games(), start_session(),
                finish()
            ],
            event_handler: |ctx, event, _, _| Box::pin(event_handler(ctx, event)),
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(ctx.http.clone(), &framework.options().commands, GuildId::new(config.server_id)).await?;
                Ok(RequestData {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(config.token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}