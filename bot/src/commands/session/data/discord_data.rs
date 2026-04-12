use serde::{Deserialize, Serialize};
use serenity::all::{GuildChannel, Role};

#[derive(Serialize, Deserialize)]
pub struct DiscordData {
    pub id: u64, pub name: String
} impl From<GuildChannel> for DiscordData {
    fn from(value: GuildChannel) -> Self {
        Self { id: value.id.get(), name: value.name }
    }
} impl From<Role> for DiscordData {
    fn from(value: Role) -> Self {
        Self { id: value.id.get(), name: value.name }
    }
}