use hyper::header::{AUTHORIZATION, USER_AGENT};
use hyper::StatusCode;
use reqwest::header::ACCEPT;
use reqwest::Client;
use rocket::serde::json::serde_json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const CLIENT_ID: &str = "705625596ca39ae3136d";

pub struct GithubRequests {
    pub client_secret: String,
}

impl GithubRequests {
    pub async fn login(&self, client: &Client, code: String) -> Result<String, String> {
        let mut map = HashMap::<&str, String>::new();

        map.insert("client_id", CLIENT_ID.to_string());
        map.insert("client_secret", self.client_secret.to_string());
        map.insert("code", code);
        map.insert("scope", "read:user".to_string());

        let result = client
            .post("https://github.com/login/oauth/access_token")
            .header(ACCEPT, "application/json")
            .json(&map)
            .send()
            .await;

        match result {
            Ok(response) => {
                let status = response.status();
                if status == StatusCode::OK {
                    let auth_response: AuthResponse = response.json().await.unwrap();
                    Ok(auth_response.access_token)
                } else {
                    Err(format!("github oauth request failed with {}", status))
                }
            }
            Err(_) => Err("github oauth failed in io".to_string()),
        }
    }

    pub async fn get_github_user(
        &self,
        client: &Client,
        access_token: String,
    ) -> Result<GithubUser, String> {
        let result = client
            .get("https://api.github.com/user")
            .header(ACCEPT, "application/vnd.github.v3+json")
            .header(AUTHORIZATION, format!("token {}", access_token))
            .header(USER_AGENT, "bevy_studio")
            .send()
            .await;

        match result {
            Ok(response) => {
                let status = response.status();
                let text = &response.text().await.unwrap();
                if status == StatusCode::OK {
                    let github_user: GithubUser =
                        serde_json::from_str::<GithubUser>(&text).unwrap();
                    Ok(github_user)
                } else {
                    Err(format!("github oauth request failed with {}", status))
                }
            }
            Err(_) => Err("github oauth failed in io".to_string()),
        }
    }
}

#[derive(Deserialize, Debug)]
struct AuthResponse {
    access_token: String,
}

#[derive(Deserialize, Debug)]
pub struct GithubUser {
    pub id: i32,
    pub avatar_url: String,
}
