use arrayvec::ArrayVec;
use serde::{Deserialize, Serialize};

use crate::{
    card::Card,
    game::HAND_SIZE,
    player::Communication,
    task::{Task, MAX_TASKS},
};

enum Message {
    Join(Join),
    PreGame(PreGame),
    Discussion(Discussion),
    Trick(Trick),
}

enum Join {
    Game { token: u64 },
    YouAre { player_index: usize },
}

enum PreGame {
    Commander {
        player_index: usize,
    },
    PickATask {
        commander: usize,
        hand: ArrayVec<Card, HAND_SIZE>,
        tasks: ArrayVec<Task, MAX_TASKS>,
    },
}

enum Discussion {
    IWantToCommunicate {
        communication: Communication,
    },
    SomeoneCommunicated {
        player_index: usize,
        communication: Communication,
    },
}

enum Trick {
    YourTurn {
        hand: ArrayVec<Card, HAND_SIZE>,
        tasks: ArrayVec<(usize, Task), MAX_TASKS>,
        trick: ArrayVec<Card, 4>,
    },
}
