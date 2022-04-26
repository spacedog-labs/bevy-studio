#[macro_use]
extern crate rocket;

use bollard::container::{CreateContainerOptions, LogsOptions, StatsOptions};
use bollard::Docker;
use chrono::{DateTime, Utc};
use github::GithubRequests;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use mongodb::{options::ClientOptions, Client};
use reqwest::Client as HTTPClient;
use rocket::futures::TryStreamExt;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome};
use rocket::Request;
use rocket::{fs::FileServer, State};
use serde::{Deserialize, Serialize};
use std::{thread, time};
use users::{User, UserManager};

mod github;
mod users;

#[get("/echo")]
async fn api_echo(_jwtAuthorized: JWTAuthorized) -> String {
    "echo".to_string()
}

#[get("/runcontainer")]
async fn run_container(_jwtAuthorized: JWTAuthorized) -> String {
    let docker = Docker::connect_with_local_defaults().unwrap();
    let container_id = docker
        .create_container(
            Some(CreateContainerOptions {
                name: "test_container",
            }),
            bollard::container::Config {
                image: Some("test"),
                ..Default::default()
            },
        )
        .await
        .unwrap()
        .id;
    docker
        .start_container::<String>(&container_id, None)
        .await
        .unwrap();

    let ten_millis = time::Duration::from_secs(5);
    thread::sleep(ten_millis);

    let vec = &docker
        .logs(
            &container_id,
            Some(LogsOptions::<String> {
                stdout: true,
                ..Default::default()
            }),
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    let value = vec.get(1).unwrap();
    println!("{}", value.to_string());

    "working".to_string()
}

#[get("/login?<code>")]
async fn login_user(
    code: Option<String>,
    client: &State<HTTPClient>,
    cosmos_client: &State<Client>,
    client_secret: &State<ClientSecret>,
    jwt_secret: &State<JWTSecret>,
) -> (Status, String) {
    let github = GithubRequests {
        client_secret: client_secret.0.to_string(),
    };

    let result = github.login(client, code.unwrap()).await;

    match result {
        Ok(access_token) => {
            let github_user = github.get_github_user(client, access_token).await;

            match github_user {
                Ok(user) => {
                    let user_manager = UserManager {};
                    let user_opt = user_manager
                        .get_user(user.id.to_string(), cosmos_client)
                        .await
                        .unwrap();

                    if let Some(user_record) = user_opt {
                        // make jwt and return it
                        let jwt = mint_jwt(&jwt_secret.0, &user_record.id);

                        return (Status::Ok, jwt.to_string());
                    } else {
                        // make user in db and return it
                        let new_user = User {
                            id: user.id.to_string(),
                            avatar_url: user.avatar_url,
                        };

                        user_manager
                            .insert_user(&new_user, &cosmos_client)
                            .await
                            .unwrap();

                        let jwt = mint_jwt(&jwt_secret.0, &new_user.id);

                        return (Status::Ok, jwt.to_string());
                    }
                }
                Err(err_msg) => (Status::Unauthorized, err_msg),
            }
        }
        Err(err_msg) => (Status::Forbidden, err_msg),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String, // Optional. Audience
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)
}

struct ClientSecret(String);
struct CosmosSecret(String);
impl AsRef<str> for CosmosSecret {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
struct JWTSecret(String);

#[launch]
async fn rocket() -> _ {
    let client_secret = ClientSecret(get_environment_variable("ROCKET_GITHUB"));
    let cosmos_secret = CosmosSecret(get_environment_variable("ROCKET_COSMOS"));
    let jwt_secret = JWTSecret(get_environment_variable("ROCKET_JWT"));

    let client_options = ClientOptions::parse(cosmos_secret).await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    rocket::build()
        .manage(reqwest::Client::new())
        .manage(client)
        .manage(client_secret)
        .manage(jwt_secret)
        .mount("/api", routes![api_echo, login_user, run_container])
        .mount("/", FileServer::from("../frontend/build"))
}

fn get_environment_variable(name: &str) -> String {
    let os_str = std::env::var_os(name);

    if let Some(result) = os_str {
        result.to_str().unwrap().to_string()
    } else {
        panic!("missing required environment variable {}", name)
    }
}

fn mint_jwt(jwt_secret: &str, uid: &str) -> String {
    let claims = Claims {
        aud: "bevy-studio".to_string(),
        // 28 day expiry
        exp: Utc::now().timestamp() as usize + 2419200 as usize,
        sub: uid.to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .unwrap()
}

/// Returns true if `key` is a valid API key string.
fn validate_jwt(jwt_secret: &str, key: &str) -> Option<String> {
    match decode::<Claims>(
        &key,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(t) => Some(t.claims.sub),
        Err(_) => None,
    }
}

struct JWTAuthorized(String);

#[derive(Debug)]
enum JWTError {
    Invalid,
}

#[async_trait]
impl<'r> FromRequest<'r> for JWTAuthorized {
    type Error = JWTError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let jwt = request.headers().get_one("Authorization").unwrap();
        let jwt_secret = request.rocket().state::<JWTSecret>().unwrap();
        match validate_jwt(&jwt_secret.0, jwt) {
            Some(user_id) => {
                return Outcome::Success(JWTAuthorized(user_id));
            }
            None => {
                return Outcome::Failure((Status::Unauthorized, JWTError::Invalid));
            }
        }
    }
}
