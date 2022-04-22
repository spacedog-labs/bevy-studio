#[macro_use]
extern crate rocket;

use std::collections::HashMap;

use reqwest::header::ACCEPT;
use reqwest::Client;
use rocket::{fairing::AdHoc, fs::FileServer, serde::Deserialize, State};

#[get("/echo")]
async fn api_echo() -> String {
    "echo".to_string()
}

#[get("/login?<code>")]
async fn login_user(
    code: Option<String>,
    client: &State<Client>,
    config: &State<Config>,
) -> String {
    let mut map = HashMap::<&str, String>::new();

    map.insert("client_id", "705625596ca39ae3136d".to_string());
    map.insert("client_secret", config.clientsecret.to_string());
    map.insert("code", code.unwrap());

    let result = client
        .post("https://github.com/login/oauth/access_token")
        .header(ACCEPT, "application/json")
        .json(&map)
        .send()
        .await;

    match result {
        Ok(response) => {
            let auth_response: AuthResponse = response.json().await.unwrap();
            auth_response.access_token
        }
        Err(_) => {
            panic!("auth failed")
        }
    }
}

#[derive(Deserialize)]
struct Config {
    clientsecret: String,
}

#[derive(Deserialize)]
struct AuthResponse {
    access_token: String,
    scope: String,
    token_type: String,
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(reqwest::Client::new())
        .attach(AdHoc::config::<Config>())
        .mount("/api", routes![api_echo, login_user])
        .mount("/", FileServer::from("../frontend/build"))
}
