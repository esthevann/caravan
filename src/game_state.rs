use std::{error, cmp::Ordering};

use crate::{
    deck::{Card, Values},
    player::{CaravanState, Player},
};

pub struct GameState {
    players: Vec<Player>,
    game_over: bool,
    first_round: bool,
}

impl GameState {
    pub fn new() -> Self {
        let players = vec![Player::new(), Player::new()];
        Self {
            players,
            game_over: false,
            first_round: true,
        }
    }

    pub fn status(&self) {
        for (i, player) in self.players.iter().enumerate() {
            println!("Player #{} Status: ", i + 1);
            println!("{}", player.status());
        }
    }

    pub fn add_number_card(
        &mut self,
        card: Card,
        player_number: usize,
        caravan_number: usize,
    ) -> Result<(), Box<dyn error::Error>> {
        if card.value.is_face() {
            return Err("Can't add a face card".into());
        };
        match self.players[player_number].caravans[caravan_number]
            .cards
            .last()
        {
            Some(last_card) => match self.players[player_number].caravans[caravan_number].state {
                CaravanState::Any => {
                    match card.value.cmp(&last_card.value) {
                        Ordering::Less => {
                            self.players[player_number].caravans[caravan_number].state =
                                CaravanState::Decreasing;
                        }
                        Ordering::Equal => {
                            return Err(
                                "Can't add a card with the same value as the previous one".into()
                            );
                        }
                        Ordering::Greater => {
                            self.players[player_number].caravans[caravan_number].state =
                                CaravanState::Increasing;
                        }
                    }
                    self.players[player_number].caravans[caravan_number].add(card);
                    Ok(())
                }
                CaravanState::Decreasing => {
                    if last_card.value.gt(&card.value) || last_card.suit == card.suit {
                        self.players[player_number].caravans[caravan_number].add(card);
                        Ok(())
                    } else {
                        Err("Card value doesn't match caravan state".into())
                    }
                }
                CaravanState::Increasing => {
                    if last_card.value.lt(&card.value) || last_card.suit == card.suit {
                        self.players[player_number].caravans[caravan_number].add(card);
                        Ok(())
                    } else {
                        Err("Card value doesn't match caravan state".into())
                    }
                }
            },
            None => {
                self.players[player_number].caravans[caravan_number].add(card);
                Ok(())
            }
        }
    }
    pub fn add_face_card(
        &mut self,
        card_played: Card,
        player_against: usize,
        caravan_number: usize,
        card_against: usize,
    ) -> Result<(), Box<dyn error::Error>> {
        match card_played.value {
            Values::Jack => {
                self.players[player_against].caravans[caravan_number].remove(card_against);
                Ok(())
            },
            Values::Queen => {
                self.players[player_against].caravans[caravan_number].cards[card_against].add_attached(card_played);
                self.players[player_against].caravans[caravan_number].swap_state();
                Ok(())
            },
            Values::King => {
                let mut card = self.players[player_against].caravans[caravan_number].remove(card_against);
                card.add_attached(card_played);
                self.players[player_against].caravans[caravan_number].add(card);
                Ok(())
            },
            _ => Err("Can't play number card".into()),
        }
    }
}
