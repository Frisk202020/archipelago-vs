use serde::{Deserialize, Serialize};
use serenity::all::User;

use crate::commands::session::data::DiscordData;

#[derive(Serialize, Deserialize)]
struct PlayerData {
    id: u64, name: String, display_name: String
} impl From<User> for PlayerData {
    fn from(value: User) -> Self {
        let display_name = value.display_name().to_string();
        Self { id: value.id.get(), name: value.name, display_name }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TeamSetup {
    team_size: usize,
    games: Vec<String>,
    teams: Vec<Vec<PlayerData>>,
    pub channels: Vec<DiscordData>,
    pub roles: Vec<DiscordData>
} impl TeamSetup {
    pub fn new(team_size: usize) -> Self {
        Self { team_size, games: vec![], teams: vec![], channels: vec![], roles: vec![] }
    }

    pub fn check(&self) -> bool {
        self.games.len() == self.team_size &&
        self.teams.iter().all(|x| x.len() == self.team_size)
    }

    pub fn n_teams(&self) -> usize { self.teams.len() }
    pub fn team_ids(&self) -> impl Iterator<Item = impl Iterator<Item = u64>> { 
        self.teams.iter().map(|x| x.iter().map(|x| x.id))
    }

    pub fn add_game(&mut self, game: String) -> bool {
        if self.team_size == self.games.len() {
            false
        } else { self.games.push(game); true }
    }
    pub fn remove_last_game(&mut self) -> Option<String> {
        self.games.pop()
    }

    pub fn add_player(&mut self, player: User) -> (usize, Option<String>) {
        for (t_id, team) in self.teams.iter_mut().enumerate() {
            if team.len() < self.team_size {
                let g_id = team.len();
                team.push(PlayerData::from(player));
                return (t_id, self.games.get(g_id).map(|x| x.to_string()))
            }
        }

        let t_id = self.teams.len();
        self.teams.push(vec![PlayerData::from(player)]);
        (t_id, self.games.get(0).map(|x| x.to_string()))
    }
    pub fn remove_last_player(&mut self) -> Option<(usize, String)> {
        for team in self.teams.iter_mut().rev() {
            if team.len() > 0 { return team.pop().map(|data| (self.team_size - team.len(), data.name)) }
        }

        None
    }
} impl ToString for TeamSetup {
    fn to_string(&self) -> String {
        let mut out = vec!["__Jeux__".to_string()];
        
        for (i, game) in self.games.iter().enumerate() {
            let n_players = self.teams.len();
            let mut team = Vec::with_capacity(n_players);
            for j in 0..n_players {
                team.push(self.teams[j][i].display_name.as_str());
            }

            out.push(format!("{game}: {}", team.join(", ")));
        }

        out.push("\n__Équipes__".to_string());
        for (i, team) in self.teams.iter().enumerate() {
            out.push(format!("**Équipe** {}: {}", i+1, team.iter().map(|x| x.display_name.as_str()).collect::<Vec<_>>().join(", ")));
        }

        return out.join("\n");
    }
}