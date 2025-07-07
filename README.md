# rust_teams

This repository is a personal project for learning Rust. It implements a simple system for managing players and teams, focusing on Rust's ownership, concurrency, and error handling features.

## Overview

The project demonstrates how to:
- Create and manage `Player` and `Team` entities
- Use traits and trait objects for service abstraction
- Handle errors using custom error types
- Share state safely across threads using `Arc<Mutex<...>>`

## Modules

- **player.rs**: Defines the `Player` struct and the `PlayerService` trait for creating, updating, retrieving, and deleting players. Uses `Arc<Mutex<Player>>` for safe shared access.
- **team.rs**: Defines the `Team` struct and the `TeamService` trait for creating teams and managing player membership within teams. Integrates with the player service for player lookups.
- **error.rs**: Contains the `ServiceError` enum and `ServiceResult` type alias for consistent error handling across services.
- **main.rs**: Entry point. Demonstrates creating players and teams, updating player information, and handling errors.

## Example Usage

To run the project:

```sh
cargo run
```

This will execute the main function, which:
- Creates two players
- Creates a team
- Adds players to the team
- Updates a player's information and shows the update propagates to the team
- Demonstrates error handling (e.g., adding a duplicate player)

## Requirements
- Rust (edition 2024 or later)

## Learning Goals
- Practice Rust's module system, traits, and generics
- Explore safe concurrency with `Arc` and `Mutex`
- Implement custom error types and error handling patterns

---

Feel free to use or modify this project as a reference for your own Rust learning journey!
