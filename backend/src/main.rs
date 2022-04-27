#[macro_use]
extern crate rocket;

use auth::{mint_jwt, JWTAuthorized, JWTSecret};
use github::GithubRequests;
use mongodb::{options::ClientOptions, Client};
use reqwest::Client as HTTPClient;
use rocket::http::Status;
use rocket::{fs::FileServer, tokio, State};
use users::{User, UserManager};

mod auth;
mod build;
mod github;
mod users;

#[get("/echo")]
async fn api_echo(_jwt_authorized: JWTAuthorized) -> String {
    "echo".to_string()
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

struct ClientSecret(String);
struct CosmosSecret(String);
impl AsRef<str> for CosmosSecret {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[launch]
async fn rocket() -> _ {
    let client_secret = ClientSecret(get_environment_variable("ROCKET_GITHUB"));
    let cosmos_secret = CosmosSecret(get_environment_variable("ROCKET_COSMOS"));
    let jwt_secret = JWTSecret(get_environment_variable("ROCKET_JWT"));

    let client_options = ClientOptions::parse(&cosmos_secret).await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    tokio::spawn(async move {
        let client_options = ClientOptions::parse(&cosmos_secret).await.unwrap();
        let builder_db_client = Client::with_options(client_options).unwrap();
        build::run_worker(builder_db_client).await
    });

    rocket::build()
        .manage(reqwest::Client::new())
        .manage(client)
        .manage(client_secret)
        .manage(jwt_secret)
        .mount("/api", routes![api_echo, login_user])
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
