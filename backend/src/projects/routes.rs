use crate::{auth::JWTAuthorized, users::db::*};
use rbatis::rbatis::Rbatis;
use rocket::{Route, State};

pub fn routes() -> Vec<Route> {
    routes![get]
}

#[get("/")]
async fn get(jwt_authorized: JWTAuthorized, sql_client: &State<Rbatis>) -> String {
    let user_manager = UserManager {};
    let user_opt = user_manager
        .get_user(jwt_authorized.0, sql_client)
        .await
        .unwrap();

    if let Some(user) = user_opt {
        user.avatar_url
    } else {
        "".to_string()
    }
}
