use crate::deck::Deck;
use crate::game_state::GameState;
use crate::player::Player;

extern crate num;
#[macro_use]
extern crate num_derive;

mod deck;
mod game_state;
mod player;

fn main() {
    let mut stock = Deck::new();

    let mut game_state = GameState::new();

    let mut player = Player::new();

    let card = deck::Card::new(deck::Values::Jack, deck::Suits::Hearts);

    println!("{}", player.status());

    player.add_number_card(card, 1);

    println!("{}", player.status());
}
