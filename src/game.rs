use crate::rendering::*;
use crate::cards::*;

pub struct Game {
    pub chips: usize,
    pub deck: Deck,
}

impl Game {
    pub fn init(&mut self) {
        println!("Let's go GAMBLING\n");

        let mut user_input: String;

        loop {
            user_input = String::new();

            println!("You currently have {} chips", self.chips);
            println!("\nAvailable games: \n\nRoulette, \nBlackjack, \nSlots");

            std::io::stdin()
                .read_line(&mut user_input)
                .expect("Failure when fetching input");

            match user_input.to_lowercase().trim() {
                "blackjack" => {
                    self.black_jack();
                },

                "slots" => {
                    self.slots();
                }

                "roulette" => {
                    self.roulette();
                },

                "exit" => {
                    break;
                },

                "clear" => {
                    clear_terminal();
                },

                "reset" => {
                    clear_terminal();
                    
                    switch_color(Color::RED);
                    println!("The game has been reset. Current balance 1000");
                    switch_color(Color::RESET);
                    
                    self.chips = 1000;
                },

                _ => {
                    clear_terminal();

                    switch_color(Color::RED);
                    println!("Unknown input. Type 'Exit' for exitting.");
                    switch_color(Color::RESET);
                }
            }
        }
    }
    
    pub fn get_bet(&self) -> usize
    {
        let mut player_bet: usize = 0;
        let mut user_input: String;
        loop {
            switch_color(Color::RESET);
            println!("Enter your bet:");
            println!("Current chips: {}", self.chips);
    
            user_input = String::new();
            std::io::stdin()
                .read_line(&mut user_input)
                .expect("Failure when fetching input");
    
            let parse_result = user_input
                .trim()
                .parse();
    
            if parse_result.is_ok() && parse_result.clone().unwrap() > 0
            {
                player_bet = parse_result.unwrap();
    
                if player_bet > self.chips
                {
                    player_bet = self.chips;
                    switch_color(Color::GREEN);
                    println!("All in!");
                }else {
                    switch_color(Color::GREEN);
                    println!("You bet: {0}", player_bet);
                }
    
                switch_color(Color::RESET);
                break;
            }else {
                clear_terminal();
                switch_color(Color::RED);
                continue;
            }
        }

        return player_bet;
    }
}
