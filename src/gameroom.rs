/// A game room holds gamesessions and conducts each instance of hi-lo
/// Server will manage the instances of game rooms, and also create / destroy them
/// 
/// A game room is an actor that contains references to each of the sessions / players
/// that it contains.
use deckofcards::{Card, Cards, Deck};

use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};
use std::collections::{HashMap};

use crate::server::Message;

pub struct GameRoom {
    pub id: usize,
    pub sessions: HashMap<usize, Recipient<Message>>,
    pub playerCards: HashMap<usize, Card>,
    pub firstRoundGuess: HashMap<usize, usize>,
    pub secondRoundGuess: HashMap<usize, String>,
    //pub sessionOrder: Vec<usize>,
}

// need to handle someone joining the room; update their information - this is done in the server mod
impl Default for GameRoom {
    fn default() -> GameRoom {
        GameRoom {
            id: 0,
            sessions: HashMap::new(),
            playerCards: HashMap::new(),
            firstRoundGuess: HashMap::new(),
            secondRoundGuess: HashMap::new(),
        }
    }
}

impl Actor for GameRoom {
    type Context = Context<Self>;
}

impl GameRoom {

    // function to get data from a player; needs to be async?
    async fn poll_session(&mut self, sessionId: usize) {

    }

    // tensor programming had a function in which you can use the await function in a non async function? In webcrawler video.

    fn start_game(&mut self) {
        // decide on the order for the players

        // deal the cards, update the player cards
        let mut deck = Deck::new();

        // need to import the cards trait to make this work.
        deck.shuffle();
        for id in self.sessions.keys() {
            self.playerCards.insert(
                *id, //* dereferences?
                deck.deal_one().unwrap(),
            );
        }

        // reveal duplicates
        // push method to all the actors.

        // send messages to the sessions about the other cards.
        // I think just a do_send to the recipient will do.

        /// match on output of firstRound
        /// We need to 
        self.first_round();

        self.second_round();


    }

    // should return OK if everything went through, otherwise should return fail, or game restart.
    // I don't think these need to be async functions since they run serially no concurrently.
    fn first_round(&mut self) {

    }

    // should return OK if everything went through, otherwise should return fail, or game restart.
    fn second_round(&mut self) {

    }

    fn score() {

    }
}

