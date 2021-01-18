#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;

//use diesel::prelude::*;
//use diesel::pg::PgConnection;

//use std::error::Error;
//use serde::Deserialize;

mod db;
mod personas;
mod error_handler;
mod schema;

//leer e insertar desde csv
/*fn read_csv(path: &str, conn: &PgConnection) -> Result<(), Box<dyn Error>>{
    
    //crear lector de csv
    let mut reader = csv::ReaderBuilder::new().delimiter(b';').has_headers(false).from_path(path)?;

    for result in reader.records(){
        let record = result?;
        
        let persona = models::NewPersona {
            identificacion: record[0].to_string(),
            nombre: record[1].to_string(),
            genero: record[2].to_string(),
            estadocivil: record[3].to_string(),
            fechanacimiento: record[4].to_string(), 
            telefono: record[5].to_string(),
            direccion: record[6].to_string(),
            email: record[7].to_string(),
            validado: true,
            observacion: String::from("N/A"),
        };

        if models::Persona::insert(persona, &conn) {
            println!("success");
        } else {
            println!("failed");
        }
    }

    Ok(())
}*/

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
    
    /*if let Err(e) = read_csv("files/personas.csv", &conn){
        eprintln!("{}",e);
    }*/

    /*dotenv().ok();

    let database_url = "postgres://postgres:1234@localhost/migrar";
    let conn = PgConnection::establish(&database_url).unwrap();

    let persona = models::NewPersona {
        identificacion: String::from("1723301246"),
        nombre: String::from("Paula Monteros"),
        genero: String::from("F"),
        estadocivil: String::from("SOLTERO"),
        fechanacimiento: String::from("1997-07-27"), 
        telefono: String::from("0987614578"),
        direccion: String::from("El condado"),
        email: String::from("ma.paulamonteros@gmail.com"),
        validado: true,
        observacion: String::from("N/A"),
    };

    if models::Persona::insert(persona, &conn) {
        println!("success");
    } else {
        println!("failed");
    }*/
    
}
