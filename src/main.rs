use crate::deck::Deck;
use crate::game_state::GameState;

extern crate num;
#[macro_use]
extern crate num_derive;

mod deck;
mod game_state;
mod player;

fn main() {
    let mut game_state = GameState::new();

    let card_ = deck::Card::new(deck::Values::King, deck::Suits::Clubs);
    let mut card = deck::Card::new(deck::Values::Ten, deck::Suits::Hearts);
    let card2 = deck::Card::new(deck::Values::Eight, deck::Suits::Diamonds);

    game_state.status();

    game_state.add_number_card(card, 0, 0).unwrap();

    game_state.status();

    game_state.add_number_card(card2, 0, 0).unwrap();

    game_state.status();

    game_state.add_face_card(card_, 0, 0, 1).unwrap();

    game_state.status();
}
