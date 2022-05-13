#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rbatis;

use crate::auth::login_user;
use auth::JWTSecret;
use rbatis::rbatis::Rbatis;
use reqwest::Client as HTTPClient;
use rocket::{fs::FileServer, tokio};
use rusoto_core::HttpClient;
use rusoto_credential::{AwsCredentials, ProvideAwsCredentials};
use rusoto_s3::S3Client;
use rusoto_signature::Region;

mod auth;
mod build;
mod files;
mod projects;
mod users;

pub struct GithubSecret(String);
pub struct SQLSecret(String);
pub struct StorageSecret(String);

#[launch]
async fn rocket() -> _ {
    let client_secret = GithubSecret(get_environment_variable("ROCKET_GITHUB"));
    let sql_secret = get_environment_variable("ROCKET_SQL");
    let jwt_secret = JWTSecret(get_environment_variable("ROCKET_JWT"));

    let client = S3Client::new_with(
        HttpClient::new().unwrap(),
        DOCredentials,
        Region::Custom {
            name: "sfo3".to_string(),
            endpoint: "sfo3.digitaloceanspaces.com".to_string(),
        },
    );

    let rb = Rbatis::new();
    rb.link(&sql_secret).await.unwrap();

    tokio::spawn(async move {
        let background_rb = Rbatis::new();
        background_rb.link(&sql_secret).await.unwrap();

        build::run_worker(&background_rb).await
    });

    rocket::build()
        .manage(reqwest::Client::new())
        .manage(rb)
        .manage(client_secret)
        .manage(jwt_secret)
        .manage(client)
        .mount("/api/file", files::routes())
        .mount("/api/user", users::routes())
        .mount("/api/project", projects::routes())
        .mount("/api/login", routes![login_user])
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

struct DOCredentials;

#[async_trait]
impl ProvideAwsCredentials for DOCredentials {
    async fn credentials(
        &self,
    ) -> Result<rusoto_credential::AwsCredentials, rusoto_credential::CredentialsError> {
        let storage_secret = StorageSecret(get_environment_variable("ROCKET_STORAGE"));
        Ok(AwsCredentials::new(
            "WMHTWNDNW2IRNI6FWPV5",
            storage_secret.0,
            None,
            None,
        ))
    }
}
