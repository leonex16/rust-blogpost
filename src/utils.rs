use diesel::pg::PgConnection;
use diesel::connection::Connection;

use std::env;
use std::net::{IpAddr, SocketAddr};

pub fn get_socket() -> SocketAddr {
    let host = env::var("API_HOST").expect("API_HOST must be set");
    let port = env::var("API_PORT").expect("API_PORT must be set");
    let host = host.parse::<IpAddr>().unwrap();
    let port = port.parse::<u16>().unwrap();

    SocketAddr::new(host, port)
}

pub fn init_db() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
    .expect(&format!("Error connecting to {}", database_url))
}
