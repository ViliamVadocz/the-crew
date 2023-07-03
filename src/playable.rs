use arrayvec::ArrayVec;

use crate::{
    card::{Card, ColorCard},
    game::HAND_SIZE,
};

/// Get a list of playable cards given the hand and optionally the first card.
#[must_use]
pub fn playable(
    hand: ArrayVec<Card, HAND_SIZE>,
    first_card: Option<Card>,
) -> ArrayVec<Card, HAND_SIZE> {
    let Some(first_card) = first_card else { return hand };
    let has_match = match first_card {
        Card::Rocket(_) => hand.iter().any(|card| matches!(card, Card::Rocket(_))),
        Card::Color(ColorCard { color, .. }) => hand.iter().any(|card| match card {
            Card::Color(ColorCard { color: c, .. }) => &color == c,
            Card::Rocket(..) => false,
        }),
    };
    if has_match {
        hand.into_iter()
            .filter(|card| compatible(*card, first_card))
            .collect()
    } else {
        hand
    }
}

/// Check whether a card is compatible with another card (i.e. whether it can be
/// played if the other card was the first card in a trick).
fn compatible(card: Card, with: Card) -> bool {
    match card {
        Card::Rocket(_) => true,
        Card::Color(ColorCard { color, .. }) => Some(color) == with.color(),
    }
}
