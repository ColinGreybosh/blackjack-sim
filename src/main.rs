use std::collections::HashMap;

use deckofcards::{Card, Cards, Deck, Hand};

const CASINO_BANKROLL_CENTS: i64 = 1_000_000 * 100;
const PLAYER_BANKROLL_CENTS: i64 = 1_000 * 100;
const NUMBER_OF_DECKS: usize = 6;
const DEALER_ID: i64 = 0xDEA1E4;

struct Table {
    player_hand_map: HashMap<PlayerId, (Option<Hand>, Option<Hand>)>,
    player_wager_map: HashMap<PlayerId, (i64, i64)>,
    shoe: Deck,
}

impl Table {
    fn new_table(players: Vec<Player>) -> Table {
        let mut player_hand_map = HashMap::new();
        let mut player_wager_map = HashMap::new();
        let dealer = Player::new_dealer(DEALER_ID, CASINO_BANKROLL_CENTS);
        let dealer_hands = (Some(Hand::new()), None);
        player_hand_map.insert(dealer.id, dealer_hands);
        for player in players {
            let player_hands = (Some(Hand::new()), Some(Hand::new()));
            player_hand_map.insert(player.id, player_hands);
            player_wager_map.insert(player.id, (0, 0));
        }
        Table {
            player_hand_map,
            player_wager_map,
            shoe: get_shoe(NUMBER_OF_DECKS),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]

struct PlayerId(i64);

impl PlayerId {
    fn from_i64(id: i64) -> PlayerId {
        PlayerId(id)
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
enum Strategy {
    Dealer,
    Basic,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Player {
    id: PlayerId,
    bankroll_cents: i64,
    strategy: Strategy,
}

impl Player {
    fn new_dealer(id: i64, bankroll_cents: i64) -> Player {
        Player {
            id: PlayerId::from_i64(id),
            bankroll_cents,
            strategy: Strategy::Dealer,
        }
    }
    fn new_player(id: i64, bankroll_cents: i64, strategy: Strategy) -> Player {
        Player {
            id: PlayerId::from_i64(id),
            bankroll_cents,
            strategy,
        }
    }
}

fn get_shoe(number_of_decks: usize) -> Deck {
    let all_cards = Card::all_cards().repeat(number_of_decks).to_owned();
    let mut shoe = Deck::from_cards(&all_cards);
    shoe.shuffle();
    shoe
}

fn main() {
    let number_of_players = 3;
    let players: Vec<Player> = (0..number_of_players)
        .map(|id| Player::new_player(id, PLAYER_BANKROLL_CENTS, Strategy::Basic))
        .collect();
    let mut table = Table::new_table(players);

    for i in 0..10 {
        println!("Shoe {}", i + 1);
        table
            .shoe
            .deal(table.shoe.undealt_count())
            .iter()
            .map(|&x| x.to_string())
            .for_each(|x| println!("{}", x));
        table.shoe.reset_shuffle();
    }
}
