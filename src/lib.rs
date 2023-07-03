#![warn(clippy::pedantic)]
#![warn(clippy::unwrap_used)]

pub mod card;
pub mod game;
pub mod message;
pub mod playable;
pub mod player;
pub mod task;
pub mod trick;

#[cfg(test)]
mod tests {
    use crate::game::Game;

    #[test]
    fn test_game() {
        let mut game = Game::new(3);
    }
}
