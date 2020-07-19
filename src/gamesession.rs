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
use crate::gameroom;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout 
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);



pub struct GameSession {
    // holds a unique player uuid
    pub id: usize,

    // holds heartbeat to check if player is still connected (Instant is a point in time)
    pub hb: Instant,

    // holds the game room that is joined
    pub room: String,

    // peer name? Is this username? Option returns an enum with Some or None
    pub name: Option<String>,
    
    // has reference to current card in hand ? Or can let the server take care of this. Needs a referece to the server however
    pub addr: Addr<server::GameServer>,

    //pub room_addr: Addr<gameroom::GameRoom>,
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

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify chat server
        self.addr.do_send(server::Disconnect { id: self.id });
        Running::Stop
    }
}

impl Handler<server::Message> for GameSession {
    type Result = ();

    fn handle(&mut self, msg: server::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }

}

/// Websocket message handler. How do we scale to having many actions / buttons to use / press?
/// Seems unwieldy to program so many of them. Is there a button push  Message? How do we differentiate between server messages and game messages?
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for GameSession {
    fn handle (
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        // handle potential error
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg
        };

        println!("Websocket Message: {:?}", msg);

        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }

            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }

            ws::Message::Text(text) => {
                let m = text.trim();


                let msg = if let Some(ref name) = self.name {
                    format!("{}: {}", name, m)
                } else {
                    m.to_owned()
                };

                self.addr.do_send(server::ServerMessage {
                    id:  self.id,
                    msg,
                    room: self.room.clone(),
                })
                


            }

            ws::Message::Binary(_) => println!("Unexpected binary"),
            ws::Message::Close(_) => {
                ctx.stop(); // what does stopping an actor do?
            }
            ws::Message::Continuation(_) => {
                ctx.stop(); // what does stopping an actor do?
            }
            ws::Message::Nop => ()
        }
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



