  use actix_web::{get, post, delete, Responder, Result, web};
  
  use crate::services;

  #[get("/users/get")]
  pub async fn get_users() -> Result<impl Responder> {
    let users = services::get_users();
  
    Ok(web::Json(users))
  }
  
  #[get("/user/get/{id}")]
  pub async fn get_user_by_id() -> impl Responder {
    format!("hello from get users by id")
  }
  
  #[post("/user/add")]
  pub async fn add_user(body: String) -> impl Responder {
    services::add_user(body);
    format!("oeu")
  }
  
  #[delete("/user/delete/{id}")]
  pub async fn delete_user() -> impl Responder {
    format!("hello from delete user")
  }
