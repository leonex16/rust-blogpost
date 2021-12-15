#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};
use dotenv::dotenv;

use crate::handlers as routes;
use crate::utils::socket;

mod services;
mod handlers;
mod models;
mod schema;
mod utils;
mod interfaces;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .service(routes::add_user)
            .service(routes::delete_user)
            .service(routes::get_users)
            .service(routes::get_user_by_id)
    })
    .bind(socket::get_instance())?
    .run()
    .await
}
