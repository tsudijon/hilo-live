use deckofcards::{Cards, Deck};

mod server;
mod gamesession;
mod gameroom;

use std::time::{Duration, Instant};
use actix::*;
use actix_files as fs;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::sync::{Arc};

// Examples: https://covidopoly.io/waiting-room

/*
* Probably should create a struct for player 
* struct for game
* the main function should just initiate the players and game
*
* cargo run --bin hilo-live
*/

async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::GameServer>>,
    //gameroom: web::Data<Addr<gameroom::GameRoom>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        gamesession::GameSession {
            id: 0,
            hb: Instant::now(),
            room: "Main".to_owned(),
            name: None,
            addr: srv.get_ref().clone(),
            //room_addr: gameroom.get_ref().clone(),
        },
        &req,
        stream,
    )
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // Start chat server actor
    let server = server::GameServer::default().start();


    // Create Http server with websocket support
    HttpServer::new(move || {
        App::new()
            .data(server.clone())
            // redirect to websocket.html
            .service(
                web::resource("/")
                    .route(
                        web::get().to(|| {
                        HttpResponse::Found()
                            .header("LOCATION", "/static/websocket.html")
                            .finish()
            })))
            // websocket
            .service(web::resource("/ws/").to(chat_route))
            // static resources
            .service(fs::Files::new("/static/", "static/"))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}