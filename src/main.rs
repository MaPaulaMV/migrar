#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;

mod db;
mod personas;
mod error_handler;
mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    dotenv().ok();
    db::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new().configure(personas::init_routes));

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = "127.0.0.1";
            let port = "5000";
            server.bind(format!("{}:{}", host, port))?
        }
    };

    server.run().await
    
}
