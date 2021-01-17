use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use super::schema::persona;
use super::schema::persona::dsl::persona as all_persona;

#[derive(Queryable)]
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

#[derive(Insertable)]
#[table_name = "persona" ]
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
    pub fn show(codigo: i32, conn: &PgConnection) -> Vec<Persona> {
        all_persona
            .find(codigo)
            .load::<Persona>(conn)
            .expect("Error cargando Persona")
    }

    pub fn all(conn: &PgConnection) -> Vec<Persona> {
        all_persona
            .order(persona::codigo.desc())
            .load::<Persona>(conn)
            .expect("Error cargando Personas")
    }

    pub fn insert(persona: NewPersona, conn: &PgConnection) -> bool {
        diesel::insert_into(persona::table)
            .values(&persona)
            .execute(conn)
            .is_ok()
    }
}