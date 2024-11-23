use rouille::{input::post, Request, Response};
use std::env;
use r2d2_postgres::{PostgresConnectionManager, postgres};
use log::{info, error};

mod user_manager;
mod user_controller;
mod library;

fn main() {
    simple_logger::SimpleLogger::new().init().unwrap();
    let server = env::var("LISTEN_HOST").unwrap_or("localhost".to_string());
    let port = env::var("LISTEN_PORT").unwrap_or("5000".to_string());
    

    let db_host: String = env::var("DATABASE_HOST").unwrap_or("localhost".to_string());
    let db_port: String = env::var("DATABASE_PORT").unwrap_or("5432".to_string());
    let db_user: String = env::var("DATABASE_USER").unwrap_or("postgres".to_string());
    let db_pass: String = env::var("DATABASE_PASS").unwrap_or("cHt0UFBbszX0YK7".to_string());
    let db_pool: u32 = env::var("DATABASE_POOL").unwrap_or("1".to_string()).parse().unwrap();
    let connection_string = format!("postgres://{}:{}@{}:{}", db_user, db_pass, db_host, db_port);
    info!("Connection String: {}", connection_string);
    let connection_manager = PostgresConnectionManager::new(
        connection_string.parse().unwrap(),
        postgres::NoTls
    );

    let pool = r2d2::Pool::builder().max_size(db_pool).build(connection_manager).unwrap();
    match user_manager::init_table(pool.clone()) {
        Some(err) => {
            error!("Error creating table: {}", err);
            return;
        },
        None => {},
    }
    info!("Database connection established");

    let addr = format!("{}:{}", server, port);
    info!("Server listening on {}", addr);

    rouille::start_server(addr, move |request| {
        if request.url() == "/user/create" {
            return user_controller::on_user_create(request, pool.clone());
        } else if request.url() == "/user/get" {
            return user_controller::on_user_get(request, pool.clone());
            
        } else if request.url() == "/user/auth"{
            return user_controller::on_user_auth(request, pool.clone());
        } else {
            Response::empty_404()
        }

    });
    
    
    

}