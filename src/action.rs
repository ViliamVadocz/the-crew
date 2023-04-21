use crate::{card::Card, player::Communication};

enum Action {
    Communicate {
        player: usize,
        communication: Communication,
    },
    Play(Card),
}
