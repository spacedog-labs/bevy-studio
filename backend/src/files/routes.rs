use rbatis::rbatis::Rbatis;
use rocket::data::{ByteUnit, Data};
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket::{tokio::io::AsyncReadExt, Route, State};
use rusoto_s3::{GetObjectRequest, S3Client, S3};
use uuid::Uuid;

use crate::auth::JWTAuthorized;
use crate::projects::ProjectData;

use super::{add_file, get_file, File, FileData};

pub fn routes() -> Vec<Route> {
    routes![add, get, get_many]
}

#[post("/add?<file_name>&<project_id>", data = "<data>")]
pub async fn add(
    jwt_authorized: JWTAuthorized,
    s3_client: &State<S3Client>,
    sql_client: &State<Rbatis>,
    data: Data<'_>,
    file_name: String,
    project_id: String,
) {
    let project = ProjectData::get_owned(&project_id, &jwt_authorized.0, sql_client)
        .await
        .unwrap()
        .unwrap();

    let existing_file = FileData::get(&project_id, &file_name, sql_client)
        .await
        .unwrap();

    let stream = data
        .open(ByteUnit::Megabyte(4))
        .into_bytes()
        .await
        .unwrap()
        .value;

    if let Some(_f) = existing_file {
        add_file(s3_client, stream, project_id, file_name)
            .await
            .unwrap();
        return;
    } else {
        let file = File {
            id: Uuid::new_v4().to_string(),
            project_id: project_id.clone(),
            name: file_name.clone(),
        };

        FileData::create(&file, sql_client).await.unwrap();

        add_file(s3_client, stream, project_id, file_name)
            .await
            .unwrap();
        return;
    }
}

#[get("/?<file_name>&<project_id>")]
pub async fn get(
    jwt_authorized: JWTAuthorized,
    s3_client: &State<S3Client>,
    sql_client: &State<Rbatis>,
    file_name: String,
    project_id: String,
) -> Result<Vec<u8>, NotFound<String>> {
    let project = ProjectData::get_owned(&project_id, &jwt_authorized.0, sql_client)
        .await
        .unwrap();

    if let None = project {
        return Err(NotFound("project not found".to_string()));
    }

    return Ok(get_file(s3_client, project_id, file_name).await.unwrap());
}

#[get("/many?<project_id>")]
pub async fn get_many(
    jwt_authorized: JWTAuthorized,
    sql_client: &State<Rbatis>,
    project_id: String,
) -> Result<Json<Vec<File>>, NotFound<&str>> {
    let project_result = ProjectData::get_owned(&project_id, &jwt_authorized.0, sql_client).await;

    match project_result {
        Ok(project) => {
            if let None = project {
                return Err(NotFound("project not found"));
            } else {
                match FileData::get_many(project_id, sql_client).await {
                    Ok(projects) => {
                        return Ok(Json(projects));
                    }
                    Err(_) => Err(NotFound("project not found")),
                }
            }
        }
        Err(_) => Err(NotFound("project not found")),
    }
}
