use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;

embed_migrations!("./migrations/");

pub fn run_migrations() {
    let connection = establish_connection();

    embedded_migrations::run_with_output(&connection, &mut std::io::stdout()).unwrap()
}

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
