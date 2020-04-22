use deckofcards::{Cards, Deck};

mod server;
mod gamesession;
mod gameroom;

// Examples: https://covidopoly.io/waiting-room

/*
* Probably should create a struct for player 
* struct for game
* the main function should just initiate the players and game
*/



fn main() {
    println!("Welcome to HiLo!");

    //prep the game
    let mut deck = Deck::new();
    deck.shuffle();

    const NUM_PLAYERS: usize = 8;

    let dealt_cards = deck.deal(NUM_PLAYERS);

    // calculate rank order of the cards

    // need some vector to hold player guesses


    // Instantiate players



    //first round
    let mut first_round_guesses: [i32; NUM_PLAYERS] = [0; NUM_PLAYERS];


    //second round
    // do we want type to be string (for rank) 
    let mut second_round_guesses: [char; NUM_PLAYERS] = ['2'; NUM_PLAYERS];
    let mut group_score = 0;

    //print out
    for _ in 0..NUM_PLAYERS {

    	// take in player guesses
    	group_score += 1;

    };

    //output score

    deck.reset()
}
