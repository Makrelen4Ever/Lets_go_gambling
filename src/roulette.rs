use rand::random_range;

use crate::game::*;
use crate::rendering::*;

struct Tile
{
    number: usize,
    bet_type: String,
}

impl Tile
{
    fn new(number: usize, bet_type: String) -> Tile
    {
        Tile
        {
            number,
            bet_type
        }
    }
}

impl Game
{
    pub fn roulette(&mut self) {
        let player_bet: usize = self.get_bet();
        let mut player_bet_type: Tile = Tile::new(0, String::new());

        let mut user_input: String;
        loop {
            println!("Enter bet type:");
            println!("Red, Black, Odd, Even, Lower, Middle, Higher or number from 0-36");
            
            user_input = String::new();
            std::io::stdin()
                .read_line(&mut user_input)
                .expect("Failure when fetching input");

            match user_input.to_lowercase().trim() {
                "red" | "black" | "odd" | "even" | "lower" | "middle" | "higher" => {
                    player_bet_type.bet_type = user_input
                        .to_lowercase()
                        .trim()
                        .to_string();
                    
                    break;
                },

                _ => {
                    let parse_result = user_input.trim().parse();
                    
                    if parse_result.is_ok()
                    {
                        let parsed_value: usize = parse_result.unwrap();
                        if parsed_value <= 36
                        {
                            player_bet_type.number = parsed_value;
                            player_bet_type.bet_type = "n".to_string();
                            break;
                        }
                    }
                    
                    println!("Unknown command");
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }
            }

            clear_terminal();
        }
        
        let mut player_won: bool = false;

        let result: usize = random_range(0..36);
        match player_bet_type.bet_type.as_str() {
            "lower" => {
                if result > 1 && result <= 12
                {
                    player_won = true;
                    self.chips += player_bet * 2;
                }
            },
            
            "middle" => {
                if result > 12 && result <= 24
                {
                    player_won = true;
                    self.chips += player_bet * 2;
                }
            },

            "higher" => {
                if result > 24 && result <= 36
                {
                    player_won = true;
                    self.chips += player_bet * 2;
                }
            },

            "red" | "even" => {
                if result % 2 == 0 && result != 0
                {
                    player_won = true;
                    self.chips += player_bet;
                }
            },

            "black" | "odd" => {
                if result % 2 == 1 && result != 0
                {
                    player_won = true;
                    self.chips += player_bet;
                }
            },

            "n" => {
                if result == player_bet_type.number
                {
                    player_won = true;
                    self.chips += player_bet * 36;
                }
            }
            
            _ => {

            }
        }

        let mut color_str: String = String::new();
        if result == 0
        {
            color_str = "Green".to_string();
        }else {
            if result % 2 == 0
            {
                color_str = "Red".to_string();
            } else
            {
                color_str = "Black".to_string();
            }
        }

        println!("Result: {0} {1}", color_str, result);
        if player_won
        {
            switch_color(Color::GREEN);
            println!("You won!");
        }else {
            switch_color(Color::RED);
            println!("You lost!");

            self.chips -= player_bet;
        }

        switch_color(Color::RESET);
    
    }
}