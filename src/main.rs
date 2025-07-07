mod player;
mod team;
mod error;

use player::{Player, PlayerService, PlayerServiceImpl};
use team::{Team, TeamService, TeamServiceImpl};
use std::sync::{Arc, Mutex};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize services
    let player_service: Arc<Mutex<dyn PlayerService>> = Arc::new(Mutex::new(PlayerServiceImpl::new()));
    let mut team_service = TeamServiceImpl::new(player_service.clone());

    println!("Creating players and teams...");

    // Create and add players
    let john = Player::new(1, String::from("John"), String::from("Doe"));
    let martin = Player::new(2, String::from("Martin"), String::from("Lopez"));

    let john_ref = player_service.lock().unwrap().create_player(john)?;
    let martin_ref = player_service.lock().unwrap().create_player(martin)?;

    // Create and add team
    let team_a = Team::new(1, String::from("Team A"));
    team_service.create_team(team_a)?;

    println!("\nAdding players to team...");

    // Add players to team
    team_service.insert_player_to_team(1, 1)?;
    team_service.insert_player_to_team(1, 2)?;

    // Get and display team info
    let team = team_service.get_team(1)?;
    println!("\nTeam '{}' has {} players:", team.name, team.player_count());
    for player in team.players.iter() {
        let p = player.lock().unwrap();
        println!("- {} {}", p.name, p.last_name);
    }

    println!("\nUpdating John's information...");

    // Update John's information
    let updated_john = Player::new(1, String::from("John Updated"), String::from("Doe Updated"));
    let updated_ref = player_service.lock().unwrap().update_player(updated_john)?;

    // Verify the update propagated to the team
    let team = team_service.get_team(1)?;
    println!("\nTeam after update:");
    for player in team.players.iter() {
        let p = player.lock().unwrap();
        println!("- {} {}", p.name, p.last_name);
    }

    // Test error handling
    println!("\nTesting error handling:");
    match team_service.insert_player_to_team(1, 1) {
        Ok(_) => println!("Unexpected: Added duplicate player"),
        Err(e) => println!("Expected error: {}", e),
    }

    Ok(())
}
