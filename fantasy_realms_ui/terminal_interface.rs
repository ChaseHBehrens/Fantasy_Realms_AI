use std::io;
use fantasy_realms_unofficial_api::{DrawCard, deck::Card, hand::Hand, PartialTurn};
use fantasy_realms_ai::{FantasyRealmsBot, new_bot};
use super::PlayerType;

/// Gets an integer from the user via the terminal
/// # Arguments
/// * `min` - A `u8` representing the minimum value accepted.
/// * `max` - A `u8` representing the maximum value accepted.
/// # Returns
/// A `u8`
pub fn get_int_input(min: u8, max: u8,) -> u8 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line.");
        match input.trim().parse::<u8>() {
            Ok(number) if (min..=max).contains(&number) => {
                return number;
            }
            Ok(_) => {
                println!("Out of range. Enter a number between {} and {}:", min, max);
                continue;
            }
            Err(_) => {
                println!("Invalid input. Enter an intiger:");
                continue;
            }
        }
    }
}

/// Gets an String from the user via the terminal
/// # Returns
/// A `String`
pub fn get_string_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line.");
    input
}

/// Gets a card from the user via the terminal
/// # Returns
/// A `Card`
pub fn get_card_input() -> Card {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line.");
        match input.trim().parse::<Card>() {
            Ok(card) => {
                return card;
            }
            Err(_) => {
                println!("Invalid input. Enter the name of the card:");
                continue;
            }
        }
    }
}

/// Gets a DrawCard from the user via the terminal
/// # Returns
/// A `DrawCard` that can be a specific card or an uknown card.
fn get_draw_card_input() -> DrawCard {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line.");
        if matches!(input.trim().to_lowercase().as_str(), 
            "none" | 
            "nothing" | 
            "deck" |
            "hidden") {
            return DrawCard::Deck;
        }
        match input.trim().parse::<Card>() {
            Ok(card) => {
                return DrawCard::Discard (card);
            }
            Err(_) => {
                println!("Invalid input. Enter the name of the card:");
                continue;
            }
        }
    }
}

/// Gets a Hand from the user via the terminal
/// # Returns
/// A `Hand`
pub fn get_hand_input() -> Hand {
    let mut hand: Vec<Card> = Vec::new();
    while hand.len() < 7 {
        let new_card = get_card_input();
        if !hand.iter().any(|card| *card == new_card) {
            hand.push(new_card);
        }
    }
    Hand::new(hand.try_into().expect("Failed to convert to array."))
}

/// Gets a PartialTurn from the user via the terminal
/// # Returns
/// A `PartialTurn` 
pub fn get_turn_input() -> PartialTurn {
    println!("Enter the name of the card drawn:");
    let draw: DrawCard = get_draw_card_input();
    println!("Enter the name of the card discarded:");
    let discard: Card = get_card_input();
    PartialTurn::new(draw, discard)
}

/// Gets a bot from the user via the terminal
/// # Returns
/// A `Box<dyn FantacyRealmsBot>` 
pub fn get_bot_input() -> Box<dyn FantasyRealmsBot> {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line.");
        match new_bot(input) {
            Ok(ai) => {
                return ai;
            }
            Err(e) => {
                println!("{e}");
                continue;
            }
        }
    }
}

/// Gets a PlayerType from the user via the terminal
/// # Returns
/// A `PartialType` 
pub fn get_player_type_input() -> PlayerType {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line.");
        match input.trim().to_lowercase().as_str() {
            "human" | "person" => {
                println!("Enter the name of the player:");
                return PlayerType::Human (get_string_input());
            }
            "bot" | "ai" | "robot" | "computer" => {
                println!("Enter the name of the bot:");
                return PlayerType::Bot (get_bot_input());
            }
            _ => {
                println!("Invalid input. Enter either Human or Bot:");
                continue;
            }
        }
    }
}

/// Gets a String from the user via the terminal
/// # Returns
/// A `bool` representing weaher or not to start a new game. 
pub fn get_new_game_input() -> bool {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line.");
        match input.trim().to_lowercase().as_str() {
            "yes" | "yeah" | "ok" | "sure" => {
                return true;
            }
            "no" => {
                return false;
            }
            _ => {
                continue;
            }
        }
    }
}