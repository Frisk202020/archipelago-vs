use std::{collections::HashMap, time::Instant};
use poise::CreateReply;
use serde::{Deserialize, Serialize};
use serenity::all::{CreateActionRow, CreateButton};

use crate::{interaction::ComponentInteraction, util::{Context, Error, get_json_data, write_json_data}};

const PATH: &'static str = "data/session.json";

#[derive(Serialize, Deserialize)]
struct Data {
    active: bool,
    start_time: u64,
    finished: HashMap<String, u64>
} impl Data {
    fn get() -> Result<Self, Error> { get_json_data(PATH) }
    fn write(&self) -> Result<(), Error> { write_json_data(self, PATH) }
} impl Default for Data {
    fn default() -> Self {
        Self { 
            active: true, 
            start_time: Instant::now().elapsed().as_secs(),
            finished: HashMap::new()
        }
    }
}

#[poise::command(slash_command)]
pub(crate) async fn start_session(ctx: Context<'_>) -> Result<(), Error> {
    let data = if let Ok(data) = Data::get() { data } else {
        let reply = CreateReply::default().components(vec![
            CreateActionRow::Buttons(vec![
                CreateButton::new(ComponentInteraction::SessionDataNotFoundOk.as_ref()).label("Ok")
            ])
        ]);
        ctx.send(reply).await?;

        return Ok(());
    };

    if data.active {
        //warn msg
    } else {
        let data = Data::default();
        data.write()?;
        ctx.say("Hop là c'est parti").await?;
    }

    Ok(())
}