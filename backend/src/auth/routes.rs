use crate::HTTPClient;
use rbatis::rbatis::Rbatis;
use rocket::{http::Status, State};

use crate::{
    users::{User, UserData},
    GithubSecret,
};

use super::{mint_jwt, GithubRequests, JWTSecret};

#[get("/?<code>")]
pub async fn login_user(
    code: Option<String>,
    client: &State<HTTPClient>,
    sql_client: &State<Rbatis>,
    client_secret: &State<GithubSecret>,
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
                    let user_manager = UserData {};
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
