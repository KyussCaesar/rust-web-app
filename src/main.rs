use actix_web::{HttpServer, web, middleware, App, Responder, Result, error, http::StatusCode};
use chrono::Utc;

use rust_web_app_client::models::IUserDto;

async fn user_put(user_dto: web::Json<IUserDto>) -> Result<impl Responder> {
  let mut response_user = IUserDto::new();
  let response_status: StatusCode;

  if let Some(user_id) = &user_dto.id {
    println!("request is to update userId={}", user_id);
    // TODO fetch existing user information from the DB
    // TODO error if the specifed ID is not found (response_status = StatusCode::NOT_FOUND)
    response_user.id = Some(user_id.clone());

    if let Some(username) = &user_dto.username {
      println!("request is to update userId={} username={}", user_id, username);
      response_user.username = Some(username.clone());
    }

    if let Some(email) = &user_dto.email {
      println!("request is to update userId={} email={}", user_id, email);
      response_user.email = Some(email.clone());
    }

    response_user.updated_at = Some(Utc::now().to_rfc3339());

    response_status = StatusCode::OK;
  } else {
    println!("request is to create user");

    match (&user_dto.username, &user_dto.email) {
      (Some(username), Some(email)) => {
        response_user.id = Some("antony temp ID".into());
        response_user.username = Some(username.clone());
        response_user.email = Some(email.clone());
        let created_at = Utc::now().to_rfc3339();
        response_user.created_at = Some(created_at.clone());
        response_user.updated_at = Some(created_at.clone());
      },
      (_, None) => {
        println!("missing email");
        return Err(error::ErrorBadRequest("email is required when creating a new user"));
      },
      (None, _) => {
        println!("missing username");
        return Err(error::ErrorBadRequest("username is required when creating a new user"));
      }
    }

    response_status = StatusCode::CREATED;
  }

  // TODO actually save the information in the DB before sending the response
  let response = web::Json(response_user)
      .customize()
      .with_status(response_status);

  Ok(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .route("/user", web::put().to(user_put))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
