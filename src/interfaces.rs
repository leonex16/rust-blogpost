use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
  pub first_name: String,
  pub last_name: String,
  pub email: String,
  pub alias: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
  SUCCESS,
  ERROR
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resp<T> {
  pub status: Status,
  pub data: T,
  pub message: String
}