//! # AutomatosV2
//! AutomatosV2 functions idenicly to AutomatosV1 with an added recursive depth search.
//! The max depth is set to one move into the future so 
//! that it can be run in a reasonable amount of time. 
//! This method did not have any noticable improvement over
//! AutomatosV1 due to the large random factor of the game itself. 

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

pub struct AutomatosV2 {
    pub(crate) discard: Option<Card>,
}

impl FantasyRealmsBot for AutomatosV2 {
    fn name(&self) -> String {
        "AutomatosV2".to_string()
    }

    fn generate_draw(
        &mut self,
        hand: &Hand, 
        discard_pile: &CardCollection, 
        known_opponent_cards: &Vec<CardCollection>, 
        minimum_turns_remaining: u8,
    ) -> DrawCard {
        let unknown_cards: CardCollection = !(
            known_opponent_cards.iter().cloned().sum::<CardCollection>() + 
            *discard_pile +
            CardCollection::from(hand)
        );
        let mut turn = BotTurn::new(DrawCard::Deck, hand[0], 
            evaluate_hand(hand, discard_pile, known_opponent_cards, minimum_turns_remaining)
        );
        for i in 0..7 {
            for draw in discard_pile {
                let mut test_hand = hand.clone();
                test_hand[i] = draw;
                let new_discard_pile = discard_pile.clone() + hand[i] - draw;
                let evaluation: f32 = evaluate_hand(
                    &test_hand, 
                    &new_discard_pile, 
                    known_opponent_cards, 
                    minimum_turns_remaining,
                );
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
                evaluation += evaluate_hand(
                    &test_hand, 
                    &new_discard_pile, 
                    known_opponent_cards, 
                    minimum_turns_remaining,
                );
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
            known_opponent_cards: &Vec<CardCollection>, 
            minimum_turns_remaining: u8,
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
            let evaluation: f32 = evaluate_hand(
                &test_hand, 
                &new_discard_pile, 
                known_opponent_cards, 
                minimum_turns_remaining,
            );
            if evaluation > turn.evaluation {
                turn = BotTurn::new(DrawCard::Deck, hand[i], evaluation);
            }
        }
        turn.discard
    }
}

fn evaluate_hand(
    hand: &Hand, 
    discard_pile: &CardCollection, 
    known_opponent_cards: &Vec<CardCollection>, 
    minimum_turns_remaining: u8,
) -> f32 {
    if minimum_turns_remaining == 0 {
        return hand.score(discard_pile) as f32;
    }
    if minimum_turns_remaining > 1 {
        return evaluate_hand(hand, discard_pile, known_opponent_cards, 1);
    }
    let unknown_cards: CardCollection = !(
        known_opponent_cards.iter().cloned().sum::<CardCollection>() + 
        *discard_pile +
        CardCollection::from(hand)
    );
    let mut max_evaluation: f32 = hand.score(discard_pile) as f32;
    for i in 0..7 {
        for draw in discard_pile {
            let mut test_hand = hand.clone();
            test_hand[i] = draw;
            let new_discard_pile = discard_pile.clone() + hand[i] - draw;
            let evaluation: f32 = evaluate_hand(
                &test_hand,
                &new_discard_pile,
                known_opponent_cards,
                minimum_turns_remaining - 1,
            );
            if evaluation > max_evaluation {
                max_evaluation = evaluation;
            }
        }
    }
    for i in 0..7 {
        let mut evaluation: f32 = 0.0;
        for draw in unknown_cards {
            let mut test_hand = hand.clone();
            test_hand[i] = draw;
            let new_discard_pile = discard_pile.clone() + hand[i] - draw;
            evaluation += evaluate_hand(
                &test_hand,
                &new_discard_pile,
                known_opponent_cards,
                minimum_turns_remaining - 1,
            );
        }
        evaluation /= unknown_cards.len() as f32;
        if evaluation > max_evaluation {
            max_evaluation = evaluation;
        }
    }
    max_evaluation
}