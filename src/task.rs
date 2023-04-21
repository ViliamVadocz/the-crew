use static_assertions::const_assert;

use crate::card::{consts::DECK_SIZE, ColorCard};

pub const MAX_TASKS: usize = 10;
const_assert!(MAX_TASKS <= DECK_SIZE - 4);

#[derive(Clone, Debug)]
pub struct Task {
    card: ColorCard,
    // TODO modifiers
}

impl Task {
    pub fn new(card: ColorCard) -> Self {
        Self { card }
    }
}
