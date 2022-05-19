use std::convert::Infallible;

use rusoto_core::{ByteStream, RusotoError};
use rusoto_s3::{
    GetObjectRequest, PutObjectError, PutObjectOutput, PutObjectRequest, S3Client, S3,
};

pub async fn add_file(
    s3_client: &S3Client,
    bytes: Vec<u8>,
    project_id: String,
    file_name: String,
) -> Result<PutObjectOutput, RusotoError<PutObjectError>> {
    Ok(s3_client
        .put_object(PutObjectRequest {
            body: Some(ByteStream::from(bytes)),
            bucket: "bevy-studio-projects".to_string(),
            key: format!("{project_id}/{file_name}",).to_string(),
            ..Default::default()
        })
        .await?)
}
