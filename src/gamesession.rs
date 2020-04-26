/*
* Implements the player actors; these are handled by the server
*
*
* Implements StreamHandler to take in information
*/

use std::time::{Duration, Instant};
use actix::*;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

use crate::server;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

struct GameSession {
    // holds a unique player uuid
    id: usize,

    // holds heartbeat to check if player is still connected
    hb: Instant,

    // holds the game room that is joined
    room: String,

    // peer name? Is this username? Option returns an enum with Some or None
    name: Option<String>,
    
    // has reference to current card in hand ? Or can let the server take care of this. Needs a referece to the server however
    addr: Addr<server::GameServer>,
}

