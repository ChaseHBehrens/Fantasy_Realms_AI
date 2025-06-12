//! # Randy
//! Randy plays random moves.
//! It has an equal probability of choosing each card from
//! each card in the discard pile, or the deck. 

use rand::seq::SliceRandom;
use rand::thread_rng;
use fantasy_realms_unofficial_api::{card_collection::CardCollection, deck::Card, hand::Hand, DrawCard};
use super::FantasyRealmsBot;

pub struct Randy;

impl FantasyRealmsBot for Randy {
    fn name(&self) -> String {
        "Randy".to_string()
    }

    fn generate_draw(
        &mut self,
        _: &Hand, 
        discard_pile: &CardCollection, 
        _: &Vec<CardCollection>, 
        _: u8,
    ) -> DrawCard {
        let mut cards: Vec<DrawCard> = discard_pile
            .iter()
            .map(|card| DrawCard::Discard (card))
            .collect();
        cards.push(DrawCard::Deck);
        let mut rng = thread_rng();
        *cards.choose(&mut rng).unwrap()
    }

    fn generate_discard(
            &mut self,
            hand: &Hand,
            _: &CardCollection,
            _: &Vec<CardCollection>, 
            _: u8,
            draw: &Card,
    ) -> Card {
        let mut cards: Vec<Card> = hand.to_vec();
        cards.push(*draw);
        let mut rng = thread_rng();
        *cards.choose(&mut rng).unwrap()
    }
}