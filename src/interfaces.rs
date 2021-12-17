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
  pub data: Option<T>,
  pub message: Option<String>
}

// TODO: Someday implement this
// impl<T> Resp<T> {
//   pub fn new(resp: Resp<T>) -> Resp<T> {
//     let Resp { status, data, message} = resp;
//     Resp { status, data, message}
//   }

//   pub fn get(self) -> Resp<T> {
//     let Resp { status, data, message} = self;
//     Resp::<T> { status, data, message}
//   }
// }