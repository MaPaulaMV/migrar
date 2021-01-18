use crate::db;
use crate::error_handler::CustomError;
use crate::schema::persona;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "persona"]
pub struct Persona{
    pub codigo: i32,
    pub identificacion: String,
    pub nombre: String,
    pub genero: String,
    pub estadocivil: String,
    pub fechanacimiento: String, 
    pub telefono: String,
    pub direccion: String,
    pub email: String,
    pub validado: bool,
    pub observacion: String,
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "persona"]
pub struct NewPersona {
    pub identificacion: String,
    pub nombre: String,
    pub genero: String,
    pub estadocivil: String,
    pub fechanacimiento: String, 
    pub telefono: String,
    pub direccion: String,
    pub email: String,
    pub validado: bool,
    pub observacion: String,
}

impl Persona {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let personas = persona::table.load::<Persona>(&conn)?;
        Ok(personas)
    }

    pub fn insert(newpersona: NewPersona) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let persona = NewPersona::from(newpersona);
        let persona = diesel::insert_into(persona::table)
            .values(persona)
            .get_result(&conn)?;
        Ok(persona)
    }
}

impl NewPersona{
    fn from(persona: NewPersona) -> NewPersona {
        NewPersona {
            identificacion: persona.identificacion,
            nombre: persona.nombre,
            genero: persona.genero,
            estadocivil: persona.estadocivil,
            fechanacimiento: persona.fechanacimiento, 
            telefono: persona.telefono,
            direccion: persona.direccion,
            email: persona.email,
            validado: persona.validado,
            observacion: persona.observacion,
        }
    }
}