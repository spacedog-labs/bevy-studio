#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rbatis;

use auth::{mint_jwt, JWTAuthorized, JWTSecret};
use github::GithubRequests;
use rbatis::rbatis::Rbatis;
use reqwest::Client as HTTPClient;
use rocket::http::Status;
use rocket::tokio::io::AsyncReadExt;
use rocket::{fs::FileServer, tokio, State};
use rusoto_core::{ByteStream, HttpClient};
use rusoto_credential::{AwsCredentials, ProvideAwsCredentials};
use rusoto_s3::{GetObjectRequest, PutObjectRequest, S3Client, S3};
use rusoto_signature::Region;
use users::db::*;

mod auth;
mod build;
mod github;

mod users;

#[post("/file/upload", data = "<text>")]
async fn upload(
    jwt_authorized: JWTAuthorized,
    s3_client: &State<S3Client>,
    text: String,
) -> String {
    let _put_result = s3_client
        .put_object(PutObjectRequest {
            body: Some(ByteStream::from(text.into_bytes())),
            bucket: "bevy-studio-projects".to_string(),
            key: "test/yolo".to_string(),
            ..Default::default()
        })
        .await
        .unwrap();
    jwt_authorized.0
}

#[get("/file?<file>")]
async fn get_file(
    jwt_authorized: JWTAuthorized,
    s3_client: &State<S3Client>,
    file: &str,
) -> String {
    let get_output = s3_client
        .get_object(GetObjectRequest {
            bucket: "bevy-studio-projects".to_string(),
            key: file.to_string(),
            ..Default::default()
        })
        .await
        .unwrap();

    let mut output: String = String::new();
    get_output
        .body
        .unwrap()
        .into_async_read()
        .read_to_string(&mut output)
        .await
        .unwrap();
    output
}

#[get("/login?<code>")]
async fn login_user(
    code: Option<String>,
    client: &State<HTTPClient>,
    sql_client: &State<Rbatis>,
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
                        .get_user(user.id.to_string(), sql_client)
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
                            .insert_user(&new_user, sql_client)
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
struct SQLSecret(String);
struct StorageSecret(String);

#[launch]
async fn rocket() -> _ {
    let client_secret = ClientSecret(get_environment_variable("ROCKET_GITHUB"));
    let sql_secret = SQLSecret(get_environment_variable("ROCKET_SQL"));
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
    rb.link(&sql_secret.0).await.unwrap();

    tokio::spawn(async move {
        let background_rb = Rbatis::new();
        background_rb.link(&sql_secret.0).await.unwrap();

        build::run_worker(&background_rb).await
    });

    rocket::build()
        .manage(reqwest::Client::new())
        .manage(rb)
        .manage(client_secret)
        .manage(jwt_secret)
        .manage(client)
        .mount("/api/projects", routes![upload, get_file])
        .mount("/api/user", users::routes())
        .mount("/api", routes![login_user])
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
