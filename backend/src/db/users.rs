use rbatis::{crud::CRUD, db::DBExecResult, rbatis::Rbatis, Error};

#[crud_table]
#[derive(Clone, Debug)]
pub struct User {
    pub id: String,
    pub avatar_url: String,
}

impl_field_name_method!(User { id, avatar_url });

pub struct UserManager {}

impl UserManager {
    pub async fn get_user(&self, id: String, sql_client: &Rbatis) -> Result<Option<User>, Error> {
        let wrapper = sql_client.new_wrapper().eq("id", id);
        sql_client.fetch_by_wrapper(wrapper).await
    }

    pub async fn insert_user(
        &self,
        user: &User,
        sql_client: &Rbatis,
    ) -> Result<DBExecResult, rbatis::Error> {
        sql_client.save(user, &[]).await
    }
}
