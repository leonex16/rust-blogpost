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
  pub created_at: chrono::NaiveDateTime,
  pub notes: Option<Notes>,
}

#[derive(Identifiable, Debug, Serialize, Deserialize, Queryable, Associations)]
#[belongs_to(Users, foreign_key="user_id")]
#[table_name = "notes"]
pub struct Notes {
  pub id: i32,
  pub description: String,
  pub user_id: i32,
}