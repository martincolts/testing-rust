use crate::player::{Player, PlayerService, PlayerRef};
use crate::error::{ServiceError, ServiceResult};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct Team {
    pub id: u32,
    pub name: String,
    pub players: Vec<PlayerRef>,
}

impl Team {
    pub fn new(id: u32, name: String) -> Self {
        Self {
            id,
            name,
            players: Vec::new(),
        }
    }

    pub fn add_player(&mut self, player: PlayerRef) -> ServiceResult<()> {
        let player_id = player.lock().unwrap().id;
        if self.players.iter().any(|p| p.lock().unwrap().id == player_id) {
            return Err(ServiceError::AlreadyExists(
                format!("Player {} is already in team {}", player_id, self.id)
            ));
        }
        self.players.push(player);
        Ok(())
    }

    pub fn player_count(&self) -> usize {
        self.players.len()
    }
}

pub trait TeamService: Send + Sync {
    fn create_team(&mut self, team: Team) -> ServiceResult<Team>;
    fn get_team(&self, id: u32) -> ServiceResult<&Team>;
    fn insert_player_to_team(&mut self, team_id: u32, player_id: u32) -> ServiceResult<()>;
    fn remove_player_from_team(&mut self, team_id: u32, player_id: u32) -> ServiceResult<()>;
}

pub struct TeamServiceImpl {
    teams: Vec<Team>,   
    player_service: Arc<Mutex<dyn PlayerService>>,
}

impl TeamServiceImpl {
    pub fn new(player_service: Arc<Mutex<dyn PlayerService>>) -> Self {
        Self {
            teams: Vec::new(),
            player_service,
        }
    }

    fn find_team_by_id(&self, id: u32) -> Option<(usize, &Team)> {
        self.teams.iter()
            .enumerate()
            .find(|(_, t)| t.id == id)
    }

    fn find_team_by_id_mut(&mut self, id: u32) -> Option<&mut Team> {
        self.teams.iter_mut()
            .find(|t| t.id == id)
    }
}

impl TeamService for TeamServiceImpl {
    fn create_team(&mut self, team: Team) -> ServiceResult<Team> {
        if let Some(_) = self.find_team_by_id(team.id) {
            return Err(ServiceError::AlreadyExists(
                format!("Team with id {} already exists", team.id)
            ));
        }
        self.teams.push(team.clone());
        Ok(team)
    } 
    
    fn get_team(&self, id: u32) -> ServiceResult<&Team> {
        self.find_team_by_id(id)
            .map(|(_, team)| team)
            .ok_or_else(|| ServiceError::NotFound(
                format!("Team with id {} not found", id)
            ))
    }

    fn insert_player_to_team(&mut self, team_id: u32, player_id: u32) -> ServiceResult<()> {
        // Get the player reference from player service
        let player = {
            let player_service = self.player_service.lock().unwrap();
            player_service.get_player(player_id)?
        };

        // Find the team and add the player
        let team = self.find_team_by_id_mut(team_id)
            .ok_or_else(|| ServiceError::NotFound(
                format!("Team with id {} not found", team_id)
            ))?;

        team.add_player(player)
    }

    fn remove_player_from_team(&mut self, team_id: u32, player_id: u32) -> ServiceResult<()> {
        let team = self.find_team_by_id_mut(team_id)
            .ok_or_else(|| ServiceError::NotFound(
                format!("Team with id {} not found", team_id)
            ))?;

        let player_pos = team.players.iter()
            .position(|p| p.lock().unwrap().id == player_id)
            .ok_or_else(|| ServiceError::NotFound(
                format!("Player {} not found in team {}", player_id, team_id)
            ))?;

        team.players.remove(player_pos);
        Ok(())
    }
}