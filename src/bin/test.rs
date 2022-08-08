use rust_web_app_client::{apis::configuration::Configuration, apis::{user_api, monitoring_api}, models::IUserDto};

#[actix_web::main]
async fn main() -> Result<(), rust_web_app_client::apis::Error<user_api::UserPutError>> {
  let configuration = {
    let mut config = Configuration::new();
    config.base_path = "http://localhost:8080".into();
    config
  };

  let healthcheck = monitoring_api::healthcheck_get(&configuration).await;
  println!("healthcheck: {:?}", healthcheck);

  let mut user_dto = IUserDto::new();
  user_dto.username = Some("test username".into());
  user_dto.email = Some("test email".into());

  let response = user_api::user_put(&configuration, user_dto.clone()).await;
  println!("response: {:?}", response);

  user_dto.id = response.unwrap().id;

  user_dto.email = Some("new email".into());
  let response = user_api::user_put(&configuration, user_dto.clone()).await;
  println!("response: {:?}", response);

  // does not work, bug in the generated client
  // the client *always* tries to deserialise as JSON even though the response type is text/*
  // let metrics = monitoring_api::metrics_get(&configuration).await;
  // println!("metrics: {:?}", metrics);

  Ok(())
}
