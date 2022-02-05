use core::fmt;
use itertools::Itertools;
use rand::{self, Rng};

use crate::{deck::{Card, Deck, Values}, game_state::Caravan};

#[derive(Debug)]
pub struct Player {
    caravans: Vec<Caravan>,
    
}

