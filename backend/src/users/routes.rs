use crate::{auth::JWTAuthorized, users::db::*};
use rbatis::rbatis::Rbatis;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{Route, State};

pub fn routes() -> Vec<Route> {
    routes![get_me, update_me, get]
}

#[get("/me")]
async fn get_me(jwt_authorized: JWTAuthorized, sql_client: &State<Rbatis>) -> Json<User> {
    let user_manager = UserManager {};
    Json(
        user_manager
            .get_user(jwt_authorized.0, sql_client)
            .await
            .unwrap()
            .unwrap(),
    )
}

#[get("/<id>")]
async fn get(sql_client: &State<Rbatis>, id: &str) -> Json<PublicUser> {
    let user_manager = UserManager {};
    Json(
        user_manager
            .get_user(id.to_string(), sql_client)
            .await
            .unwrap()
            .unwrap()
            .to_public_user(),
    )
}

#[post("/me", data = "<user>")]
async fn update_me(
    jwt_authorized: JWTAuthorized,
    sql_client: &State<Rbatis>,
    user: Json<User>,
) -> status::NoContent {
    let user_manager = UserManager {};
    user_manager
        .update_user(jwt_authorized.0.to_string(), &user, sql_client)
        .await
        .unwrap();
    status::NoContent
}
