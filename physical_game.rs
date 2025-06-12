use fantasy_realms_unofficial_api::{DrawCard, Player, PartialGame, PartialGamePlayer, PartialPlayer, PartialGameTurn};
use fantasy_realms_unofficial_api::hand::{Hand, Turn};
use fantasy_realms_unofficial_api::deck::Card;
use fantasy_realms_unofficial_api::card_collection::{CardCollection};
use fantasy_realms_ai::FantasyRealmsBot;
use super::terminal_interface::*;
use super::PlayerType;

/// Runs a physical game.
/// # Arguments 
/// * `player_types` - A `Vec<PlayerType>` representing a all players in the game. 
/// # Errors
/// This function returns an `Err(String)` if:
/// * Turn validation fails for a generated turn.
/// # Returns
/// A `Result<Self, String>` which is:
/// * `Ok(Game)` if the game is successfully run.
/// * `Err(String)` containing an error message if validation fails.
pub fn run_physical_game(mut player_types: Vec<PlayerType>) -> Result<(), String> {
    let players = create_players(&player_types);
    let mut game = PartialGame::new(players).unwrap();
    while !game.over {
        match &mut player_types[game.current_turn] {
            PlayerType::Human (name) => handle_human_turn(name, &mut game)?,
            PlayerType::Bot (bot) => handle_bot_turn(&mut**bot, &mut game)?,
        }
    }
    display_final_scores(game);
    Ok(())
}

/// Helper function for `runs_physical_game`.
/// Creates a `PartialGamePlayer` for each player in the game
/// # Arguments 
/// * `player_types` - A `&Vec<PlayerType>` representing a all players in the game. 
/// # Returns
/// A `Vec<PartialGamePlayer>` representing a all players in the game.
fn create_players(player_types: &Vec<PlayerType>) -> Vec<PartialGamePlayer> {
    player_types
        .iter()
        .map(|player_type| match player_type {
            PlayerType::Human(name) => {
                PartialGamePlayer::Human(PartialPlayer::new(name.clone()))
            }
            PlayerType::Bot(bot) => {
                let name = bot.name();
                println!("Enter the names of the cards in {}'s hand:", name.trim());
                let hand = get_hand_input();
                PartialGamePlayer::Bot(Player::new(name, hand))
            }
        })
        .collect()
}

/// Helper function for `runs_physical_game`.
/// Interfaces with a bot to generate a turn and plays the turn generated.
/// # Arguments 
/// * `name` - A `&String` representing the bot whos turn it is to play. 
/// * `game` - A `&mut PartialGame` representing the game being played. 
/// # Side Effects
/// * **`PartialGame`**:
///    * plays a turn of the game.
/// # Errors
/// This function returns an `Err(String)` if:
/// * The turn generated is not valid. 
/// # Returns
/// A `Result<(), String>` which is:
/// * `Ok(())` if the turn is prossesed sucessfully.
/// * `Err(String)` containing an error message if the turn validation failed.
fn handle_human_turn(name: &String, game: &mut PartialGame) -> Result<(), String> {
    println!("{}'s turn.", name.trim());
    game.play_turn(PartialGameTurn::Human (get_turn_input()))
}

/// Helper function for `runs_physical_game`.
/// Interfaces with a bot to generate a turn and plays the turn generated.
/// # Arguments 
/// * `bot` - A `&mut dyn FantasyRealmsBot` representing the bot whos turn it is to play. 
/// * `game` - A `&mut PartialGame` representing the game being played. 
/// # Side Effects
/// * **`PartialGame`**:
///    * plays a turn of the game.
/// # Errors
/// This function returns an `Err(String)` if:
/// * The turn generated is not valid. 
/// # Returns
/// A `Result<(), String>` which is:
/// * `Ok(())` if the turn is prossesed sucessfully.
/// * `Err(String)` containing an error message if the turn validation failed.
fn handle_bot_turn(bot: &mut dyn FantasyRealmsBot, game: &mut PartialGame) -> Result<(), String> {
    println!("{}'s turn.", bot.name().trim());
    let hand = get_bot_hand(&game.players[game.current_turn]);
    let opponent_cards = get_opponent_known_cards(game);
    let min_turns_remaining = (10 - (game.discard_pile.len() as u8)) / (game.players.len() as u8);
    let draw_card: DrawCard = bot
        .generate_draw(
            &hand,
            &game.discard_pile,
            &opponent_cards,
            min_turns_remaining,
    );
    let draw = match draw_card {
        DrawCard::Discard(card) => {
            println!("{} draws {}.", bot.name().trim(), card);
            card
        }
        _ => {
            println!("{} draws from the deck. Enter the name of the card drawn:", bot.name().trim());
            get_card_input()
        }
    };
    let discard: Card = bot
        .generate_discard(
            &hand,
            &game.discard_pile,
            &opponent_cards,
            min_turns_remaining,
            &draw,
    );
    println!("{} discards {}.", bot.name().trim(), discard);
    let turn = Turn::new(draw, discard);
    game.play_turn(PartialGameTurn::Bot (turn))
}

/// Helper function for `handle_bot_turn`.
/// # Arguments 
/// * `player` - A `&PartialGamePlayer` representing a bot.
/// # Panics
/// * A human player is recieved. 
/// # Returns
/// A `&Hand` representing the hand of the bot.
fn get_bot_hand(player: &PartialGamePlayer) -> &Hand {
    match player {
        PartialGamePlayer::Bot(player) => &player.hand,
        _ => panic!("Expected bot, got human"),
    }
}

/// Helper function for `handle_bot_turn`.
/// # Arguments 
/// * `game` - A `&PartialGame` representing the game being played.
/// # Returns
/// A `Vec<CardCollection>` containing known cards from each opponents hands.  
fn get_opponent_known_cards(game: &PartialGame) -> Vec<CardCollection> {
    game.players
        .iter()
        .enumerate()
        .filter_map(|(i, player)| {
            if i != game.current_turn {
                Some(match player {
                    PartialGamePlayer::Bot(p) => p.cards_known_to_opponents,
                    PartialGamePlayer::Human(p) => p.cards_known_to_opponents,
                })
            } else {
                None
            }
        })
        .collect()
}

/// A helper function for `run_physical_game`.
/// Displays the final scores for each player in a game.
/// # Arguments
/// * `game` - A `PartialGame` containing the hand information of all players.
fn display_final_scores(game: PartialGame) {
    println!("The game has finished.");
    let players: Vec<Player> = game.players
        .into_iter()
        .map(|player| match player {
            PartialGamePlayer::Human (partial_player) => {
                println!("Enter cards in {}'s hand:", partial_player.name.trim());
                Player::new(partial_player.name, get_hand_input())
            }
            PartialGamePlayer::Bot(bot_player) => bot_player,
        })
        .collect();
    for player in &players {
        let score = player.hand.score(&game.discard_pile);
        println!("{} scored {score} points.", player.name);
    }
}