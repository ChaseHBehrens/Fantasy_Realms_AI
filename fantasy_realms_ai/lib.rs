use fantasy_realms_unofficial_api::{DrawCard, deck::Card, hand::Hand, card_collection::CardCollection};
pub mod randy; pub use randy::Randy;
pub mod automatos_v1; pub use automatos_v1::AutomatosV1;
pub mod automatos_v2; pub use automatos_v2::AutomatosV2;

/// Defines the functions needed for a bot.
pub trait FantasyRealmsBot {
    /// Gets the name of the bot.
    /// # Returns
    /// A `Srting` representing the name of the bot.
    fn name(&self) -> String;
    /// Generates the desision of what card to draw.
    /// # Arguments
    /// * `hand` - A `&Hand` representing the bots current hand.
    /// * `discard_pile` - A `&CardCollection` representing the discard pile. 
    /// * `known_opponent_cards` - A `&Vec<CardCollection>` representing the known cards in opponents hands.
    /// * `minimum_turns_remaining` - A `u8` representing the minimum turns that remain in the game. 
    /// # Returns
    /// A `DrawCard` representing what the bot drew. 
    fn generate_draw(
        &mut self,
        hand: &Hand,
        discard_pile: &CardCollection,
        known_opponent_cards: &Vec<CardCollection>, 
        minimum_turns_remaining: u8,
    ) -> DrawCard;
    /// Generates the desision of what card to discard.
    /// # Arguments
    /// * `hand` - A `&Hand` representing the bots current hand.
    /// * `discard_pile` - A `&CardCollection` representing the discard pile. 
    /// * `known_opponent_cards` - A `&Vec<CardCollection>` representing the known cards in opponents hands.
    /// * `minimum_turns_remaining` - A `u8` representing the minimum turns that remain in the game. 
    /// * `draw` - A `&Card` representing the card drawn.
    /// # Returns
    /// A `Card` representing what the bot discarded. 
    fn generate_discard(
        &mut self,
        hand: &Hand,
        discard_pile: &CardCollection,
        known_opponent_cards: &Vec<CardCollection>, 
        minimum_turns_remaining: u8,
        draw: &Card,
    ) -> Card;
}

/// Creates a new bot instance
/// # Arguments
/// * `bot_type` - A `String` representing the name of the bot.
/// # Errors
/// This function returns an `Err(String)` if:
/// * There is no bot coresponding to the name given
/// # Returns
/// A `Result<Box<dyn FantasyRealmsBot>, String>` which is:
/// * `Ok(Box<dyn FantasyRealmsBot>)` if the bot is successfully initialized.
/// * `Err(String)` containing an error message if validation fails.
pub fn new_bot(bot_type: String) -> Result<Box<dyn FantasyRealmsBot>, String> {
    let candidates = vec![
        Box::new(Randy) as Box<dyn FantasyRealmsBot>,
        Box::new(AutomatosV1 {discard: None}) as Box<dyn FantasyRealmsBot>,
        Box::new(AutomatosV2 {discard: None}) as Box<dyn FantasyRealmsBot>,
    ];
    for bot in candidates {
        if bot.name().trim().to_lowercase() == bot_type.trim().to_lowercase() {
            return Ok(bot);
        }
    }
    Err(format!("Unknown AI type: {}", bot_type.trim()))
}