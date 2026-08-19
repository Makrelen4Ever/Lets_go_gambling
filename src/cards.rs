use std::{ops::Add, vec};

use rand::{RngExt, rng, seq::SliceRandom};

#[derive(Copy, Clone)]
pub struct Card
{
    pub ch: [char; 3],
    pub val: usize,
}

impl Card
{
    pub fn new(display: [char; 3], val_new: usize) -> Card
    {
        Card { ch: (display), val: (val_new) }
    }

    pub fn render(&self)
    {
        let ch_string: String = self.ch.iter().collect();
        print!("{ch_string}");
    }

    pub fn render_debug(&self)
    {
        let ch_string: String = self.ch.iter().collect();
        print!("Display: {ch_string}, Value: {0}", self.val);
    }
}

pub struct Deck
{
    pub cards: Vec<Card>,
    pub deck_count: usize,
}

impl Deck {
    pub fn new(deck_count: usize) -> Deck
    {
        let card_displays: [&str; 13] = [
            "A",
            "2",
            "3",
            "4",
            "5",
            "6",
            "7",
            "8",
            "9",
            "10",
            "J",
            "Q",
            "K"
        ];

        let suits: [&str; 4] = [
            "♥",
            "♣",
            "♦",
            "♠"
        ];

        let mut card_buffer: Vec<Card> = Vec::new();

        for d in 0..deck_count
        {
            for i in 0..4
            {
                for j in 0..13
                {
                    //Calculates the suit and value of the card, and composes it into a string
                    //Adds additional " " to make sure the length is always 3 or more
                    let card_display_string: String = card_displays[j].to_string().add(suits[i]).add(" ");
    
                    card_buffer.push( Card::new([
                            card_display_string.chars().nth(0).unwrap(),
                            card_display_string.chars().nth(1).unwrap(),
                            card_display_string.chars().nth(2).unwrap()
                        ],
                        (j + 1)));
                }
            }
        }

        card_buffer.shuffle(&mut rand::rng());

        return  Deck {
            cards: card_buffer,
            deck_count: deck_count,
        };
    }

    pub fn shuffle(&mut self)
    {
        self.cards = Deck::new(self.deck_count).cards;
    }

    pub fn render_deck(&self)
    {
        for c in self.cards.clone()
        {
            println!("{0}", c.val);
        }
    }
}

pub struct Hand
{
    pub cards: Vec<Card>,
    pub value: usize,
    pub busted: bool,
}

impl Hand
{
    pub fn new() -> Hand
    {
        Hand {
            cards: Vec::new(),
            value: 0,
            busted: false,
        }
    }

    pub fn draw_cards(&mut self, card_count: usize, deck: &mut Deck)
    {
        if deck.cards.len() < card_count
        {
            deck.shuffle();
        }

        for _ in 0..card_count
        {
            let drawn_card: Card = deck.cards[0];
            self.cards.push(drawn_card);
            deck.cards.remove(0);
        }
    }

    pub fn render_hand(&self, show_full: bool)
    {
        if show_full
        {
            for card in &self.cards {
                card.render();
                print!(" ");
            }
        }else {
            self.cards[0].render();
        }
    }

    pub fn calculate_hand(&mut self)
    {
        let mut has_ace: bool = false;

        self.value = 0;
        for card in &self.cards {
            if card.val == 1
            {
                has_ace = true;
            }
            
            self.value += card.val.clamp(1, 10);
        }

        if has_ace && self.value <= 11
        {
            self.value += 10;
        }
    }
}