use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{users, notes};
use crate::models::{Users, Notes, NewUser};
use crate::utils::db;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
  pub first_name: String,
  pub last_name: String,
  pub email: String,
  pub alias: String,
}

pub fn add_user(user: String) -> (){
  let conn = db::get_instance();
  let inputUser: Result<InputUser, serde_json::Error> = serde_json::from_str(&user);

  match inputUser {
    Ok(user) => {
      let InputUser { first_name, last_name, email, alias } = user;
      let new_user = NewUser {
        first_name: &first_name,
        last_name: &last_name,
        email: &email,
        alias: &alias,
      };
      // diesel::insert_into(users::table)
      //   .values(&new_user)
      //   .execute(&conn)
      //   .expect("Error saving new user");
    },
    Err(e) => println!("ERROR ON ADD_USER: {}", e),
  }

}

pub fn get_users() -> Vec<(Users, Option<Notes>)> {
  let conn = db::get_instance();
  let results = users::table
  .left_join(notes::table)
  .load::<(Users, Option<Notes>)>(&conn);

  match results {
    Ok(results) => results,
    Err(err) => panic!("ERROR ON GET_USERS: {}", err),
  }
}