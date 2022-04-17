#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;

#[get("/echo")]
async fn api_echo() -> String {
    "echo".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![api_echo])
        .mount("/", FileServer::from("../frontend/build"))
}
