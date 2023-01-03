mod card;
mod player;
mod schedule;
mod board;

pub mod main {
    /* Rust imports */
    use core::panic;
    use std::io;

    /* Crate Imports */
    use super::card::Card;
    use super::player::Player;
    use super::schedule::Schedule;
    use super::board::Board;
    
    /* Main Struct of the Game */
    pub struct Game {
        pub players: Vec<Player>,
        pub deck: Vec<Card>,
        pub board: Board,  // size 3
        pub hidden_cards: Vec<Card>, // size 2
        pub schedule: Schedule
    }

    /* Implementation of the Game struct */
    impl Game {
        /* Game Constructor */
        pub fn new() -> Self {
            println!("Game Commencing...");
            Game {
                players: vec![],
                hidden_cards: vec![],
                deck: super::card::fill_cards(),
                board: Board::new(),
                schedule: Schedule::new()
            }
        }

        /* Beginning of the game */
        pub fn start_game(&mut self) -> () {
            if !self.room_is_filled() {
                panic!("Not enough players\n {}", self.players.len());
            }
            self.setup();
            self.update();
        }

        pub fn setup(&mut self) -> () {
            self.deal_cards();
            self.let_player_hide_cards();
            self.print_hidden_cards();
            self.print_all_hands();
        }

        pub fn update(&mut self) -> () {
            /* Game Loop */
            loop {
                Self::print_title(&self.schedule.get_game().to_string().to_owned());
                self.play_round();
                self.print_scores();
                self.schedule.next_game();
                self.setup();
            } // End of the loop
        }

        pub fn play_round(&mut self) -> () {
            self.board.clean();
            let mut turn: usize = 0; /* Tracks turns */
            let mut gone_cards: Vec<Card> = vec![]; /* Tracks gone cards */
            let mut current_suit: char = '0'; /* Tracks current suit */
            let mut trump: char = '0';

            if self.schedule.get_game() == '+' {
                trump = self.plus_game_setup();
            }

            /* Round loop */
            loop {
                /* Handle if player has not got any cards */
                if self.players[turn].hand.len() == 0 {
                    break;
                }

                /* Add drawn card and its owner to board hashmap */
                let mut drawn_card: Card = self.players[turn].handle_player_turn();

                /* Assign current suit */
                if self.board.cards_on_board.len() == 0 {
                    current_suit = drawn_card.suit;
                }

                /* Check if drawn card is legal */
                while self.is_illegal_king_move(&drawn_card, &turn) {
                    println!("Illegal move! Try again");
                    self.players[turn].print_hand();
                    drawn_card = self.players[turn].handle_player_turn();
                }
                
                /* Case during trump card */
                if trump != '0' {
                    /* 
                        If current suit is not the same as drawn one and drawn suit is trump
                        holds at least one card with current suit, it means that player used
                        trump card illegally and we need to prevent it
                        
                        If drawn card's suit is not the same as the current one and player is holding
                        trump, it is illegal move and we need to force player to draw trump card
                    */    
                    while   (current_suit != drawn_card.suit &&
                            drawn_card.suit == trump &&
                            self.contains_specific_suit(
                                &self.players[turn].hand.clone(),
                                current_suit
                            )) 
                            ||
                            (current_suit != drawn_card.suit &&
                                drawn_card.suit != trump &&
                                self.contains_specific_suit(
                                    &self.players[turn].hand.clone(),
                                    trump
                            ))
                    {
                        println!("Illegal move! Try again");
                        self.players[turn].print_hand();
                        drawn_card = self.players[turn].handle_player_turn();
                    }

                }

                /*
                    If first suit of the first card on the board is not the same as players drawn card
                    and player has the card of the suit, then it is an illegal move and let player draw again
                */
                while current_suit != drawn_card.suit && self.contains_specific_suit(&self.players[turn].hand.clone(), current_suit) {
                    println!("Illegal move! Try again");
                    self.players[turn].print_hand();
                    drawn_card = self.players[turn].handle_player_turn();
                }

                /* Remove from players' hand */
                self.players[turn].draw_card(drawn_card); 
                
                /* Update HashMap */
                self.board.cards.insert(drawn_card, self.players[turn].clone());
                
                /* Update array*/
                self.board.cards_on_board.push(drawn_card);

                /* Update gone cards */
                gone_cards.push(drawn_card);

                match self.board.cards_on_board.len() {
                    3 => {
                        /* Case when there are 3 cards on the board */
                        let biggest_card: &Card = &self.board.handle_board(trump);
                        let taken_cards: Vec<Card> = self.board.get_cards_on_board();
                        println!("Cards goes to {:?}", self.board.cards[biggest_card].username);
                        
                        let is_last_two = self.is_last_two();

                        /*  Get final score of current type of game
                            If it is NOT -1, then that means special cards occured on the board 
                        */
                        let final_score: i32 = self.board.handle_current_game(
                            self.schedule.get_game(),
                            &taken_cards,
                            is_last_two
                        );

                        /* Handle user scores */
                        if final_score != -1 {
                            let round_has_ended = self.handle_specific_play(final_score, &biggest_card, &gone_cards);
                            if round_has_ended {
                                break;
                            }
                        }

                        /* Handle changing turn value to the proper one */
                        turn = self.players
                        .iter()
                        .position(|p| p.username == self.board.cards[biggest_card].username)
                        .unwrap();

                        self.print_all_hands();

                        current_suit = '0';
                        /* Clean up board */
                        self.board.clean();
                    },
                    1 | 2 => {
                        /* Case if only 1 or 2 players layed down their card */
                        match turn {
                            2 => { turn = 0; },
                            0 | 1 => { turn += 1; },
                            _ => { panic!("Panicked during checking turn"); }
                        }
                    },
                    _ => {
                        panic!("Panicked during checking cards.board length LENGTH: {}", self.board.cards.len());
                    }
                }
            } // End of loop
            println!("Round has ended");
        }

