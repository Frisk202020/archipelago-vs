use serenity::all::{
    ChannelType, CreateChannel, EditChannel, EditRole, PermissionOverwrite, 
    PermissionOverwriteType, Permissions, RoleId, User
};
use crate::{
    commands::session::{data::{Data, Status}, 
    interaction::{NO_REPLY, SessionInteraction}}, 
    util::{Context, Output}
};

#[poise::command(slash_command, subcommands(
    "init", "add_game", "add_player", "remove_last_game", 
    "remove_last_player", "start", "help"
))]
pub async fn builder(_ctx: Context<'_>) -> Output { Ok(()) }

/// Commencer la construction d'un versus, écrasant les données d'une session précédente
/// 
/// Les données relatives à une précédente session seront supprimées
#[poise::command(slash_command)]
async fn init(
    ctx: Context<'_>, 
    #[description = "Le nombre de joueurs dans chaque équipe (égal au nombre de jeux)"] team_size: usize
) -> Output {
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

/// Aide utilisateur pour construire une session
#[poise::command(slash_command)]
async fn help(ctx: Context<'_>) -> Output {
    ctx.say(format!("Pour commencer une configuration, exécutez `init_session [N]`. Cela initie une session d'équipes de **N** joueurs sur **N** jeux (chaque jeu et joué par un membre de chaque équipe). Ensuite, renseignez les jeux de la session (cela est utile pour la publication vers Google Sheets) : ajoutez les jeux un par un avec `add_session_game [game]`. Vous devez ajouter exactement **N** jeux.\n\nCela fait, préparez ensuite les équipes. Les joueurs sont ajoutés un par un avec `add_session_player [@someone]`, où les N premiers joueurs sont associés à l'équipe 1, les N suivants à l'équipe 2... A noter qu'il n'y a pas de limite sur le nombre d'équipes.\n**Attention** : ajoutez les joueurs dans le même ordre des jeux, c'est-à-dire que le joueur 1 d'une équipe doit être celui qui jouera au premier jeu, ect.\n\nPour finir, lancez `start_session`, vérifiez la configuration et lancez la partie.\nA noter que cette commande :\n- Lance le chrono de la session au moment où la commande se termine\n- Crée un rôle par équipe attribué aux joueurs respectifs\n- Crée un channel par équipe dans la catégorie **Versus**. Ces canaux sont restreint à leur équipe et permettent d'executer des commandes en toute discrétion (bon évidemment les admins, jouez le jeu)")).await?;
    Ok(())
}

/// Ajouter un jeu à la session en cours de construction
/// 
/// Il doit au final y avoir autant de jeux que la taille des équipes
#[poise::command(slash_command)]
async fn add_game(
    ctx: Context<'_>, 
    #[description = "Nom de jeu à ajouter"] game: String
) -> Output {
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

/// Ajouter un joueur à la session en cours de construction
#[poise::command(slash_command)]
async fn add_player(
    ctx: Context<'_>, 
    #[description = "Utilisateur à ajouter"] player: User
) -> Output {
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

/// Retirer le dernier jeu ajouté à la session en cours de construction
#[poise::command(slash_command)]
async fn remove_last_game(ctx: Context<'_>) -> Output {
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

/// Retirer le dernier joueur ajouté à la session en cours de construction
#[poise::command(slash_command)]
async fn remove_last_player(ctx: Context<'_>) -> Output {
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
async fn start(ctx: Context<'_>) -> Output {
    let data = Data::get();
    if data.is_err() {
        Data::write_new(None)?;
        ctx.say("Session initiée, sans aucune équipe").await?;
        return Ok(());
    } let mut data = data.unwrap();

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

                    let mut roles = Vec::with_capacity(data.n_teams());
                    let mut channels = Vec::with_capacity(data.n_teams() + 1);
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

                        channels.push(channel);
                        roles.push(team_role);
                    }

                    channels.push(category);
                    data.activate(roles, channels)?;
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