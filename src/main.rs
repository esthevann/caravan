use crate::deck::Deck;
use crate::player::Player;
use crate::game_state::Caravan;

extern crate num;
#[macro_use]
extern crate num_derive;

mod deck;
mod player;
mod game_state;

fn main() {
    let mut stock = Deck::new();

    let opponents_names = vec!["John"];
   
    let i = Caravan::new();

    println!("{i}");
}
