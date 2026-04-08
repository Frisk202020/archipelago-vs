use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct TeamSetup {
    team_size: usize,
    games: Vec<String>,
    teams: Vec<Vec<String>>
} impl TeamSetup {
    pub(crate) fn new(team_size: usize) -> Self {
        Self { team_size, games: vec![], teams: vec![] }
    }

    pub(crate) fn team_size(&self) -> usize { self.team_size }

    pub(crate) fn set_games(&mut self, games: Vec<String>) -> bool {
        if self.team_size == games.len() {
            self.games = games;
            true
        } else { false }
    }

    pub(crate) fn add_team(&mut self, team: Vec<String>) -> bool {
        if self.team_size == team.len() {
            self.teams.push(team);
            true
        } else { false }
    }
}