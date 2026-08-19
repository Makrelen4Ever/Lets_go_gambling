use std::collections::HashMap;

use crate::game::*;
use crate::cards::*;
use crate::rendering::*;

impl Game {
    pub fn slots(&mut self)
    {
        let player_bet = self.get_bet();
        
        let mut user_input: String = String::new();
        
        loop
        {
            self.chips -= player_bet;

            let mut player_score: usize = 0;

            let mut hand: Hand = Hand::new();
            hand.draw_cards(5, &mut Deck::new(1));
            hand.cards.sort_unstable_by(|a, b| b.val.cmp(&a.val));
            hand.render_hand(true);
            print!("\n");
            
            let is_flush = hand.cards.iter().all(|c| c.ch == hand.cards[0].ch);
            let mut is_straight = true;

            let mut has_pair: bool = false;
            let mut has_tris: bool = false;
            let mut has_quads: bool = false;
            
            for i in 0..4
            {
                if hand.cards[i].val != hand.cards[i + 1].val + 1 {
                    is_straight = false;
                }
            }

            let mut card_array: [usize; 13] = [0; 13];

            for i in 0..5 
            {
                card_array[hand.cards[i].val - 1] += 1;
            }

            for val in card_array
            {
                match val {
                    2 => {
                        has_pair = true;
                    },
                    3 => {
                        has_tris = true;
                    },
                    4 => {
                        has_quads = true;
                    },
                    _ => {

                    }
                }
            }

            if is_flush && is_straight
            {
                player_score = 8;
                println!("Straight flush X1000");
            } else if has_quads
            {
                player_score = 7;
                println!("Quads X200");
            } else if has_tris && has_pair
            {
                player_score = 6;
                println!("Full house X50");
            } else if is_flush
            {
                player_score = 5;
                println!("Flush X25");
            } else if is_straight
            {
                player_score = 4;
                println!("Straight X10");
            } else if has_tris
            {
                player_score = 3;
                println!("Three of a kind X3");
            } else if has_pair
            {
                player_score = 2;
                println!("Pair X0.5");
            } else
            {
                player_score = 1;
                println!("High X0");
            }

            let score_table: [usize; 8] = [
                0,
                0,
                10,
                25,
                50,
                100,
                200,
                1000,
            ];

            if player_score == 2
            {
                self.chips += player_bet / 2;
            } else
            {
                self.chips += player_bet * score_table[player_score - 1];
            }

            println!("\nCurrent chips: {}", self.chips);
            
            switch_color(Color::GREEN);
            println!("Press Enter to continue");
            switch_color(Color::RED);
            println!("Type 'Exit' to exit");
            switch_color(Color::RESET);

            if self.chips < player_bet
            {
                println!("Out of chips");
                break;
            }

            user_input = String::new();
            std::io::stdin()
                .read_line(&mut user_input)
                .expect("Failure when fetching input");

            match user_input.to_lowercase().trim() {
                "exit" => {
                    break;
                },

                _ => {
                    
                }
            }
            
            clear_terminal();
        }
    }
}