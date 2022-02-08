use std::error;
use std::fmt::Display;

use rand::{self, Rng};

use crate::deck::{Card, Deck, Values};

#[derive(Debug, Clone)]
pub struct Player {
    pub caravans: Vec<Caravan>,
    stock: Deck,
    hand: Deck,
}

impl Player {
    pub fn new() -> Self {
        let caravans = vec![Caravan::new(), Caravan::new(), Caravan::new()];
        let mut deck = Deck::new();
        let mut hand = Vec::new();
        deck.shuffle();
        for i in 0..8 {
            hand.push(deck.deal(i));
        }
        Self {
            caravans,
            stock: deck,
            hand: Deck::from_slice(&hand),
        }
    }

    pub fn status(&self) -> String {
        let mut report = String::new();
        for caravan in self.caravans.iter() {
            report.push_str(&(caravan.to_string() + "\n"));
        }
        report.push_str(&format!("stock count: {}\n", self.stock.len()));
        report.push_str(&format!("hand count: {}\n", self.hand.len()));
        report
    }
}

#[derive(Default, Debug, Clone)]
pub struct Caravan {
    pub number: u8,
    pub sold: bool,
    pub state: CaravanState,
    pub cards: Vec<Card>,
}

impl Caravan {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, card: Card) {
        self.number += card.score();
        self.cards.push(card);
    }

    pub fn remove(&mut self, card: usize) -> Card{
        self.number -= self.cards[card].score();
        self.cards[card].attached.take();
        self.cards.remove(card)
    }

    fn toggle(&mut self) {
        self.sold = !self.sold;
    }

    fn change_state(&mut self, state: CaravanState) {
        self.state = state;
    }

    pub fn swap_state(&mut self){
        match self.state {
            CaravanState::Any => {},
            CaravanState::Decreasing => {
                self.state = CaravanState::Increasing;
            },
            CaravanState::Increasing => {
                self.state = CaravanState::Decreasing;
            },
        }
    }
}

impl Display for Caravan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "number: {}, sold: {}, state: {}, cards: {}", self.number, self.sold, self.state, self.cards.len())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CaravanState {
    Any,
    Decreasing,
    Increasing,
}


impl Default for CaravanState {
    fn default() -> Self {
        Self::Any
    }
}

impl Display for CaravanState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}