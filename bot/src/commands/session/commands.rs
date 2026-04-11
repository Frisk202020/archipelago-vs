use chrono::Utc;
use serenity::all::{
    ChannelType, CreateChannel, EditChannel, EditRole, Mentionable, PermissionOverwrite, PermissionOverwriteType, Permissions, RoleId, User
};

use crate::{
    commands::session::{data::{Data, Status}, interaction::{NO_REPLY, SessionInteraction}}, 
    util::{Context, Output, vec_to_list}
};

const SESSION_DATA_MISSING_MSG: &'static str = "Commence peut-être par lancer une session hein.";
const GIF: &'static str = "https://giphy.com/gifs/football-lost-hashtagunited-jU9OCvBiO1besabUKU";

#[poise::command(slash_command)]
pub async fn init_session(ctx: Context<'_>, team_size: usize) -> Output {
    let data = Data::get();
    if data.is_err() || data.unwrap().is_closed() {
        Data::write_new(Some(team_size))?;
        ctx.say("Session initiée. Pour plus de détail sur la construction de la session, lancez `help_session`").await?;
        return Ok(());
    };

    let res = SessionInteraction::handle_interaction(
        ctx,
        "Pour info une session en cours ou en création existe déjà selon mes données, et procéder va tout foutre à la poubelle. T'es sur de ton coup ?",
        vec![SessionInteraction::StartAccept, SessionInteraction::StartDeny]
    ).await?;

    if let Some(res) = res {
        match res {
            SessionInteraction::StartAccept => {
                Data::write_new(Some(team_size))?;
                ctx.say("C'est parti, maintenant faut me dire les jeux et les équipes.").await?;
            },
            _ => {},
        }
    } else {
        ctx.say(NO_REPLY).await?;
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn add_session_game(ctx: Context<'_>, game: String) -> Output {
    let mut data = Data::get()?;
    if !data.is_building() {
        ctx.say("La session n'est pas en état de construction").await?;
        return Ok(());
    }

    if !data.is_building() {
        ctx.say("Commence déjà par initier une nouvelle session").await?;
        return Ok(());
    }

    let reply = 
        if data.add_game(game.clone())? { format!("Jeu ajouté : {game}") }
        else { "Tous les jeux ont déjà été spécifiés".to_string() };

    ctx.say(reply).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn add_session_player(ctx: Context<'_>, player: User) -> Output {
    let mut data = Data::get()?;
    if !data.is_building() {
        ctx.say("La session n'est pas en état de construction").await?;
        return Ok(());
    }

    if !data.is_building() {
        ctx.say("Commence déjà par initier une nouvelle session").await?;
        return Ok(());
    }

    let (t_id, game) = data.add_player(player)?;
    ctx.say(format!("Joueur ajouté à la team {} sur: {}", t_id + 1, game.unwrap_or("jeu non défini".to_string()))).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn remove_last_session_game(ctx: Context<'_>) -> Output {
    let mut data = Data::get()?;
    if !data.is_building() {
        ctx.say("La session n'est pas en état de construction").await?;
        return Ok(());
    }

    if let Some(game) = data.remove_last_game()? {
        ctx.say(format!("Le jeu {game} a été retiré")).await?;
    } else { ctx.say("Plus aucun jeu à retirer").await?; }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn remove_last_player(ctx: Context<'_>) -> Output {
    let mut data = Data::get()?;
    if !data.is_building() {
        ctx.say("La session n'est pas en état de construction").await?;
        return Ok(());
    }

    if let Some((players_left, player)) = data.remove_last_player()? {
        ctx.say(format!("{player} retiré. Il reste maintenant {players_left} joueurs à ajouter dans cette équipe")).await?;
    } else {
        ctx.say("Plus aucun joueur à retirer").await?;
    }
    Ok(())
} 

/// Lancer le chrono pour un nouveau versus.
/// 
/// Cela supprime les données relatives à une précédente session.
#[poise::command(slash_command)]
pub async fn start_session(ctx: Context<'_>) -> Output {
    let data = Data::get();
    if data.is_err() {
        Data::write_new(None)?;
        ctx.say("Session initiée, sans aucune équipe").await?;
        return Ok(());
    } let data = data.unwrap();

    match data.status() {
        Status::Active => {
            let res = SessionInteraction::handle_interaction(
                ctx,
                "Une session active est en cours, on s'en fout ?",
                vec![SessionInteraction::StartAccept, SessionInteraction::StartDeny]
            ).await?;

            if let Some(res) = res {
                match res {
                    SessionInteraction::StartAccept => {
                        Data::write_new(None)?;
                        ctx.say("Hop là c'est parti").await?;
                    },
                    _ => {},
                }
            } else {
                ctx.say(NO_REPLY).await?;
            }
        },
        Status::Building => 'build : {
            if !data.check_setup() {
                ctx.say("Configuration invalide, veuillez en créer une nouvelle").await?;
                break 'build;
            }

            let res = SessionInteraction::handle_interaction(
                ctx, 
                format!("Voici la config actuelle, confirme que c'est bon et on y va.\n{}", data.setup_to_string()).as_str(),
                vec![SessionInteraction::ConfirmBuild, SessionInteraction::DenyBuild]
            ).await?;

            if let Some(res) = res {
                match res {
                    SessionInteraction::DenyBuild => return Ok(()),
                    _ => {}
                }
            } else { ctx.say(NO_REPLY).await?; return Ok(()); }

            if let Some(guild) = ctx.partial_guild().await {
                if let Some(bot_role) = guild.role_by_name("Bot") {
                    let category = guild.create_channel(
                        ctx.http(), 
                        CreateChannel::new("Versus").kind(ChannelType::Category)
                    ).await?;

                    let permissions = 
                        Permissions::VIEW_CHANNEL |
                        Permissions::READ_MESSAGE_HISTORY |
                        Permissions::SEND_MESSAGES;

                    for (index, team) in data.team_ids().enumerate() {
                        let index = index + 1;
                        let mut channel = guild.create_channel(
                            ctx.http(), 
                            CreateChannel::new(format!("team-{index}")).category(category.id)
                        ).await?;

                        let team_role = guild.create_role(
                            ctx.http(),
                            EditRole::new().name(format!("Team {index}"))
                        ).await?;

                        for id in team {
                            let member = guild.member(ctx.http(), id).await?;
                            member.add_role(ctx.http(), team_role.id).await?;
                        }

                        channel.edit(ctx.http(), EditChannel::new().permissions(vec![
                            PermissionOverwrite {
                                allow: permissions,
                                deny: Permissions::empty(),
                                kind: PermissionOverwriteType::Role(team_role.id)
                            }, PermissionOverwrite {
                                allow: permissions,
                                deny: Permissions::empty(),
                                kind: PermissionOverwriteType::Role(bot_role.id)
                            }, PermissionOverwrite {
                                allow: Permissions::empty(),
                                deny: permissions,
                                kind: PermissionOverwriteType::Role(RoleId::new(guild.id.get()))
                            }
                        ])).await?;
                    }

                    ctx.say("Session initiée avec succès !").await?;
                } else {
                    ctx.say("Je ne trouve pas mon rôle -- configuration interompue").await?;
                } 
            } else {
                ctx.say("Je ne trouve pas le server -- configuration interompue").await?;
            }
        },
        _ => {
            Data::write_new(None)?;
            ctx.say("Session initiée, sans aucune équipe").await?;
        }
    }

    Ok(())
}

/// Marquer la victoire d'un joueur (sauvegarder le temps de jeu actuel).
#[poise::command(slash_command)]
pub async fn finish(
    ctx: Context<'_>, 
    #[description = "Le joueur qui a terminé, par défaut le client de la commande"] player: Option<User>) -> Output {
    if let Ok(mut data) = Data::get() { 
        let user = player.as_ref().unwrap_or(ctx.author());
        
        if let Some(end) = data.finish(&user.name, false)? {
            ctx.say(format!("GG {}, tu as fini en {} !", user.mention(), data.display_elapsed(&end))).await?;
        } else {
            let res = SessionInteraction::handle_interaction(
                ctx, 
                &format!("Mais... {} avait déjà finit, on remplace par ce nouveau temps ?", user.mention()),
                vec![SessionInteraction::FinishAccept, SessionInteraction::FinishDeny]
            ).await?;

            if let Some(res) = res {
                match res {
                    SessionInteraction::FinishAccept => {
                        let end = data.finish(&user.name, true)?.unwrap();
                        ctx.say(format!("GG {}, tu as fini en {} !", user.mention(), data.display_elapsed(&end))).await?;
                    },
                    _ => {},
                }
            } else {
                ctx.say(NO_REPLY).await?;
            }
        }
    } else {
        ctx.say(SESSION_DATA_MISSING_MSG).await?;
    }
    
    Ok(())
}

/// Afficher le temps de partie enregistré pour un joueur.
#[poise::command(slash_command)]
pub async fn get_time(
    ctx: Context<'_>, 
    #[description = "Le joueur qui a terminé, par défaut le client de la commande"] player: Option<User>
) -> Output {
    if let Ok(data) = Data::get() {
        let target = player.as_ref().unwrap_or(ctx.author());
        if let Some(end) = data.get_time(&target.name) {
            ctx.say(format!("{} a fini en {} !", target.mention(), data.display_elapsed(end))).await?;
        } else {
            ctx.say(format!("{} n'a pas finit.", target.mention())).await?;
        }
    } else {
        ctx.say(SESSION_DATA_MISSING_MSG).await?;
    }

    Ok(())
}

/// Ajouter un split sur la run d'un joueur.
#[poise::command(slash_command)]
pub async fn add_tms(
    ctx: Context<'_>, 
    #[description = "Description du split"] label: String, 
    #[description = "Le joueur qui a terminé, par défaut le client de la commande"] player: Option<User>
) -> Output {
    if let Ok(mut data) = Data::get() {
        let name = player.as_ref().map(|x| &x.name).unwrap_or(&ctx.author().name);
        data.add_tms(name.as_str(), label.as_str())?;
        ctx.say("C'est noté.").await?;
    } else {
        ctx.say(SESSION_DATA_MISSING_MSG).await?;
    }

    Ok(())
}

/// Afficher la liste de tous les splits enregistrés.
#[poise::command(slash_command)]
pub async fn list_tms(
    ctx: Context<'_>, 
    #[description = "Le joueur qui a terminé, par défaut le client de la commande"] player: Option<User>
) -> Output {
    let res = if let Ok(data) = Data::get() {
        let user = player.as_ref().unwrap_or(ctx.author());
        if let Some(tms) = data.get_tms(&user.name) {
            &vec_to_list(tms)
        } else {
            GIF
        }
    } else {
        SESSION_DATA_MISSING_MSG
    };

    ctx.say(res).await?;
    Ok(())
}

/// Afficher la durée de la session en cours.
#[poise::command(slash_command)]
pub async fn session_duration(ctx: Context<'_>) -> Output {
    let res = if let Ok(data) = Data::get() {
        &data.display_elapsed(&Utc::now().naive_utc())
    } else { SESSION_DATA_MISSING_MSG };

    ctx.say(res).await?;
    Ok(())
}