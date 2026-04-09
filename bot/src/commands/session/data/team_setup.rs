use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TeamSetup {
    team_size: usize,
    games: Vec<String>,
    teams: Vec<Vec<String>>
} impl TeamSetup {
    pub fn new(team_size: usize) -> Self {
        Self { team_size, games: vec![], teams: vec![] }
    }

    pub fn check(&self) -> bool {
        self.games.len() == self.team_size &&
        self.teams.iter().all(|x| x.len() == self.team_size)
    }

    pub fn team_size(&self) -> usize { self.team_size }

    pub fn add_game(&mut self, game: String) -> bool {
        if self.team_size == self.games.len() {
            false
        } else { self.games.push(game); true }
    }
    pub fn remove_last_game(&mut self) -> Option<String> {
        self.games.pop()
    }

    pub fn add_player(&mut self, player: String) -> Option<(usize, Option<String>)> {
        for (t_id, team) in self.teams.iter_mut().enumerate() {
            if team.len() < self.team_size {
                let g_id = team.len();
                team.push(player);
                return Some((t_id, team.get(g_id).map(|x| x.to_string())))
            }
        }

        None
    }
    pub fn remove_last_player(&mut self) -> Option<(usize, String)> {
        for team in self.teams.iter_mut().rev() {
            if team.len() > 0 { return team.pop().map(|x| (self.team_size - team.len(), x)) }
        }

        None
    }
} impl ToString for TeamSetup {
    fn to_string(&self) -> String {
        let n_games= self.games.len();
        let mut out = Vec::with_capacity(n_games);
        for i in 0..n_games {
            let n_players = self.teams.len();
            let mut x = Vec::with_capacity(n_players);
            for j in 0..n_players {
                x[j] = self.teams[j][i].as_str();
            }

            out[i] = x.join(" | ");
        }

        out.join("\n")
    }
}