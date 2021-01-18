use crate::error_handler::CustomError;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use r2d2;
use std::env;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

embed_migrations!();

lazy_static! {
    static ref POOL: Pool = {
        let db_url = "postgres://postgres:1234@localhost/migrar";
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("No se pudo crear el grupo de bases de datos")
    };
}

pub fn init() {
    lazy_static::initialize(&POOL);
    let conn = connection().expect("No se pudo obtener la conexión de base de datos");
    embedded_migrations::run(&conn).unwrap();
}

pub fn connection() -> Result<DbConnection, CustomError> {
    POOL.get()
        .map_err(|e| CustomError::new(500, format!("No se pudo obtener la conexión de base de datos: {}", e)))
}