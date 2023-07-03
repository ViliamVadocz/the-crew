use arrayvec::ArrayVec;

use crate::{
    card::{Card, ColorCard},
    game::HAND_SIZE,
};

#[derive(Clone, Debug)]
pub struct Player {
    hand: ArrayVec<Card, HAND_SIZE>,
    communication: Option<Communication>,
}

impl Player {
    #[must_use]
    pub fn new(hand: ArrayVec<Card, HAND_SIZE>) -> Self {
        Self {
            hand,
            communication: None,
        }
    }

    /// Check whether a player has a card.
    #[must_use]
    pub fn has(&self, card: &Card) -> bool {
        self.hand.contains(card)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Communication {
    card: ColorCard,
    info: Info,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Info {
    Highest,
    Lowest,
    Only,
}
