use std::io::Error;

use r2d2_postgres::{postgres, PostgresConnectionManager};

pub fn init_table(pool: r2d2::Pool<PostgresConnectionManager<postgres::NoTls>>) -> Option<Error> {
    let mut conn = pool.get().unwrap();
    let result = conn.execute(
        
        "CREATE TABLE IF NOT EXISTS platform_users (
                name VARCHAR(255) NOT NULL,
                password VARCHAR(255) NOT NULL,
                email VARCHAR(255) NOT NULL UNIQUE,
                organization VARCHAR(255),
                version INT NOT NULL,
                phone VARCHAR(15) NOT NULL,
                location VARCHAR(255) NOT NULL,
                level INT NOT NULL,
                PRIMARY KEY (email)
            )",
        &[]
    );
    match result {
        Ok(_) => {
            log::info!("Table created");
            return None;
        },
        Err(err) => {
            log::error!("Error creating table: {}", err);
            return Some(Error::new(std::io::ErrorKind::Other, "Error with table creation"));
        }
        
    }

}