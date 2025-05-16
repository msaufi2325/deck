use rand::{rng, seq::SliceRandom};

#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        // List of 'suit' - 'hearts', 'spades'
        let suits: [&str; 2] = ["Hearts", "Spades"];

        // List of 'values' 'ace', 'two', 'three'
        let values: [&str; 3] = ["Ace", "Two", "Three"];

        let mut cards: Vec<String> = vec![];

        // Double nested for loop
        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut t_rng = rng();
        self.cards.shuffle(&mut t_rng);
    }    
}

fn main() {
    let mut deck: Deck = Deck::new();

    deck.shuffle();

    println!("Heres your deck: {:#?}", deck);
}
