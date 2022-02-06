use crate::player::{Player};


pub struct GameState {
    players: Vec<Player>,
    game_over: bool,
    first_round: bool,
}

impl GameState {
    pub fn new() -> Self {
        let players = vec![Player::new(), Player::new()];
        Self { players, game_over: false, first_round: true }
    }

    pub fn status(&self) {
        for (i, player) in self.players.iter().enumerate() {
            println!("Player #{} Status: ", i + 1);
            println!("{}", player.status());
        }
    }


}
