#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}

fn main() {
    // List of 'suit' - 'hearts', 'spades'
    let suits: [&str; 2] = ["Hearts", "Spades"];

    // List of 'values' 'ace', 'two', 'three'
    let values: [&str; 3] = ["Ace", "Two", "Three"];

    // Double nested for loop

    let deck: Deck = Deck { cards: vec![] };

    println!("Heres your deck: {:?}", deck);
}
