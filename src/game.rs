use arrayvec::ArrayVec;
use rand::{seq::SliceRandom, thread_rng};

use crate::{
    card::{
        consts::{DECK, ROCKET_4},
        Card,
    },
    player::Player,
    task::{Task, MAX_TASKS},
};

// assume 4 players
pub const PLAYERS: usize = 4;
pub const HAND_SIZE: usize = 40 / PLAYERS;

#[derive(Clone, Debug)]
pub struct Game {
    players: [Player; PLAYERS],
    tasks: ArrayVec<Task, MAX_TASKS>,
    leader: usize, // 0..4
}

impl Game {
    pub fn new(number_of_tasks: usize) -> Game {
        assert!(number_of_tasks <= MAX_TASKS);
        let mut deck = DECK;

        let mut rng = thread_rng();

        deck.shuffle(&mut rng);
        let players = {
            let mut iter = 0..PLAYERS;
            [(); 4].map(|()| {
                let i = iter.next().unwrap();
                Player::new(
                    deck[i * HAND_SIZE..(i + 1) * HAND_SIZE]
                        .iter()
                        .copied()
                        .collect(),
                )
            })
        };

        deck.shuffle(&mut rng);
        let tasks = {
            let mut tasks = ArrayVec::default();
            let mut iter = deck.into_iter();
            while tasks.len() < number_of_tasks {
                match iter.next() {
                    Some(Card::Color(card)) => tasks.push(Task::new(card)),
                    Some(_) => continue,
                    None => unreachable!(),
                }
            }
            tasks
        };

        let leader = players
            .iter()
            .position(|player| player.has(&ROCKET_4))
            .expect("one of the players must have a Rocket-4");

        Self {
            players,
            tasks,
            leader,
        }
    }
}

// TODO
// divide up tasks among players clockwise from commander
// move-gen
// stages
// - pre-game: divide up tasks, identify commander
// - pre-trick: communication
// - trick: each player starting at leader plays a card, winner is next leader
