/*
* Implements the server as an actor. This manages the gamesessions and their playing states.
* 
* Might want to create another actor that serves as 'dealer' for a single game.
*
*/
use deckofcards::{Cards, Deck};

use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};
use std::collections::{HashMap, HashSet};

/// might want to create two types of messages, one for order guess and another for rank guess. For now can accept strings and 
/// error handle as need be.

#[derive(Message)]
#[rtype(result="()")]
pub struct Message(pub String);


pub struct GameServer {
    sessions: HashMap<usize, Recipient<Message>>,
    rooms: HashMap<usize, HashSet<usize>>,
    rng: ThreadRng,
}

impl Actor for GameServer {
    /// Just need simple capability to communicate with the other actors.
    type Context = Context<Self>;
}