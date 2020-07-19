/*
* Implements the server as an actor. This manages the gamesessions and their playing states.
* 
* Might want to create another actor that serves as 'dealer' for a single game.
* Allow people to join the room until someone starts the game.
*/
use deckofcards::{Cards, Deck};

use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};
use std::collections::{HashMap, HashSet};


use crate::gameroom::{GameRoom, GameMessage};

/// might want to create two types of messages, one for order guess and another for rank guess. For now can accept strings and 
/// error handle as need be.

#[derive(Message)]
#[rtype(result="()")]
pub struct Message(pub String);

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ServerMessage {
    // Id of the client session
    pub id: usize,
    pub msg: String,
    pub room: String,
    // should we replace this with the gameroom struct? At least gameroom id,
    // and have the server send the message.
}




pub struct GameServer {
    sessions: HashMap<usize, Recipient<Message>>,
    rooms: HashMap<String, HashSet<usize>>,
    room_addresses: HashMap<String, Recipient<GameMessage>>,
    rng: ThreadRng,
}

// how to organize rust code with actix? 
impl Actor for GameServer {
    /// Just need simple capability to communicate with the other actors.
    type Context = Context<Self>;
}

// to_owned converts to a string?
impl GameServer {
    /// send message to all users in the room (this is horrendous, need to create
    /// some type of room struct, and just send the message out that way? )
    fn send_message(&self, room: &str, message: &str, skip_id: usize) {
        if let Some(sessions) = self.rooms.get(room) {
            for id in sessions {
                if *id != skip_id {
                    if let Some(addr) = self.sessions.get(id) {
                        let _ =addr.do_send(Message(message.to_owned()));
                    }
                }
            }
        }
    }

    fn game_command(&self, room: &str, command: &str) {
        // implement case handling
        match command.trim() {
            "start_game" => {
                let addr = self.room_addresses.get(room).unwrap();
                addr.do_send(GameMessage{ msg: "start_game".to_owned(), id:0});
            }
            _ => println!("Invalid Game Command")
        }
        
                  
    }

}

impl Default for GameServer  {
    fn default() -> GameServer {

        let mut rooms = HashMap::new();
        let mut room_addresses = HashMap::new();

        rooms.insert("Main".to_owned(), HashSet::new());
        room_addresses.insert("Main".to_owned(), GameRoom::default().start().recipient());

        // need to put these two objects toegether, or store all players into a room.

        GameServer {
            sessions: HashMap::new(),
            rooms,
            room_addresses,
            rng: rand::thread_rng()
        }
    }
}


// implement default method (instantiates the object), starts the  game
impl Handler<Connect> for GameServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        println!("Someone joined");

        // notify all users in same room
        self.send_message(&"Main".to_owned(), "Someone joined", 0);

        // register session with random id
        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, msg.addr);

        // auto join session to Main room
        self.rooms
            .entry("Main".to_owned())
            .or_insert(HashSet::new())
            .insert(id);

        // send id back
        id
    }
}

impl Handler<ServerMessage> for GameServer {
    type Result = ();

    // need to check whether the message is a game command.
    fn handle(&mut self, msg: ServerMessage, _: &mut Context<Self>) {
        let m = msg.msg;

        if m.starts_with("/command ") {
            let command = m.split("/command").collect::<Vec<_>>()[1];
            self.game_command(&msg.room, command);
            
        } else {
            self.send_message(&msg.room, m.as_str(), msg.id);
        }
    }
}


impl Handler<Disconnect> for GameServer {
    type Result = ();

    // remove the uid from the map of sessions -- have to iterate through the rooms?
    // that's really fucking bad
    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("Someone disconnected");

        let mut room = String::new();

        //remove address; currently assume player is in only one game.
        if self.sessions.remove(&msg.id).is_some() {
            // remove session from all rooms; can't we just query if name is in hashmap?
            for (name, sessions) in &mut self.rooms {
                if sessions.remove(&msg.id){
                    room = name.to_owned();
                    break;
                }
            }
        }

        /// can get their user id / name here -- need to make Disconnect message 
        /// store the user name
        self.send_message(&room, "Someone disconnected", 0);



    }
}







