/// A game room holds gamesessions and conducts each instance of hi-lo
/// Server will manage the instances of game rooms, and also create / destroy them
/// 
/// A game room is an actor that contains references to each of the sessions / players
/// that it contains.
use deckofcards::{Cards, Deck};

use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};
use std::collections::{HashMap, HashSet};

use crate::server::Message;

pub struct GameRoom {
    pub id: usize,
    pub sessions: HashMap<usize, Recipient<Message>>,
    pub deck: Deck,
    pub firstRoundGuess: HashMap<usize, usize>,
    pub secondRoundGuess: HashMap<usize, String>,
}

// need to handle someone joining the room; update their information - this is done in the server mod
impl Default for GameRoom {
    fn default() -> GameRoom {
        GameRoom {
            id: 0,
            sessions: HashMap::new(),
            deck: Deck::new(),
            firstRoundGuess: HashMap::new(),
            secondRoundGuess: HashMap::new(),
        }
    }
}

impl Actor for GameRoom {
    type Context = Context<Self>;
}

impl GameRoom {

    async fn startGame(&mut self) {
        // decide on the order for the players
        self.firstRound()
            .await;

        self.secondRound()
            .await;


    }

    // should return OK if everything went through, otherwise should return fail, or game restart.
    async fn firstRound(&mut self) {

    }

    // should return OK if everything went through, otherwise should return fail, or game restart.
    async fn secondRound(&mut self) {

    }

    fn score() {

    }
}

