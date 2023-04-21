mod action;
mod card;
mod game;
mod messages;
mod player;
mod task;
mod trick;

#[cfg(test)]
mod tests {
    use crate::game::Game;

    #[test]
    fn test_game() {
        let mut game = Game::new(3);
    }
}
