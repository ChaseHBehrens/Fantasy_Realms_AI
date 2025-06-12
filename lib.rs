pub mod terminal_interface;
pub mod physical_game;
pub mod simulated_game;

use fantasy_realms_ai::FantasyRealmsBot;

/// Represents the type of a player.
/// # Variants
/// * 'Human' - Contains a `String` representing a name of a human player.
/// * `Bot` - Contains a `Box<dyn FantasyRealmsBot>` a Fantasy Realms bot.
pub enum PlayerType {
    Human (String),
    Bot (Box<dyn FantasyRealmsBot>),
}