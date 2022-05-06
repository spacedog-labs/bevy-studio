use rocket::{tokio::io::AsyncReadExt, Route, State};
use rusoto_core::ByteStream;
use rusoto_s3::{GetObjectRequest, PutObjectRequest, S3Client, S3};

use crate::auth::JWTAuthorized;

pub fn routes() -> Vec<Route> {
    routes![upload, get]
}

#[post("/file/upload", data = "<text>")]
pub async fn upload(
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
pub async fn get(
    _jwt_authorized: JWTAuthorized,
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
