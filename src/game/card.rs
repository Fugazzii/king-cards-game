extern crate rand;

use rand::Rng;

const SUITS: [char; 4] = ['H', 'D', 'S', 'C'];
const RANKS: [&str; 8] = ["7", "8", "9", "10", "J", "Q", "K", "A"];

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Card {
    pub suit: char,
    pub rank: &'static str,
}

impl Card {
    pub fn new(suit: char, rank: &'static str) -> Self {
        Card { suit, rank }
    }

    pub fn get_rank_idx(&mut self) -> usize {
        RANKS
        .iter()
        .position(|r| *r == self.rank)
        .unwrap()
    }
}

pub fn fill_cards() -> Vec<Card> {
    let mut deck: Vec<Card> = vec![];
    for suit in SUITS {
        for rank in RANKS {
            deck.push(Card::new(suit, rank));
        }
    }
    deck
}

pub fn shuffle(d: &Vec<Card>) -> [Vec<Card>; 3] {
    /* Clone our deck into a new variable to avoid changing original deck */
    let mut deck = d.clone();

    /* Make 3-length array of vectors to handle cards for each player */
    let mut shuffled: [Vec<Card>; 3] = [vec![], vec![], vec![]];

    for _i in 0..10 {
        for j in 0..3 {
            /* Handle Randomizing Cards */
            let mut rng = rand::thread_rng();
            let random_number = rng.gen_range(0..deck.len());
            
            /* Take value from the vector and return it */
            let random_card = deck.remove(random_number);
            shuffled[j].push(random_card);
        }
    }

    /* Handle additional two cards for the players that is the main in a round */
    shuffled[0].push(deck[0]);
    shuffled[0].push(deck[1]);

    shuffled
}
