use std::collections::HashMap;

use crate::game::card::Card;
use crate::game::player::Player;

pub struct Board {
    pub cards: HashMap<Card, Player>,
    pub cards_on_board: Vec<Card>
}

impl Board {
    pub fn new() -> Self {
        Board {
            cards: HashMap::new(),
            cards_on_board: vec![]
        }
    }

    pub fn handle_board(&mut self, trump: char) -> Card {
        let mut c0: Card = self.cards_on_board[0];
        let mut c1: Card = self.cards_on_board[1];
        let mut c2: Card = self.cards_on_board[2];

        let biggest_card: Card = self.compare_cards(
            &mut c0, // current suit
            &mut c1, 
            &mut c2,
            trump 
        );
        println!("Winner card is {:?}", biggest_card);
        biggest_card
    }

    pub fn handle_current_game(&mut self, game_type: char, taken_cards: &Vec<Card>, is_last_two: bool) -> i32 {
        match game_type {
            'K' => self.handle_king_game(taken_cards),
            '2' => self.handle_last_two(is_last_two),
            'Q' => self.handle_queens_or_jacks_game(taken_cards, 'Q'),
            'J' => self.handle_queens_or_jacks_game(taken_cards, 'J'),
            'H' => self.handle_hearts_game(taken_cards),
            'V' => self.handle_vziatk_game(),
            '+' => self.handle_plus_game(),
            _   => panic!("Invalid game type")
        }
    }

    pub fn handle_king_game(&mut self, taken_cards: &Vec<Card>) -> i32 {
        for card in taken_cards {
            if card.suit == 'H' && card.rank == "K" {
                return -40;
            }
        }
        -1 /* Game has not ended yet */
    }

    pub fn handle_last_two(&mut self, is_last_two: bool) -> i32 {
        if is_last_two {
            return -20;
        } else {
            return -1;
        }
    }

    pub fn handle_queens_or_jacks_game(&mut self, taken_cards: &Vec<Card>, game_type: char) -> i32 {
        for card in taken_cards {
            if card.rank.to_string() == game_type.to_string() {
                return -10;
            }
        }
        return -1;
    }

    pub fn handle_hearts_game(&mut self, taken_cards: &Vec<Card>) -> i32 {
        for card in taken_cards {
            if card.suit == 'H' {
                return -5;
            }
        }
        -1
    }

    pub fn handle_vziatk_game(&mut self) -> i32 {
        -4
    }

    pub fn handle_plus_game(&mut self) -> i32 {
        4
    }

    pub fn compare_cards(&mut self, c1: &mut Card, c2: &mut Card, c3: &mut Card, trump: char) -> Card {
        let c1_rank = c1.get_rank_idx();
        let c2_rank = c2.get_rank_idx();
        let c3_rank = c3.get_rank_idx();

        if trump != '0' {
            /* If there is no trump or all of them are trumps */
            if c1.suit == c2.suit && c2.suit == c3.suit {
                if c1_rank > c2_rank && c1_rank > c3_rank {
                    return *c1;
                } else if c2_rank > c1_rank && c2_rank > c3_rank {
                    return *c2;
                } else {
                    return *c3;
                }
            } 
            /* If there is only one trump */
            else if c1.suit == trump && c2.suit != trump && c3.suit != trump { return *c1 }
            else if c2.suit == trump && c1.suit != trump && c3.suit != trump { return *c2 }
            else if c3.suit == trump && c1.suit != trump && c2.suit != trump { return *c3 }

            /* If there are two trumps */
            else if c1.suit == trump && c2.suit == trump && c3.suit != trump { 
                return self.biggest(&c1, &c2);
            }
            else if c1.suit == trump && c3.suit == trump && c2.suit != trump { 
                return self.biggest(&c1, &c3);
            }
            else if c1.suit == trump && c2.suit == trump && c3.suit != trump { 
                return self.biggest(&c1, &c2);
            }
        }

        if c1.suit == c2.suit && c2.suit == c3.suit {
            if c1_rank > c2_rank && c1_rank > c3_rank {
                return *c1;
            } else if c2_rank > c1_rank && c2_rank > c3_rank {
                return *c2;
            } else {
                return *c3;
            }
        } else if c1.suit != c3.suit && c1.suit == c2.suit {
            self.biggest(&c1, &c2)
        }  else if c1.suit != c2.suit && c1.suit == c3.suit {
            self.biggest(&c1, &c3)
        } else {
            return *c1;
        }
    }

    pub fn biggest(&mut self, c1: &Card, c2: &Card) -> Card {
        if c1.rank > c2.rank {
            return *c1;
        } else {
            return *c2;
        }
    }

    pub fn get_cards_on_board(&mut self) -> Vec<Card> {
        let mut result: Vec<Card> = vec![];
        for (key, _value) in &self.cards {
            result.push(*key);
        }
        result
    }

    pub fn clean(&mut self) -> () {
        self.cards.clear();
        self.cards_on_board.clear();
    }
}