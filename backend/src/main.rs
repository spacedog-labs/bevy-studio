#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;

#[get("/echo")]
async fn api_echo() -> String {
    "echo".to_string()
}

#[get("/login?<code>")]
async fn login_user(code: Option<String>) -> String {
    "this is an access token yo".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![api_echo, login_user])
        .mount("/", FileServer::from("../frontend/build"))
}
