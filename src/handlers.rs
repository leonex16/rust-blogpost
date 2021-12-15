  use actix_web::{get, post, delete, Responder, Result, web};
  
  use crate::services;
  use crate::models::{Users, Notes};
  use crate::interfaces::{Resp};

  #[post("/users")]
  pub async fn add_user(body: String) -> Result<impl Responder> {
    let resp: Resp::<bool> = services::add_user(body);

    Ok(web::Json(resp))
  }

  #[get("/users")]
  pub async fn get_users() -> Result<impl Responder> {
    let resp: Resp::<Vec<(Users, Option<Notes>)>> = services::get_users();
    
    Ok(web::Json(resp))
  }
  
  #[get("/users/{id}")]
  pub async fn get_user_by_id() -> impl Responder {
    format!("hello from get users by id")
  }
  
  #[delete("/users/{id}")]
  pub async fn delete_user() -> impl Responder {
    format!("hello from delete user")
  }
