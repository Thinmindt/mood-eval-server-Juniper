extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

#[cfg(test)]
mod tests {
    #[test]
    fn establish_connection() -> PgConnection {
        dtoenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        assert!(PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url)))
    }
}