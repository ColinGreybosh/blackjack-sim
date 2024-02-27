use deckofcards::{Card, Cards, Deck};

fn get_shoe(number_of_decks: usize) -> Deck {
    let all_cards = Card::all_cards().repeat(number_of_decks).to_owned();
    let mut shoe = Deck::from_cards(&all_cards);
    shoe.shuffle();
    shoe
}

fn main() {
    let number_of_decks = 6;
    let mut shoe = get_shoe(number_of_decks);
    for i in 0..10 {
        println!("Shoe {}", i + 1);
        shoe.deal(shoe.undealt_count()).iter().map(|&x| x.to_string()).for_each(|x| println!("{}", x));
        shoe.reset_shuffle();
    }
}
