use crate::personas::{Persona, NewPersona};
use crate::error_handler::CustomError;

use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

use std::error::Error;
use serde::Deserialize;

#[get("/personas")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let personas = Persona::find_all()?;
    Ok(HttpResponse::Ok().json(personas))
}

fn read_csv_insert(path: &str) -> Result<(), Box<dyn Error>>{
    
    //crear lector de csv
    let mut reader = csv::ReaderBuilder::new().delimiter(b';').has_headers(false).from_path(path)?;

    for result in reader.records(){
        let record = result?;
        
        let persona = NewPersona {
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

        let insert_persona = Persona::insert(persona);

    }

    Ok(())
}

#[get("/insert-personas-csv")]
async fn create() -> Result<HttpResponse, CustomError> {
    if let Err(e) = read_csv_insert("files/personas.csv"){
        eprintln!("{}",e);
    }
    Ok(HttpResponse::Ok().json("OK"))
}

pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(create);
}

