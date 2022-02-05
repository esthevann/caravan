use core::fmt;
use rand::Rng;
use std::{fmt::Debug, ops::Deref, ops::DerefMut};

#[allow(dead_code)]
#[derive(
    Debug, strum_macros::Display, Clone, FromPrimitive, PartialEq, Copy, PartialOrd, Eq, Ord,
)]
pub enum Values {
    Ace = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
}

#[allow(dead_code)]
#[derive(
    Debug, strum_macros::Display, Clone, FromPrimitive, PartialEq, Copy, PartialOrd, Eq, Ord,
)]
pub enum Suits {
    Diamonds,
    Clubs,
    Hearts,
    Spades,
}

#[allow(dead_code)]
#[derive(Clone, Eq, PartialOrd, Ord, PartialEq, Debug)]
pub struct Card {
    pub value: Values,
    pub suit: Suits,
    name: String,
}

impl Card {
    pub fn new(value: Values, suit: Suits) -> Self {
        let name = format!("{} of {}", value, suit);
        Self { value, suit, name }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Clone)]
pub struct Deck(Vec<Card>);

impl Deck {
    pub fn new() -> Self {
        let mut cards: Vec<Card> = vec![];
        for i in 0..4 {
            for j in 1..14 {
                cards.push(Card::new(
                    num::FromPrimitive::from_i32(j).unwrap(),
                    num::FromPrimitive::from_i32(i).unwrap(),
                ));
            }
        }
        Self(cards)
    }
    pub fn from_slice(cards: &[Card]) -> Self {
        Self(cards.to_vec())
    }
    pub fn deal(&mut self, index: usize) -> Card {
        self.remove(index)
    }
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        let mut deck_clone = self.clone();
        self.clear();
        while deck_clone.len() > 0 {
            let index = rng.gen_range(0..deck_clone.len());
            let card = deck_clone.get_mut(index).unwrap();
            self.push(deck_clone.remove(index));
        }
    }
}

impl Deref for Deck {
    type Target = Vec<Card>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Deck {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
