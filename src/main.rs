#[macro_use]
extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;

mod schema;
mod models;

fn main() {
    dotenv().ok();

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
    }
    
}