        pub fn plus_game_setup(&mut self) -> char {
            let mut trump_announcement: String = String::new();
            let announcer_idx = self.players
            .iter()
            .position(|p| p.is_his_play == true)
            .unwrap();

            let announcer = &self.players[announcer_idx];

            println!("{}, choose trumps cards", announcer.username);

            io::stdin().read_line(&mut trump_announcement).expect("Failed to read trump");
            
            let trump_announcement: char = match trump_announcement.trim().parse() {
                Ok(ch) => ch,
                Err(_) => panic!("Error during reading trumps card")
            };

            if !vec!['H','D','C','S'].contains(&trump_announcement) {
                panic!("Invalid Input")
            }

            trump_announcement
        }

        pub fn handle_specific_play(&mut self, final_score: i32, biggest_card: &Card, gone_cards: &Vec<Card>) -> bool {
            let p_idx = self.players
                .iter()
                .position(|p| p.username == self.board.cards[biggest_card].username)
                .unwrap();

                self.players[p_idx].score += final_score;

                match self.schedule.get_game() {
                    'K' => return true,
                    'Q' => {
                        let counter: u8 = self.count_specific_card(gone_cards, 'Q');
                        println!("Gone queens: {}", counter);
                        if counter == 4 { return true; }
                    },
                    'J' => {
                        let counter: u8 = self.count_specific_card(gone_cards, 'J');
                        if counter == 4 { return true; }
                    },
                    '2' => {
                        println!("Last two has started!");
                    },
                    '+' => {
                        println!("Plus Score goes to {}", self.players[p_idx].username);
                    },
                    _ => { println!("Another game") }
                };
                false
        }

        pub fn contains_specific_suit(&mut self, hand: &Vec<Card>, suit: char) -> bool {
            for c in hand {
                if c.suit == suit {
                    return true;
                }
            }
            false
        }

        pub fn count_specific_card(&mut self, gone_cards: &Vec<Card>, ch: char) -> u8 {
            let mut counter = 0;
            for c in gone_cards {
                if c.rank.to_string() == ch.to_string() {
                    counter += 1;
                }
            }
            counter
        }

