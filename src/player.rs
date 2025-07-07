use crate::error::{ServiceError, ServiceResult};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct Player {
    pub id: u32,
    pub name: String,
    pub last_name: String,
}

impl Player {
    pub fn new(id: u32, name: String, last_name: String) -> Self {
        Self { id, name, last_name }
    }

    pub fn update_from(&mut self, other: &Player) {
        self.name = other.name.clone();
        self.last_name = other.last_name.clone();
    }
}

pub type PlayerRef = Arc<Mutex<Player>>;

pub trait PlayerService: Send + Sync {
    fn create_player(&mut self, player: Player) -> ServiceResult<PlayerRef>;
    fn get_player(&self, id: u32) -> ServiceResult<PlayerRef>;
    fn update_player(&mut self, player: Player) -> ServiceResult<PlayerRef>;
    fn delete_player(&mut self, id: u32) -> ServiceResult<()>;
    fn count(&self) -> usize;
}

pub struct PlayerServiceImpl {
    players: Vec<PlayerRef>,
}

impl PlayerServiceImpl {
    pub fn new() -> Self {
        Self { 
            players: Vec::new() 
        }
    }

    fn find_player_by_id(&self, id: u32) -> Option<(usize, PlayerRef)> {
        self.players.iter()
            .enumerate()
            .find(|(_, p)| p.lock().unwrap().id == id)
            .map(|(i, p)| (i, p.clone()))
    }
}

impl PlayerService for PlayerServiceImpl {
    fn create_player(&mut self, player: Player) -> ServiceResult<PlayerRef> {
        if let Some(_) = self.find_player_by_id(player.id) {
            return Err(ServiceError::AlreadyExists(
                format!("Player with id {} already exists", player.id)
            ));
        }

        let player_ref = Arc::new(Mutex::new(player));
        self.players.push(player_ref.clone());
        Ok(player_ref)
    }
    
    fn get_player(&self, id: u32) -> ServiceResult<PlayerRef> {
        self.find_player_by_id(id)
            .map(|(_, player)| player)
            .ok_or_else(|| ServiceError::NotFound(
                format!("Player with id {} not found", id)
            ))
    }

    fn update_player(&mut self, player: Player) -> ServiceResult<PlayerRef> {
        let (_, player_ref) = self.find_player_by_id(player.id)
            .ok_or_else(|| ServiceError::NotFound(
                format!("Player with id {} not found", player.id)
            ))?;

        {
            let mut existing_player = player_ref.lock().unwrap();
            existing_player.update_from(&player);
        }
        
        Ok(player_ref)
    }

    fn delete_player(&mut self, id: u32) -> ServiceResult<()> {
        let (index, _) = self.find_player_by_id(id)
            .ok_or_else(|| ServiceError::NotFound(
                format!("Player with id {} not found", id)
            ))?;
            
        self.players.remove(index);
        Ok(())
    }

    fn count(&self) -> usize {
        self.players.len()
    }
}