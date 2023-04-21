#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    Blue,
    Green,
    Pink,
    Yellow,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ColorValue(u8);

impl ColorValue {
    pub fn new(value: u8) -> Self {
        assert!((1..=9).contains(&value));
        Self(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ColorCard {
    pub color: Color,
    pub value: ColorValue,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RocketValue(u8);

impl RocketValue {
    pub fn new(value: u8) -> Self {
        assert!((1..=4).contains(&value));
        Self(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RocketCard {
    pub value: RocketValue,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Card {
    Rocket(RocketCard),
    Color(ColorCard),
}

impl Card {
    pub fn color(self) -> Option<Color> {
        match self {
            Card::Color(ColorCard { color, .. }) => Some(color),
            Card::Rocket(_) => None,
        }
    }

    pub fn value(self) -> u8 {
        match self {
            Card::Color(ColorCard {
                value: ColorValue(value),
                ..
            }) => value,
            Card::Rocket(RocketCard {
                value: RocketValue(value),
            }) => value + 10,
        }
    }
}

#[rustfmt::skip]
#[allow(unused)]
pub mod consts {
    use super::{Card, RocketCard, RocketValue, Color, ColorValue, ColorCard};

    pub const ROCKET_1: Card = Card::Rocket(RocketCard { value: RocketValue(1) });
    pub const ROCKET_2: Card = Card::Rocket(RocketCard { value: RocketValue(2) });
    pub const ROCKET_3: Card = Card::Rocket(RocketCard { value: RocketValue(3) });
    pub const ROCKET_4: Card = Card::Rocket(RocketCard { value: RocketValue(4) });
    pub const BLUE_1: Card = Card::Color(ColorCard { color: Color::Blue, value: ColorValue(1) });
    pub const BLUE_2: Card = Card::Color(ColorCard { color: Color::Blue, value: ColorValue(2) });
    pub const BLUE_3: Card = Card::Color(ColorCard { color: Color::Blue, value: ColorValue(3) });
    pub const BLUE_4: Card = Card::Color(ColorCard { color: Color::Blue, value: ColorValue(4) });
    pub const BLUE_5: Card = Card::Color(ColorCard { color: Color::Blue, value: ColorValue(5) });
    pub const BLUE_6: Card = Card::Color(ColorCard { color: Color::Blue, value: ColorValue(6) });
    pub const BLUE_7: Card = Card::Color(ColorCard { color: Color::Blue, value: ColorValue(7) });
    pub const BLUE_8: Card = Card::Color(ColorCard { color: Color::Blue, value: ColorValue(8) });
    pub const BLUE_9: Card = Card::Color(ColorCard { color: Color::Blue, value: ColorValue(9) });
    pub const GREEN_1: Card = Card::Color(ColorCard { color: Color::Green, value: ColorValue(1) });
    pub const GREEN_2: Card = Card::Color(ColorCard { color: Color::Green, value: ColorValue(2) });
    pub const GREEN_3: Card = Card::Color(ColorCard { color: Color::Green, value: ColorValue(3) });
    pub const GREEN_4: Card = Card::Color(ColorCard { color: Color::Green, value: ColorValue(4) });
    pub const GREEN_5: Card = Card::Color(ColorCard { color: Color::Green, value: ColorValue(5) });
    pub const GREEN_6: Card = Card::Color(ColorCard { color: Color::Green, value: ColorValue(6) });
    pub const GREEN_7: Card = Card::Color(ColorCard { color: Color::Green, value: ColorValue(7) });
    pub const GREEN_8: Card = Card::Color(ColorCard { color: Color::Green, value: ColorValue(8) });
    pub const GREEN_9: Card = Card::Color(ColorCard { color: Color::Green, value: ColorValue(9) });
    pub const PINK_1: Card = Card::Color(ColorCard { color: Color::Pink, value: ColorValue(1) });
    pub const PINK_2: Card = Card::Color(ColorCard { color: Color::Pink, value: ColorValue(2) });
    pub const PINK_3: Card = Card::Color(ColorCard { color: Color::Pink, value: ColorValue(3) });
    pub const PINK_4: Card = Card::Color(ColorCard { color: Color::Pink, value: ColorValue(4) });
    pub const PINK_5: Card = Card::Color(ColorCard { color: Color::Pink, value: ColorValue(5) });
    pub const PINK_6: Card = Card::Color(ColorCard { color: Color::Pink, value: ColorValue(6) });
    pub const PINK_7: Card = Card::Color(ColorCard { color: Color::Pink, value: ColorValue(7) });
    pub const PINK_8: Card = Card::Color(ColorCard { color: Color::Pink, value: ColorValue(8) });
    pub const PINK_9: Card = Card::Color(ColorCard { color: Color::Pink, value: ColorValue(9) });
    pub const YELLOW_1: Card = Card::Color(ColorCard { color: Color::Yellow, value: ColorValue(1) });
    pub const YELLOW_2: Card = Card::Color(ColorCard { color: Color::Yellow, value: ColorValue(2) });
    pub const YELLOW_3: Card = Card::Color(ColorCard { color: Color::Yellow, value: ColorValue(3) });
    pub const YELLOW_4: Card = Card::Color(ColorCard { color: Color::Yellow, value: ColorValue(4) });
    pub const YELLOW_5: Card = Card::Color(ColorCard { color: Color::Yellow, value: ColorValue(5) });
    pub const YELLOW_6: Card = Card::Color(ColorCard { color: Color::Yellow, value: ColorValue(6) });
    pub const YELLOW_7: Card = Card::Color(ColorCard { color: Color::Yellow, value: ColorValue(7) });
    pub const YELLOW_8: Card = Card::Color(ColorCard { color: Color::Yellow, value: ColorValue(8) });
    pub const YELLOW_9: Card = Card::Color(ColorCard { color: Color::Yellow, value: ColorValue(9) });

    pub const DECK_SIZE: usize = 40;
    pub const DECK: [Card; DECK_SIZE] = [
        ROCKET_1, ROCKET_2, ROCKET_3, ROCKET_4, BLUE_1, BLUE_2, BLUE_3, BLUE_4, BLUE_5, BLUE_6, BLUE_7,
        BLUE_8, BLUE_9, GREEN_1, GREEN_2, GREEN_3, GREEN_4, GREEN_5, GREEN_6, GREEN_7, GREEN_8,
        GREEN_9, PINK_1, PINK_2, PINK_3, PINK_4, PINK_5, PINK_6, PINK_7, PINK_8, PINK_9, YELLOW_1,
        YELLOW_2, YELLOW_3, YELLOW_4, YELLOW_5, YELLOW_6, YELLOW_7, YELLOW_8, YELLOW_9,
    ];
}
