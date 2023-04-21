use crate::{card::Card, game::PLAYERS};

/// Get the index of the winner of the trick.
/// The first card in the trick was played by the leader for that round.
pub fn winner(trick: [Card; PLAYERS]) -> usize {
    let color = trick[0].color();
    trick
        .iter()
        .enumerate()
        .max_by_key(|(_, card)| match card.color() {
            None => card.value(),
            c if c == color => card.value(),
            _ => 0,
        })
        .unwrap()
        .0
}

#[cfg(test)]
mod tests {
    use crate::{card::consts::*, trick::winner};

    #[test]
    fn highest_color_wins() {
        assert_eq!(0, winner([BLUE_9, BLUE_7, BLUE_5, BLUE_3]));
        assert_eq!(1, winner([PINK_1, PINK_8, PINK_4, PINK_6]));
        assert_eq!(2, winner([GREEN_2, GREEN_3, GREEN_4, GREEN_1]));
        assert_eq!(3, winner([YELLOW_1, YELLOW_7, YELLOW_5, YELLOW_9]));
    }

    #[test]
    fn color_priority() {
        assert_eq!(0, winner([BLUE_1, GREEN_7, PINK_9, YELLOW_4]));
        assert_eq!(1, winner([GREEN_5, GREEN_8, YELLOW_9, BLUE_9]));
        assert_eq!(2, winner([GREEN_2, PINK_8, GREEN_3, GREEN_1]));
    }

    #[test]
    fn rocket_priority() {
        assert_eq!(2, winner([BLUE_5, BLUE_8, ROCKET_1, YELLOW_9]));
        assert_eq!(1, winner([YELLOW_9, ROCKET_4, GREEN_9, ROCKET_1]));
        assert_eq!(0, winner([ROCKET_1, BLUE_9, GREEN_9, YELLOW_9]));
    }

    #[test]
    fn highest_rocket_wins() {
        assert_eq!(0, winner([ROCKET_4, PINK_9, ROCKET_3, ROCKET_1]));
        assert_eq!(2, winner([ROCKET_1, ROCKET_3, ROCKET_4, ROCKET_2]));
    }
}
