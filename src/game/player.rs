use crate::game::card::Card;
use std::io;

#[derive(Clone, Debug)]
pub struct Player {
    pub username: String,
    pub is_his_play: bool, // Determine if it is his play meaning in King's rules
    pub hand: Vec<Card>, // Size 10-12
    pub score: i32
}

impl Player {
    pub fn new(username: String, is_his_play: bool) -> Self {
        Player { 
            username,
            is_his_play,
            hand: vec![],
            score: 0 
        }
    }

    pub fn draw_card(&mut self, drawn_card: Card) -> () {
        let card_idx = self.hand
        .iter()
        .position(|c| c.rank == drawn_card.rank && c.suit == drawn_card.suit)
        .unwrap();
        
        println!("Drawn {:?}", self.hand[card_idx]);
        self.hand.remove(card_idx);
    }

    pub fn handle_player_turn(&mut self) -> Card {
        println!("It's {} turn", self.username);
        let mut choice: String = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read user choice");
        
        let choice: usize = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("Error occured while getting user choice"),
        };

        /* Handle invalid input */
        if choice >= self.hand.len() {
            panic!("Invalid input");
        }

        let card_to_draw: Card = self.hand[choice];
        //self.draw_card(self.hand[choice]);
        card_to_draw
}

    pub fn print_hand(&mut self) -> () {
        println!("Player {} hand: ", self.username);
        for i in &self.hand {
            println!("{:?}", i);
        }
    }
}