use rbatis::{crud::CRUD, db::DBExecResult, rbatis::Rbatis, Error};

#[crud_table]
#[derive(Clone, Debug)]
pub struct File {
    pub id: String,
    // id of the project
    pub project_id: String,
    // name of the file in conjuction with its directory
    // i.e: cargo.toml or src/main.rs or src/module/test.rs
    pub name: String,
}

pub struct FileData {}

impl FileData {
    pub async fn get(
        project_id: &String,
        file_name: &String,
        sql_client: &Rbatis,
    ) -> Result<Option<File>, Error> {
        let wrapper = sql_client
            .new_wrapper()
            .eq("project_id", project_id)
            .eq("name", file_name);
        sql_client.fetch_by_wrapper(wrapper).await
    }

    pub async fn get_many(project_id: String, sql_client: &Rbatis) -> Result<Vec<File>, Error> {
        let wrapper = sql_client.new_wrapper().eq("project_id", project_id);
        sql_client.fetch_list_by_wrapper(wrapper).await
    }

    pub async fn create(file: &File, sql_client: &Rbatis) -> Result<DBExecResult, Error> {
        sql_client.save(&file, &[]).await
    }
}
