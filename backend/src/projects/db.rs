use rbatis::{
    crud::{Skip, CRUD},
    db::DBExecResult,
    rbatis::Rbatis,
    Error,
};

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
    pub release_id: String,
}

pub struct ProjectData {}

impl ProjectData {
    pub async fn get(id: String, sql_client: &Rbatis) -> Result<Option<Project>, Error> {
        let wrapper = sql_client.new_wrapper().eq("id", id);
        sql_client.fetch_by_wrapper(wrapper).await
    }

    pub async fn create(project: &Project, sql_client: &Rbatis) -> Result<DBExecResult, Error> {
        sql_client.save(&project, &[]).await
    }

    pub async fn get_many(owner_id: String, sql_client: &Rbatis) -> Result<Vec<Project>, Error> {
        let wrapper = sql_client.new_wrapper().eq("owner_id", owner_id);
        sql_client.fetch_list_by_wrapper(wrapper).await
    }

    pub async fn update(
        owner_id: String,
        project: &Project,
        sql_client: &Rbatis,
    ) -> Result<u64, Error> {
        let wrapper = sql_client
            .new_wrapper()
            .eq("id", &project.id)
            .and()
            .eq("owner_id", owner_id);
        sql_client
            .update_by_wrapper(
                project,
                wrapper,
                &[Skip::Column("id"), Skip::Column("owner_id")],
            )
            .await
    }
}
