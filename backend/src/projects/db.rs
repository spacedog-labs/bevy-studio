use rbatis::{crud::CRUD, db::DBExecResult, rbatis::Rbatis, Error};

#[crud_table]
#[derive(Clone, Debug)]
pub struct Project {
    pub id: String,
    // name of the project
    pub name: String,
    // owner of the project, if not public, only owner has access
    pub owner_id: String,
    // whether or not the project is publicly available and forkable
    pub is_public: bool,
    // this property represents the html entrypoint after the build
    pub entry_point: String,
    // this property represents the folder to upload as release
    pub release_folder: String,
    // if true the release is routable
    pub is_released: bool
}

#[crud_table]
#[derive(Clone, Debug)]
pub struct File {
    pub id: String,
    // id of the project
    pub project_id: String,
    // name of the file in conjuction with its directory
    // i.e: cargo.toml or src/main.rs or src/module/test.rs
    pub name: String,
    pub extension: FileType
}

enum FileType {
    Text,
    Binary
}

pub struct ProjectManager {}

impl ProjectManager {
    pub async fn get_by_id(&self, id: String, sql_client: &Rbatis) -> Result<Option<Project>, Error> {
        let wrapper = sql_client.new_wrapper().eq("id", id);
        sql_client.fetch_by_wrapper(wrapper).await
    }

    pub async fn insert(
        &self,
        project: &Project,
        sql_client: &Rbatis,
    ) -> Result<DBExecResult, rbatis::Error> {
        sql_client.save(project, &[]).await
    }
}
