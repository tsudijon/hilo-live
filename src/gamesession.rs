/*
* Implements the player actors; these are handled by the server
*
*
* Implements StreamHandler to take in information from the player.
*/

use std::time::{Duration, Instant};
use actix::*;
use actix_files as fs;
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

    // holds heartbeat to check if player is still connected (Instant is a point in time)
    hb: Instant,

    // holds the game room that is joined
    room: String,

    // peer name? Is this username? Option returns an enum with Some or None
    name: Option<String>,
    
    // has reference to current card in hand ? Or can let the server take care of this. Needs a referece to the server however
    addr: Addr<server::GameServer>,
}

impl Actor for GameSession {
    type Context = ws::WebsocketContext<Self>;

    // Method is called when gamesession spun up; register with the gameserver
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx); // this is the method, not the field; confusing?

        // register with the gameserver (gotten via the addr field)
        // there's two addr variables here, which is confusing
        let addr = ctx.address();
        self.addr
            .send(server::Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);

         

    }
}

impl Handler<server::Message> for GameSession {
    type Result = ();

    fn handle(&mut self, msg: server::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }

}

impl GameSession {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            
            if Instant::now().duration_since(act.hb) > Duration::new(10, 0) {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // How do we get a disconnected actor to rejoin the same game?
                act.addr.do_send(server::Disconnect {id: act.id});

                ctx.stop();

                return;
            }

            ctx.ping(b"");
        });
    }
}

