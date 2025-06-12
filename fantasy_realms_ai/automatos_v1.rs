//! # AutomatosV1
//! AutomatosV1 plays the move that will give it the best
//! posible score at the end of its turn. 
//! For each card checked it finds the maximum score of replacing every card in its hand.
//! It checks each card in the discard pile.
//! Then it checks every unknown card, and takes the average score. 
//! This is the expected evaluation for the deck. 
//! It then chooses the highest scoring option. 
//! After drawing a card it decids if it is benificial to keep the card drawn or simply discard it.

use fantasy_realms_unofficial_api::{card_collection::CardCollection, deck::Card, hand::Hand, DrawCard};
use super::FantasyRealmsBot;

struct BotTurn {
    draw: DrawCard,
    discard: Card,
    evaluation: f32,
} impl BotTurn {
    fn new(draw: DrawCard, discard: Card, evaluation: f32) -> Self {
        BotTurn {draw, discard, evaluation}
    }
}

pub struct AutomatosV1 {
    pub(crate) discard: Option<Card>,
}

impl FantasyRealmsBot for AutomatosV1 {
    fn name(&self) -> String {
        "AutomatosV1".to_string()
    }

    fn generate_draw(
        &mut self,
        hand: &Hand, 
        discard_pile: &CardCollection, 
        known_opponent_cards: &Vec<CardCollection>, 
        _: u8,
    ) -> DrawCard {
        let unknown_cards: CardCollection = !(
            known_opponent_cards.iter().cloned().sum::<CardCollection>() + 
            *discard_pile +
            CardCollection::from(hand)
        );
        let mut turn = BotTurn::new(DrawCard::Deck, hand[0], hand.score(discard_pile) as f32);
        for i in 0..7 {
            for draw in discard_pile {
                let mut test_hand = hand.clone();
                test_hand[i] = draw;
                let new_discard_pile = discard_pile.clone() + hand[i] - draw;
                let evaluation: f32 = test_hand.score(&new_discard_pile) as f32;
                if evaluation > turn.evaluation {
                    turn = BotTurn::new(DrawCard::Discard (draw), hand[i], evaluation);
                }
            }
        }
        for i in 0..7 {
            let mut evaluation: f32 = 0.0;
            for draw in unknown_cards {
                let mut test_hand = hand.clone();
                test_hand[i] = draw;
                let new_discard_pile = discard_pile.clone() + hand[i] - draw;
                evaluation += test_hand.score(&new_discard_pile) as f32;
            }
            evaluation /= unknown_cards.len() as f32;
            if evaluation > turn.evaluation {
                turn = BotTurn::new(DrawCard::Deck, hand[i], evaluation)
            }
        }
        if turn.draw == DrawCard::Deck {
            self.discard = None;
        } else {
            self.discard = Some(turn.discard);
        }
        turn.draw
    }

    fn generate_discard(
            &mut self,
            hand: &Hand,
            discard_pile: &CardCollection,
            _: &Vec<CardCollection>, 
            _: u8,
            draw: &Card,
        ) -> Card {
        if self.discard.is_some() {
            return self.discard.unwrap();
        }
        let mut turn = BotTurn::new(DrawCard::Deck, *draw, hand.score(discard_pile) as f32);
        for i in 0..7 {
            let mut test_hand = hand.clone();
            test_hand[i] = *draw;
            let new_discard_pile = discard_pile.clone() + hand[i] - *draw;
            let evaluation: f32 = test_hand.score(&new_discard_pile) as f32;
            if evaluation > turn.evaluation {
                turn = BotTurn::new(DrawCard::Deck, hand[i], evaluation);
            }
        }
        turn.discard
    }
}