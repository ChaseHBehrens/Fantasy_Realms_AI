use fantasy_realms_ui::terminal_interface::*;
use fantasy_realms_ui::PlayerType;
use fantasy_realms_ui::physical_game::run_physical_game;
use fantasy_realms_ui::simulated_game::run_simulated_game;

/// Gets user input to create a new game.
/// Then runs the type of game based on input recieved from the user.
fn new_game() {
    println!("Enter the number of players, (3 to 6): ");
    let number_of_players: u8 = get_int_input(3, 6);
    let mut player_types: Vec<PlayerType> = Vec::new();
    for i in 1..number_of_players + 1 {
        println!("Enter the type of player {i}, (Human, Bot):");
        let player_type: PlayerType = get_player_type_input();
        player_types.push(player_type);
    }
    if player_types.iter().any(|player_type| matches!(player_type, PlayerType::Human(_))) {
        if let Err(e) = run_physical_game(player_types) {
            eprintln!("Invalid game state: {}", e);
        }
    } else {
        println!("running simulated game.");
        if let Err(e) = run_simulated_game(player_types) {
            eprintln!("Invalid game state: {}", e);
        }
    }
}

fn main() {
    let mut running: bool = true;
    while running {
        new_game();
        println!("Would you like to start a new game, (yes, no):");
        running = get_new_game_input();
    }
}