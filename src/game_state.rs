use std::{default::Default, fmt::Display};

#[derive(Default, Debug)]
pub struct Caravan {
    number: u8,
    sold: bool,
}

impl Caravan {
    pub fn new() -> Self {
        Default::default()
    }

    fn add(&mut self, number: u8){
        self.number += number;
    }

    fn toggle(&mut self){
        self.sold = !self.sold;
    }
}

impl Display for Caravan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "value: {}, sold: {}", self.number, self.sold)
    }
}