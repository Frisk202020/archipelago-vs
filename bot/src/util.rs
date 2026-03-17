use serde::{Deserialize, Serialize};

pub(crate) struct RequestData {}
pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;
pub(crate) type Context<'a> = poise::Context<'a, RequestData, Error>;

#[derive(Serialize, Deserialize)]
pub(crate) struct Data {
    pub(crate) random_game_picker: Vec<PlayerData>
}

#[derive(Serialize, Deserialize)]
pub(crate) struct PlayerData {
    pub(crate) player: String,
    pub(crate) games: Vec<String>
}