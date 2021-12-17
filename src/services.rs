use diesel::prelude::*;
use diesel::result::Error::NotFound;

use crate::interfaces::*;
use crate::models::{NewUser, Notes, Users};
use crate::schema::{notes, users};
use crate::utils::{db, logs};

pub fn add_user(user: String) -> Resp<bool> {
    let conn = db::get_instance();
    let input_user: Result<InputUser, serde_json::Error> = serde_json::from_str(&user);

    match input_user {
        Ok(user) => {
            let InputUser {
                first_name,
                last_name,
                email,
                alias,
            } = user;
            let new_user = NewUser {
                first_name: &first_name.trim().to_uppercase(),
                last_name: &last_name.trim().to_uppercase(),
                email: &email.trim().to_uppercase(),
                alias: &alias.trim().to_uppercase(),
            };
            // TODO: IMPLEMENT LOGIC FOR AVOID DUPLICATE USER
            diesel::insert_into(users::table)
                .values(&new_user)
                .execute(&conn)
                .expect("ERROR ON ADD_USER");
            let resp = Resp::<bool> {
                status: Status::SUCCESS,
                data: Some(true),
                message: Some(String::from("Usuario Creado")),
            };

            resp
        }
        Err(e) => {
            let resp = Resp::<bool> {
                status: Status::ERROR,
                data: None,
                message: Some(String::from("Ups, Tenemos Algunos Problemas.")),
            };
            logs::error(&("ERROR ON ADD_USER:", e));

            resp
        }
    }
}

pub fn get_users() -> Resp<Vec<(Users, Option<Notes>)>> {
    let conn = db::get_instance();
    let users = users::table
        .left_join(notes::table)
        .load::<(Users, Option<Notes>)>(&conn);

    match users {
        Ok(users) => {
            let resp = Resp::<Vec<(Users, Option<Notes>)>> {
                status: Status::SUCCESS,
                data: Some(users),
                message: None,
            };

            resp
        }
        Err(e) => {
            let resp = Resp::<Vec<(Users, Option<Notes>)>> {
                status: Status::ERROR,
                data: None,
                message: Some(String::from("Ups, Tenemos Algunos Problemas.")),
            };

            logs::error(&("ERROR ON GET_USERS:", e));

            resp
        }
    }
}

pub fn get_user_by_id(id: i32) -> Resp<(Users, Option<Notes>)> {
    let conn = db::get_instance();
    let user = users::table
        .find(id)
        .left_join(notes::table)
        .first::<(Users, Option<Notes>)>(&conn);

    match user {
        Ok(user) => {
            let resp = Resp::<(Users, Option<Notes>)> {
                status: Status::SUCCESS,
                data: Some(user),
                message: None,
            };

            resp
        }
        Err(e) => {
            let mut resp = Resp::<(Users, Option<Notes>)> {
                status: Status::ERROR,
                data: None,
                message: None,
            };
            match e {
                NotFound => {
                    resp.message = Some(String::from("Ups, El Usuario No Exite."));
                    logs::error(&("ERROR ON GET_USER:", e));

                    resp
                }
                _ => {
                    resp.message = Some(String::from("Ups, Tenemos Algunos Problemas."));
                    logs::error(&("ERROR ON GET_USER:", e));

                    resp
                }
            }
        }
    }
}

pub fn delete_user_by_id(id: i32) -> Resp<bool> {
    let conn = db::get_instance();
    diesel::delete(users::table.find(id)).execute(&conn);

    let resp = Resp::<bool> {
        status: Status::SUCCESS,
        data: None,
        message: None,
    };

    resp
}