        /* 
            If it is kings game and player draw King Heart on the firstly,
            that is illegal move and we need to prevent it.
        */
        pub fn is_illegal_king_move(&mut self, drawn_card: &Card, turn: &usize) -> bool {
            if
                self.board.cards_on_board.len() == 0 &&
                self.schedule.get_game() == 'K' &&
                drawn_card.suit == 'H' &&
                drawn_card.rank == "K" &&
                self.players[*turn].hand.iter().position(|c| c.suit != 'H') != None
                {
                    return true;
                }
            false
        }

        /* Shuffle and deal cards */
        pub fn deal_cards(&mut self) -> () {
            /* Shuffle cards and deal them to players */
            let shuffled: [Vec<Card>; 3] = super::card::shuffle(&self.deck);
            
            for i in 0..3 {
                self.players[i].hand = shuffled[i].clone();
            }
        }

        /* Let the main player choose 2 cards to hide */
        pub fn let_player_hide_cards(&mut self) -> () {
            let main_player = self.players
            .iter()
            .position(|c| c.is_his_play == true)
            .unwrap();
            self.players[main_player].print_hand();
            
            for _i in 0..2 {
                let mut choice: String = String::new();
                io::stdin().read_line(&mut choice).expect("Failed to read choice");
                /* Validate user choice */
                let choice: usize = match choice.trim().parse() {
                    Ok(num) => num,
                    Err(_) => panic!("Invalid input, Try again"),
                };

                if choice >= self.players[main_player].hand.len() {
                    panic!("Invalid input");
                }

                /* Handle Removing */
                /* Get the card that user wants to draw */
                let card_to_remove = self.players[main_player].hand[choice];

                /* Prohibit removing some cards */
                match self.schedule.get_game() {
                    'K' => {
                        if card_to_remove.suit == 'H' && card_to_remove.rank == "K" {
                            println!("You cant remove King Heart");
                            self.let_player_hide_cards();
                        }
                    },
                    'Q' => {
                        if card_to_remove.rank == "Q" {
                            println!("You cant remove Queen");
                            self.let_player_hide_cards();
                        }
                    },
                    'J' => {
                        if card_to_remove.rank == "J" {
                            println!("You cant remove Jack");
                            self.let_player_hide_cards();
                        }
                    },
                    'H' => {
                        if card_to_remove.suit == 'H' {
                            println!("You cant remove Heart");
                            self.let_player_hide_cards();
                        }
                    },
                    _ => {}
                }

                /* Print and remove the card from the hand array */
                self.players[main_player].draw_card(card_to_remove);
                
                /* Add removed cards in hidden cards */
                self.hidden_cards.push(card_to_remove);
            }
        }

        pub fn is_last_two(&mut self) -> bool {
            self.players[2].hand.len() < 2
        }

        /* Handle adding new player */
        pub fn new_player(&mut self, username: String, is_his_play: bool) -> () {
            if self.players.len() >= 3 {
                panic!("Room has already been filled.");
            } else {
                self.players.push(Player::new(username, is_his_play));
            }
        }

        fn room_is_filled(&mut self) -> bool { 
            &self.players.len() == &3
        }

        /* Handle print functions*/
        pub fn print_title(text: &str) {
            println!("<-------------------{}------------------->", text);
        }

        pub fn print_players(&mut self) -> () {
            Self::print_title("Players");
            for i in &self.players {
                print!("<-{}-> ", i.username);
            }
            println!();
        }

        pub fn print_all_hands(&mut self) -> () {
            Self::print_title("All Hands");
            self.players[0].print_hand();
            self.players[1].print_hand();
            self.players[2].print_hand();
        }

        pub fn print_hidden_cards(&mut self) -> () {
            Self::print_title("Hidden Cards");
            for i in &self.hidden_cards {
                println!("{:?}", i);
            }
            println!("__________________________________________\n");
        }

        pub fn print_scores(&mut self) -> () {
            Self::print_title("Scores");
            println!("{}: {}", self.players[0].username, self.players[0].score);
            println!("{}: {}", self.players[1].username, self.players[1].score);
            println!("{}: {}", self.players[2].username, self.players[2].score);
        }

    }
}