use arrayvec::ArrayVec;

use crate::task::{Task, MAX_TASKS};

enum Message {
    Join,
    PreGame(PreGame),
}

// Player sends Join message
// Receives PreGame message with current amount of waiting players

struct PreGame {
    players: usize,
}
