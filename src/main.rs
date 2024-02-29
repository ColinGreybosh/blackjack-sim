use std::{collections::HashMap, fmt};

use deckofcards::{Card, Cards, Deck, Hand};

const CASINO_BANKROLL_CENTS: u64 = 1_000_000 * 100;
const PLAYER_BANKROLL_CENTS: u64 = 1_000 * 100;
const MINIMUM_BET_CENTS: u64 = 25 * 100;
const NUMBER_OF_DECKS: u64 = 6;
const DEALER_ID: u64 = 0xDEA1E4;

enum State {
    Betting,
    // Dealing,
    // Playing,
}

struct Hands {
    hands: Vec<Hand>,
}

impl Hands {
    fn new() -> Hands {
        Hands { hands: vec![] }
    }
}

struct Game {
    state: State,
    players: Vec<Player>,
    player_hand_map: HashMap<PlayerId, Hands>,
    player_bets_map: HashMap<PlayerId, Bets>,
    shoe: Deck,
}

impl Game {
    fn new_game(number_of_decks: u64, number_of_players: u64) -> Game {
        let players: Vec<Player> = (0..number_of_players)
            .map(|id| Player::new_player(id, PLAYER_BANKROLL_CENTS, Strategy::Basic))
            .collect();
        let shoe = get_shoe(number_of_decks);
        let mut player_hand_map = HashMap::new();
        let mut player_bets_map = HashMap::new();
        let dealer = Player::new_dealer(DEALER_ID, CASINO_BANKROLL_CENTS);
        let dealer_hands = Hands::new();
        player_hand_map.insert(dealer.id, dealer_hands);
        for player in &players {
            let player_hands = Hands::new();
            player_hand_map.insert(player.id, player_hands);
            player_bets_map.insert(player.id, Bets::none());
        }
        Game {
            state: State::Betting,
            players,
            player_hand_map,
            player_bets_map,
            shoe,
        }
    }
    fn update(&mut self) {
        match self.state {
            State::Betting => {
                for player in &mut self.players {
                    let bets = player.get_bets();
                    let bet_total = bets.total();
                    self.player_bets_map.insert(player.id, bets);
                    player.bankroll_cents -= bet_total;
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]

struct PlayerId(u64);

impl PlayerId {
    fn from_u64(id: u64) -> PlayerId {
        PlayerId(id)
    }
}

impl fmt::Display for PlayerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
enum Strategy {
    Dealer,
    Basic,
}

impl Strategy {
    fn get_bets(&self, bankroll_cents: u64) -> Bets {
        match self {
            Strategy::Dealer => todo!(),
            Strategy::Basic => {
                if bankroll_cents > MINIMUM_BET_CENTS {
                    Bets::from_vec(vec![MINIMUM_BET_CENTS])
                } else {
                    Bets::none()
                }
            }
        }
    }
}

struct Bets {
    bets: Vec<u64>,
}

impl Bets {
    fn none() -> Bets {
        Bets { bets: vec![] }
    }
    fn from_vec(bets: Vec<u64>) -> Bets {
        Bets { bets }
    }
    fn total(&self) -> u64 {
        self.bets.iter().sum()
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Player {
    id: PlayerId,
    bankroll_cents: u64,
    strategy: Strategy,
}

impl Player {
    fn new_dealer(id: u64, bankroll_cents: u64) -> Player {
        Player {
            id: PlayerId::from_u64(id),
            bankroll_cents,
            strategy: Strategy::Dealer,
        }
    }
    fn new_player(id: u64, bankroll_cents: u64, strategy: Strategy) -> Player {
        Player {
            id: PlayerId::from_u64(id),
            bankroll_cents,
            strategy,
        }
    }
    fn get_bets(&self) -> Bets {
        self.strategy.get_bets(self.bankroll_cents)
    }
}

fn get_shoe(number_of_decks: u64) -> Deck {
    let all_cards = Card::all_cards()
        .repeat(number_of_decks.try_into().unwrap())
        .to_owned();
    let mut shoe = Deck::from_cards(&all_cards);
    shoe.shuffle();
    shoe
}

fn main() {
    let mut game = Game::new_game(NUMBER_OF_DECKS, 3);
    game.update();
    for (player_id, bets) in game.player_bets_map.iter() {
        let maybe_player = game.players.iter().find(|player| player.id.eq(player_id));
        match maybe_player {
            Some(player) => println!(
                "Player {player_id} has bet {:?} and has {:?} cents left in their bankroll.",
                bets.bets, player.bankroll_cents,
            ),
            None => continue,
        }
    }
    // deal_shoes(game.shoe, 10);
}

fn deal_shoes(mut shoe: Deck, count: u64) {
    for i in 0..count {
        println!("Shoe {}", i + 1);
        shoe.deal(shoe.undealt_count())
            .iter()
            .map(|&x| x.to_string())
            .for_each(|x| println!("{}", x));
        shoe.reset_shuffle();
    }
}
