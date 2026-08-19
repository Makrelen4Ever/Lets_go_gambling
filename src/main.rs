use std::{fs, process::Output};
use serde_json::*;

mod rendering;
mod game;
mod cards;
mod blackjack;
mod roulette;
mod slots;

use rendering::*;
use game::*;
use cards::*;

fn main() {
    clear_terminal();
    let mut game: Game = Game { chips: 1000, deck: Deck::new(4) };
    game.init();
}