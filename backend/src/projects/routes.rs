use crate::auth::JWTAuthorized;
use rbatis::rbatis::Rbatis;
use rocket::response::status::{BadRequest, NotFound};
use rocket::serde::json::Json;
use rocket::{Route, State};
use uuid::Uuid;

use super::{Project, ProjectData};

pub fn routes() -> Vec<Route> {
    routes![get, get_public, create, get_many]
}

#[get("/")]
pub async fn get_many(
    jwt_authorized: JWTAuthorized,
    sql_client: &State<Rbatis>,
) -> Result<Json<Vec<Project>>, NotFound<&str>> {
    match ProjectData::get_many(jwt_authorized.0, sql_client).await {
        Ok(projects) => {
            return Ok(Json(projects));
        }
        Err(_) => Err(NotFound("project not found")),
    }
}

#[get("/<project_id>")]
pub async fn get(
    jwt_authorized: JWTAuthorized,
    project_id: String,
    sql_client: &State<Rbatis>,
) -> Result<Json<Project>, NotFound<&str>> {
    match ProjectData::get(project_id, sql_client).await {
        Ok(project_opt) => {
            if let Some(project) = project_opt {
                if project.owner_id != jwt_authorized.0 {
                    return Err(NotFound("project not found"));
                } else {
                    return Ok(Json(project));
                }
            } else {
                return Err(NotFound("project not found"));
            }
        }
        Err(_) => Err(NotFound("project not found")),
    }
}

#[get("/public/<project_id>")]
async fn get_public(
    project_id: String,
    sql_client: &State<Rbatis>,
) -> Result<Json<Project>, NotFound<&str>> {
    match ProjectData::get(project_id, sql_client).await {
        Ok(project_opt) => {
            if let Some(project) = project_opt {
                if project.is_public != true {
                    return Err(NotFound("project not found"));
                } else {
                    return Ok(Json(project));
                }
            } else {
                return Err(NotFound("project not found"));
            }
        }
        Err(_) => Err(NotFound("project not found")),
    }
}

#[post("/create?<name>")]
async fn create(
    jwt_authorized: JWTAuthorized,
    name: String,
    sql_client: &State<Rbatis>,
) -> Result<Json<Project>, BadRequest<&str>> {
    let project = Project {
        id: Uuid::new_v4().to_string(),
        name: name,
        owner_id: jwt_authorized.0,
        is_public: false,
        entry_point: "index.html".to_string(),
        release_folder: "dist".to_string(),
        is_released: false,
        release_id: "".to_string(),
    };
    match ProjectData::create(&project, sql_client).await {
        Ok(_) => {
            return Ok(Json(project));
        }
        Err(_) => Err(BadRequest(Some("failed to create"))),
    }
}

#[post("/update", data = "<project>")]
async fn update(
    jwt_authorized: JWTAuthorized,
    sql_client: &State<Rbatis>,
    project: Json<Project>,
) -> Result<Json<Project>, BadRequest<&str>> {
    match ProjectData::update(jwt_authorized.0, &project, sql_client).await {
        Ok(_result) => match ProjectData::get(project.id.to_string(), sql_client).await {
            Ok(get_result) => {
                if let Some(project) = get_result {
                    return Ok(Json(project));
                } else {
                    return Err(BadRequest(Some("failed to create")));
                }
            }
            Err(_) => return Err(BadRequest(Some("failed to create"))),
        },
        Err(_) => Err(BadRequest(Some("failed to update"))),
    }
}
