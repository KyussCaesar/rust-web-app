use actix_web::{
  error, http::StatusCode, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
  Result,
};
use actix_web_prom::{PrometheusMetrics, PrometheusMetricsBuilder};
use chrono::{DateTime, Utc};
use deadpool_postgres::{self, Client, Pool};
use prometheus::process_collector::ProcessCollector;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::NoTls;
use uuid::Uuid;

use rust_web_app_client::models::IUserDto;

/// User entity
#[derive(PostgresMapper)]
#[pg_mapper(table = "user")]
struct User {
  id: Uuid,
  username: String,
  email: String,
  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>,
  deleted_at: Option<DateTime<Utc>>,
}

impl Into<IUserDto> for User {
  fn into(self) -> IUserDto {
    IUserDto {
      id: Some(self.id.to_string()),
      username: Some(self.username),
      email: Some(self.email),
      created_at: Some(self.created_at.to_rfc3339()),
      updated_at: Some(self.updated_at.to_rfc3339()),
      deleted_at: self.deleted_at.map(|d| d.to_rfc3339()),
    }
  }
}

async fn healthcheck_get() -> impl Responder {
  HttpResponse::Ok()
}

async fn user_put(user_dto: web::Json<IUserDto>, pool: web::Data<Pool>) -> Result<impl Responder> {
  let response_user: IUserDto;
  let response_status: StatusCode;

  let db: Client = pool.get().await.map_err(|e| {
    println!("uh oh: unable to get connection from pool: {}", e);
    error::ErrorInternalServerError("unable to handle request")
  })?;

  if let Some(user_id) = &user_dto.id {
    println!("request is to update userId={}", user_id);
    let mut existing_user = {
      let user_id = Uuid::parse_str(&user_id).map_err(|e| {
        println!("uh oh: unable to update user information: unable to parse user UUID: {}", e);
        error::ErrorBadRequest("unable to parse the provided user UUID")
      })?;

      let rows = db
        .query("SELECT * FROM \"user\" WHERE id = $1 AND deleted_at IS NULL", &[&user_id])
        .await
        .map_err(|e| {
          println!("uh oh: unable to fetch existing user({}): {}", user_id, e);
          error::ErrorInternalServerError("unable to handle request")
        })?;

      match rows[..] {
        [ref row] => User::from_row_ref(row).map_err(|e| {
          println!("uh oh: unable to convert row into struct: {}", e);
          error::ErrorInternalServerError("unable to handle request")
        })?,
        [] => return Err(error::ErrorNotFound("no such user exists for the provided user ID")),
        _ => {
          // should never happen because PK constraint
          println!("uh oh: unable to update user({}): more than 1 row returned from DB!", user_id);
          return Err(error::ErrorInternalServerError("unable to handle request"));
        }
      }
    };

    if let Some(username) = &user_dto.username {
      println!("request is to update userId={} username={}", user_id, username);
      existing_user.username = username.clone();
    }

    if let Some(email) = &user_dto.email {
      println!("request is to update userId={} email={}", user_id, email);
      existing_user.email = email.clone();
    }

    // TODO the fetch and the update should be happening in the same transaction
    // otherwise we might try update a deleted user
    let updated_user = db.query("UPDATE \"user\" SET username = $2, email = $3, updated_at = now() WHERE id = $1 RETURNING *", &[&existing_user.id, &existing_user.username, &existing_user.email]).await
      .map_err(|e| {
        println!("uh oh: error when trying to update in db: {}", e);
        error::ErrorInternalServerError("unable to handle request")
      })
      .and_then(|rows| {
        match rows[..] {
          [ref row] => User::from_row_ref(row)
            .map_err(|e| {
              println!("uh oh: unable to convert row into struct: {}", e);
              error::ErrorInternalServerError("unable to handle request")
            }),
          _ => {
            println!("uh oh: no user returned after update");
            Err(error::ErrorInternalServerError("unable to handle request"))
          },
        }
      })?;

    response_user = updated_user.into();
    response_status = StatusCode::OK;
  } else {
    println!("request is to create user");

    let created_user = match (&user_dto.username, &user_dto.email) {
      (Some(username), Some(email)) =>
        db.query("INSERT INTO \"user\"(username, email, created_at, updated_at) VALUES ($1, $2, now(), now()) RETURNING *", &[username, email]).await
        .map_err(|e| {
          println!("uh oh: error when trying to create in db: {}", e);
          error::ErrorInternalServerError("unable to handle request")
        })
        .and_then(|rows| {
          match rows[..] {
            [ref row] => User::from_row_ref(row)
              .map_err(|e| {
                println!("uh oh: unable to convert row into struct: {}", e);
                error::ErrorInternalServerError("unable to handle request")
              }),
            _ => {
              println!("uh oh: no user returned after insert");
              Err(error::ErrorInternalServerError("unable to handle request"))
            }
          }
        }),
      (_, None) => {
        println!("missing email");
        Err(error::ErrorBadRequest("email is required when creating a new user"))
      },
      (None, _) => {
        println!("missing username");
        Err(error::ErrorBadRequest("username is required when creating a new user"))
      }
    }?;

    response_user = created_user.into();
    response_status = StatusCode::CREATED;
  }

  let response = web::Json(response_user)
    .customize()
    .with_status(response_status);

  Ok(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let prometheus = PrometheusMetricsBuilder::new("api")
    .endpoint("/metrics")
    .build()
    .unwrap();

  let process_collector = ProcessCollector::for_self();
  prometheus
    .registry
    .register(Box::new(process_collector))
    .expect("unable to register the prometheus process collector");

  let pool = {
    let mut config = deadpool_postgres::Config::default();
    config.dbname = Some("rust-web-app".into());
    config.user = Some("rust-web-app".into());
    config.application_name = Some("rust-web-app".into());
    config.host = Some("postgres".into());
    config.port = Some(4321);

    config
      .create_pool(None, NoTls)
      .expect("unable to create DB pool")
  };

  HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(pool.clone()))
      .wrap(middleware::Compress::default())
      .route("/healthcheck", web::get().to(healthcheck_get))
      .wrap(prometheus.clone())
      .route("/user", web::put().to(user_put))
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}

#[cfg(test)]
mod tests {
  use super::*;
  use actix_web::test;

  #[actix_web::test]
  async fn test_healthcheck() {
    let req = test::TestRequest::default().to_http_request();
    let resp = healthcheck_get().await.respond_to(&req);
    assert_eq!(resp.status(), StatusCode::OK);
  }
}
