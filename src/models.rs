use serde::{Deserialize, Serialize};

use super::schema::{users, notes};

#[derive(Identifiable, Debug, Serialize, Deserialize, Queryable)]
#[table_name = "users"]
pub struct Users {
  pub id: i32,
  pub first_name: String,
  pub last_name: String,
  pub email: String,
  pub alias: String,
  pub created_at: chrono::NaiveDateTime
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
  pub first_name: &'a str,
  pub last_name: &'a str,
  pub email: &'a str,
  pub alias: &'a str,
}
#[derive(Identifiable, Debug, Serialize, Deserialize, Queryable, Associations)]
#[belongs_to(Users, foreign_key="user_id")]
#[table_name = "notes"]
pub struct Notes {
  pub id: i32,
  pub description: String,
  pub user_id: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "notes"]
pub struct NewNote<'a>{
  pub description: &'a str,
  pub user_id: i32,
}