use rbatis::{crud::CRUD, db::DBExecResult, rbatis::Rbatis, Error};

#[crud_table]
#[derive(Clone, Debug)]
pub struct Project {
    pub id: String
}

impl_field_name_method!(Project { id });

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
