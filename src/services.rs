use diesel::prelude::*;

use crate::interfaces::*;
use crate::schema::{users, notes};
use crate::models::{Users, Notes, NewUser};
use crate::utils::{db, logs};

pub fn add_user(user: String) -> Resp<bool> {
  let conn = db::get_instance();
  let input_user: Result<InputUser, serde_json::Error> = serde_json::from_str(&user);

  match input_user {
    Ok(user) => {
      let InputUser { first_name, last_name, email, alias } = user;
      let new_user = NewUser {
        first_name: &first_name.trim().to_uppercase(),
        last_name: &last_name.trim().to_uppercase(),
        email: &email.trim().to_uppercase(),
        alias: &alias.trim().to_uppercase(),
      };

      diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&conn)
        .expect("ERROR ON ADD_USER");

      Resp::<bool> {
        status: Status::SUCCESS,
        data: Some(true),
        message: String::from("Usuario Creado")
      }
    },
    Err(e) => {
      logs::error(&("ERROR ON ADD_USER:", e));

      Resp::<bool> {
        status: Status::ERROR,
        data: None,
        message: String::from("Ups, Tenemos Algunos Problemas.")
      }
    },
  }

}

pub fn get_users() -> Resp::<Vec<(Users, Option<Notes>)>> {
  let conn = db::get_instance();
  let users = users::table
  .left_join(notes::table)
  .load::<(Users, Option<Notes>)>(&conn);

  match users {
    Ok(users) => Resp::<Vec<(Users, Option<Notes>)>> {
      status: Status::SUCCESS,
      data: Some(users),
      message: String::from("")
    },
    Err(e) => {
      logs::error(&("ERROR ON GET_USERS:", e));

      Resp::<Vec<(Users, Option<Notes>)>> {
        status: Status::ERROR,
        data: None,
        message: String::from("Ups, Tenemos Algunos Problemas.")
      }
    },
  }
}

pub fn get_user(id: i32) -> Resp::<(Users, Option<Notes>)> {
  let conn = db::get_instance();
  let user = users::table.find(id).left_join(notes::table).first::<(Users, Option<Notes>)>(&conn);

  match user {
    Ok(user) => {
      Resp::<(Users, Option<Notes>)>{
        status: Status::SUCCESS,
        data: Some(user),
        message: String::from("")
      }
    },
    Err(e) => {
      logs::error(&("ERROR ON GET_USER:", e));
      Resp::<(Users, Option<Notes>)> {
        status: Status::ERROR,
        data: None,
        message: String::from("Ups, Tenemos Algunos Problemas.")
      }
    },
  }

}