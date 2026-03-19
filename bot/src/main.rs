mod commands;
mod util;

use std::fs;
use poise::serenity_prelude as serenity;
use serenity::{all::GuildId, json};
use serde::Deserialize;

use crate::{commands::{add_game, list_games, pick_random_game, poll, remove_game}, util::RequestData}; 

#[derive(Deserialize)]
struct Config {
    server_id: u64,
    token: String
}

#[tokio::main]
async fn main() {
    let config_file = fs::read_to_string("config.json").unwrap();
    let config = json::from_str::<Config>(config_file).unwrap();
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![poll(), pick_random_game(), add_game(), remove_game(), list_games()],
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