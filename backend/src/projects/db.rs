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
    pub is_released: bool,
    pub release_id: String
}

pub struct ProjectData {}

impl ProjectData {
    pub async fn get(&self, id: String, sql_client: &Rbatis) -> Result<Option<Project>, Error> {
        let wrapper = sql_client.new_wrapper().eq("id", id);
        sql_client.fetch_by_wrapper(wrapper).await
    }

    pub async fn create(&self, project: Project, sql_client: &Rbatis) -> Result<(), Error> {

    }

    pub async fn get_many(&self, owner_id: String, public: bool, sql_client: &Rbatis) -> Result<Vec<Project>, Error>{

    }

    pub async fn set_display(&self, project_id: String, public: bool, sql_client: &Rbatis) -> Result<(), Error> {

    }

    pub async fn set_name(&self, project_id: String, name: String, sql_client: &Rbatis) -> Result<(), Error> {
        
    }

    pub async fn set_release_folder(&self, project_id: String, name: String, sql_client: &Rbatis) -> Result<(), Error> {
        
    }
}
