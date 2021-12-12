use actix_web::{get, post, delete, Responder, Result, web};
use diesel::prelude::*;

use super::models::{Users, Notes};
use super::schema::users::dsl::users;
use super::schema::notes::dsl::notes;
use super::utils::init_db;

#[get("/users/get")]
pub async fn get_users() -> impl Responder {
  let conn = init_db();

  let results = users.left_join(notes);
  println!("{:#?}", results.load::<???>(&conn));
  // ERROR TOP LINE, I DO NOT THE TYPE RETURNED OF RESULTS QUERY :(
  format!("hello from get users by id")

}

#[get("/user/get/{id}")]
pub async fn get_user_by_id() -> impl Responder {
  format!("hello from get users by id")
}

#[post("/users/add")]
pub async fn add_user() -> impl Responder {
  format!("hello from add user")
}

#[delete("/user/delete/{id}")]
pub async fn delete_user() -> impl Responder {
  format!("hello from delete user")
}
