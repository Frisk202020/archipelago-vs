use std::collections::HashMap;
use rand::Rng;
use serenity::all::{Mentionable, User};
use crate::util::{Context, Error, Output, THUMB, get_json_data, vec_to_list, write_json_data};

type Data = HashMap<String, Vec<String>>;

const PATH: &'static str = "data/games.json";
const GIF: &'static str = "https://giphy.com/gifs/spongebob-26ufnwz3wDUli7GU0";

#[poise::command(slash_command, subcommands("pick_random_game", "add_game", "remove_game", "list_games"))]
pub async fn picker(_ctx: Context<'_>) -> Output { Ok(()) }

fn get_data() -> Result<Data, Error> { get_json_data(PATH) }
fn write_data(data: &Data) -> Output { write_json_data(data, PATH) }

/// Choisir un jeu aléatoire parmi la liste de jeux enregistrée du client.
/// 
/// Cette commande est nécessairement appliquée au client (pas d'accès délégué).
#[poise::command(slash_command)]
pub async fn pick_random_game(
    ctx: Context<'_>,
) -> Output {
    let data = get_data()?;

    ctx.say(get_message(&data, ctx)).await?;
    Ok(())
}

/// Ajouter un jeu à la bibliothèque de cet utilisateur. 
/// 
/// Cette commande est nécessairement appliquée au client (pas d'accès délégué).
#[poise::command(slash_command)]
async fn add_game(
    ctx: Context<'_>,
    #[description = "Un jeu goatesque à ajouter à ta liste"] game: String
) -> Output {
    let mut data = get_data()?;
    let name = ctx.author().name.as_str();
    if let Some(games) = data.get_mut(name) {
        for g in games.iter() {
            if g == &game {
                ctx.say("Frérot il est déjà dans la liste :joy:").await?;
                return Ok(());
            }
        }

        games.push(game);
    } else {
        data.insert(name.to_string(), vec![game]);
    }
    
    write_data(&data)?;
    ctx.say(THUMB).await?;
    Ok(())
}

/// Enlever un jeu de la bibliothèque du client
/// 
/// Attention : l'orthographe du jeu à enlever doit exactement correspondre.
/// Cette commande est nécessairement appliquée au client (pas d'accès délégué).
#[poise::command(slash_command)]
async fn remove_game(
    ctx: Context<'_>,
    #[description = "Le nom du jeu à dégager"] game: String,
) -> Output {
    let mut data = get_data()?;
    let name = ctx.author().name.as_str();

    let msg = if let Some(games) = data.get_mut(name) {
        let mut id: Option<usize> = None;
        for i in 0..games.len() {
            if games[i] == game {
                id = Some(i);
                break;
            }
        }

        if let Some(id) = id {
            games.remove(id);
            println!("{:?}", data);
            write_data(&data)?;
            format!("{game}, c'est ciao !")
        } else {
            "J'comprends que tu veuilles pas y jouer, il était même pas dans ta liste !".to_string()
        }
    } else {
        "Commence déjà par ajouter des jeux en fait :exploding_head:".to_string()
    };

    ctx.say(msg).await?;
    Ok(())
}

/// Afficher tous les jeux enregistrés pour un utilisateur.
/// 
/// Cet commande supporte l'accès délégué. Si le paramètre `player` n'est pas précisé,
/// la commande est appliquée au client.
#[poise::command(slash_command)]
async fn list_games(
    ctx: Context<'_>, 
    #[description = "L'utilisateur client, par défaut celui à l'origine de cette requête"] player: Option<User>
) -> Output {
    let data = get_data()?;
    let player = player.as_ref().unwrap_or(ctx.author());
    
    let msg = if let Some(games) = data.get(player.name.as_str()) {
        if games.is_empty() {
            GIF.to_string()
        } else {
            vec_to_list(games)
        }
    } else {
        GIF.to_string()
    };

    ctx.say(format!("Jeux enregistrés pour {} :\n{msg}", player.mention())).await?;
    Ok(())
}

fn get_message(data: &Data, ctx: Context<'_>) -> String {
    let mut rng = rand::rng();

    if let Some(games) = data.get(&ctx.author().name) {
        if games.is_empty() {
            "Par pitié ajoute un jeu à ta liste".to_string()
        } else {
            format!(
                "Aujourd'hui tu as envie de jouer à {}", 
                games[rng.next_u32() as usize % games.len()]
            )
        }
    } else {
        "Tu es inconnu au bataillon".to_string()
    }
}