use rust_web_app_client::{apis::configuration::Configuration, apis::user_api, models::IUserDto};

#[actix_web::main]
async fn main() -> Result<(), rust_web_app_client::apis::Error<user_api::UserPutError>> {
  let configuration = {
    let mut config = Configuration::new();
    config.base_path = "http://localhost:8080".into();
    config
  };

  let user_dto = {
    let mut dto = IUserDto::new();
    dto.username.insert("test username".into());
    dto.email.insert("test email".into());
    dto
  };

  let response = user_api::user_put(&configuration, user_dto).await;
  println!("response: {:?}", response);
  Ok(())
}
