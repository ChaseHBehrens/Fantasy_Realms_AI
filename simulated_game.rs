use fantasy_realms_unofficial_api::{DrawCard, Game, Player};
use fantasy_realms_unofficial_api::hand::{Hand, Turn};
use fantasy_realms_unofficial_api::deck::{Card};
use fantasy_realms_unofficial_api::card_collection::{CardCollection};
use fantasy_realms_ai::FantasyRealmsBot;
use super::PlayerType;

/// Runs a simulated game.
/// # Arguments
/// * `player_types` - A `Vec<PlayerType>` representing a all players in the game. 
/// # Errors
/// This function returns an `Err(String)` if:
/// * Any of the players are human.
/// * Turn validation fails for a generated turn.
/// # Returns
/// A `Result<Self, String>` which is:
/// * `Ok(Game)` if the game is successfully run.
/// * `Err(String)` containing an error message if validation fails.
pub fn run_simulated_game(player_types: Vec<PlayerType>) -> Result<(), String> {
    let mut deck = !CardCollection::new();
    let mut players: Vec<Player> = Vec::new();
    let mut bots: Vec<Box<dyn FantasyRealmsBot>> = Vec::new();
    for (i, player) in player_types.into_iter().enumerate() {
        match player {
            PlayerType::Human (_) => {
                return Err("Human found in simulated game players".to_string());
            }
            PlayerType::Bot (bot) => {
                bots.push(bot);
            }
        }
        let name: String = bots[i as usize].name();
        let hand: Hand = deck.draw_hand();
        players.push(Player::new(name, hand));
    }
    let mut game = Game::new(players).unwrap();
    while !game.over {
        handle_bot_turn(&mut*bots[game.current_turn], &mut game)?;
        println!("discard pile size = {}", game.discard_pile.len());
    }
    display_final_scores(game);
    Ok(())
}

/// A helper function for `run_simpulated_game`.
/// Interfaces with a bot to generate a turn and plays the turn generated.
/// # Arguments 
/// * `bot` - A `&mut dyn FantasyRealmsBot` representing the bot whos turn it is to play. 
/// * `game` - A `&mut Game` representing the game being played. 
/// # Side Effects
/// * **`Game`**:
///    * plays a turn of the game.
/// # Errors
/// This function returns an `Err(String)` if:
/// * The turn generated is not valid. 
/// # Returns
/// A `Result<(), String>` which is:
/// * `Ok(())` if the turn is prossesed sucessfully.
/// * `Err(String)` containing an error message if the turn validation failed.
fn handle_bot_turn(bot: &mut dyn FantasyRealmsBot, game: &mut Game) -> Result<(), String> {
    let opponent_cards = get_opponent_known_cards(game);
    let min_turns_remaining = (10 - (game.discard_pile.len() as u8)) / (game.players.len() as u8);
    let draw_card: DrawCard = bot
        .generate_draw(
            &game.players[game.current_turn].hand,
            &game.discard_pile,
            &opponent_cards,
            min_turns_remaining,
    );
    let draw = match draw_card {
        DrawCard::Discard(card) => card,
        _ => game.deck.get_random_card().unwrap(),
    };
    let discard: Card = bot
        .generate_discard(
            &game.players[game.current_turn].hand,
            &game.discard_pile,
            &opponent_cards,
            min_turns_remaining,
            &draw,
    );
    let turn = Turn::new(draw, discard);
    display_bot_turn_info(bot, &turn);
    game.play_turn(turn)
}

/// A helper function for `handle_bot_turn`.
/// Gets the known cards for all opponents.
/// # Arguments
/// * `game` - A `Game` representing the current game being played
fn get_opponent_known_cards(game: &Game) -> Vec<CardCollection> {
    game.players
        .iter()
        .enumerate()
        .filter_map(|(i, player)| {
            if i != game.current_turn {
                Some(player.cards_known_to_opponents)
            } else {
                None
            }
        })
        .collect()
}

/// A helper function for `handle_bot_turn`.
/// Displays turn inforomation to the screen.
/// # Arguments
/// * `bot` - A `&dyn FantasyRealmsBot` Fantacy Realms bot.
/// * `turn` - A `&Turn` containing the turn information to be displayed.
fn display_bot_turn_info(bot: &dyn FantasyRealmsBot, turn: &Turn) {
    let name = bot.name();
    println!("{}'s turn.", name.trim());
    println!("{} draws {}.", name.trim(), turn.draw);
    println!("{} discards {}.",name.trim(), turn.discard);
}

/// A helper function for `run_simpulated_game`.
/// Displays the final scores for each player in a game.
/// # Arguments
/// * `game` - A `Game` containing the hand information of all players.
fn display_final_scores(game: Game) {
    for player in game.players.iter() {
        let name: String = player.clone().name;
        let score: i16 = player.hand.score(&game.discard_pile);
        println!("{} scored {score} points.", name.trim());
        println!("{} had the hand: ", name.trim());
        for card in player.hand {
            print!("{}, ", card.to_string())
        }
        println!();
    }
}