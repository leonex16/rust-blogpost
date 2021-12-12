#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};
use dotenv::dotenv;

mod handlers;
mod models;
mod utils;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Start http server
    HttpServer::new(|| App::new().service(handlers::get_users))
        .bind(utils::get_socket())?
        .run()
        .await
}
