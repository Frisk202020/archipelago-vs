use std::fs;
use rand::Rng;

use crate::util::{Context, Data, Error};

#[poise::command(slash_command)]
pub(crate) async fn pick_random_game(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let raw_data = fs::read_to_string("data.json")?;
    let data = serde_json::from_str::<Data>(raw_data.as_str())?;

    ctx.say(get_message(&data, ctx)).await?;
    Ok(())
}

fn get_message(data: &Data, ctx: Context<'_>) -> String {
    let mut rng = rand::rng();
    for p in &data.random_game_picker {
        if p.player == ctx.author().name {
            if p.games.is_empty() {
                return "Par pitié ajoute un jeu à ta liste".to_string();
            }

            return format!(
                "Aujourd'hui tu as envie de jouer à {}", 
                p.games[rng.next_u32() as usize % p.games.len() - 1]
            );
        }
    }

    return "Tu es inconnu au bataillon".to_string();
}