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
    pub async fn get(&self, id: String, sql_client: &Rbatis) -> Result<Option<File>, Error> {
        let wrapper = sql_client.new_wrapper().eq("id", id);
        sql_client.fetch_by_wrapper(wrapper).await
    }
}
