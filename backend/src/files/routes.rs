use rocket::data::{ByteUnit, Data};
use rocket::{tokio::io::AsyncReadExt, Route, State};
use rusoto_s3::{GetObjectRequest, S3Client, S3};

use crate::auth::JWTAuthorized;

use super::add_file;

pub fn routes() -> Vec<Route> {
    routes![upload, get]
}

#[post("/upload/<file_name>?<project_id>", data = "<data>")]
pub async fn upload(
    jwt_authorized: JWTAuthorized,
    s3_client: &State<S3Client>,
    data: Data<'_>,
    file_name: String,
    project_id: String,
) -> String {
    let stream = data
        .open(ByteUnit::Megabyte(2))
        .into_bytes()
        .await
        .unwrap()
        .value;
    let _put_result = add_file(s3_client, stream, project_id, file_name).await;
    jwt_authorized.0
}

#[get("/?<file>&<project_id>")]
pub async fn get(
    _jwt_authorized: JWTAuthorized,
    s3_client: &State<S3Client>,
    file: &str,
    project_id: &str,
) -> String {
    let get_output = s3_client
        .get_object(GetObjectRequest {
            bucket: "bevy-studio-projects".to_string(),
            key: format!("{project_id}/{file}").to_string(),
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
