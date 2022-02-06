use std::fmt::{Display};
use std::error;


use rand::{self, Rng};


use crate::deck::{Card, Deck, Values};

#[derive(Debug, Clone)]
pub struct Player {
    caravans: Vec<Caravan>,
    stock: Deck,
    hand: Deck,
}

impl Player {
    pub fn new() -> Self {
        let caravans = vec![Caravan::new(), Caravan::new(), Caravan::new()];
        let mut deck = Deck::new();
        let mut hand = Vec::new();
        deck.shuffle();
        for i in 0..8{
            hand.push(deck.deal(i));
        }
        Self { caravans, stock: deck, hand: Deck::from_slice(&hand) }
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

    pub fn add_number_card(&mut self, card: Card, caravan_number: usize) -> Result<(), Box<dyn error::Error>>{
        if card.value.is_face() {
            return Err("Can't add a face card".into());
        }; 
        match self.caravans[caravan_number].cards.last() {
            Some(last_card) => {
                match self.caravans[caravan_number].state {
                    CaravanState::Any => {
                        self.caravans[caravan_number].add(card.value as u8);
                        Ok(())
                    },
                    CaravanState::Decreasing => {
                        if last_card.value.gt(&card.value){
                            self.caravans[caravan_number].add(card.value as u8);
                            Ok(())
                        } else {
                            Err("Card value doesn't match caravan state".into())
                        }
                    },
                    CaravanState::Increasing => {
                        if last_card.value.lt(&card.value){
                            self.caravans[caravan_number].add(card.value as u8);
                            Ok(())
                        } else {
                            Err("Card value doesn't match caravan state".into())
                        }
                    },
                }
            },
            None => {
                self.caravans[caravan_number].add(card.value as u8);
                Ok(())
             },
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Caravan {
    number: u8,
    sold: bool,
    state: CaravanState,
    cards: Vec<Card>,
}

impl Caravan {
    pub fn new() -> Self {
        Default::default()
    }

    fn add(&mut self, number: u8) {
        self.number += number;
    }

    fn toggle(&mut self) {
        self.sold = !self.sold;
    }
    fn change_state(&mut self, state: CaravanState){
        self.state = state;
    }
}

impl Display for Caravan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "number: {}, sold: {}", self.number, self.sold)
    }
}

#[derive(Debug, Clone)]
pub enum CaravanState {
    Any,
    Decreasing,
    Increasing
}

impl Default for CaravanState {
    fn default() -> Self {
        Self::Any
    }
} 