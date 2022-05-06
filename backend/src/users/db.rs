use rbatis::{
    crud::{Skip, CRUD},
    db::DBExecResult,
    rbatis::Rbatis,
    Error,
};
use serde::{Deserialize, Serialize};

/// User struct translated directly to a database row in the table user
#[crud_table]
#[derive(Clone, Debug)]
pub struct User {
    pub id: String,
    pub avatar_url: String,
}

impl User {
    pub fn to_public_user(&self) -> PublicUser {
        PublicUser {
            id: self.id.to_string(),
            avatar_url: self.avatar_url.to_string(),
        }
    }
}

/// Represents information available publicly from the base struct
#[derive(Serialize)]
pub struct PublicUser {
    pub id: String,
    pub avatar_url: String,
}

pub struct UserData {}

/// Contains database interactions for manipulating the user
impl UserData {
    /// Gets any user by id
    pub async fn get_user(&self, id: String, sql_client: &Rbatis) -> Result<Option<User>, Error> {
        let wrapper = sql_client.new_wrapper().eq("id", id);
        sql_client.fetch_by_wrapper(wrapper).await
    }

    /// Inserts user record without any checks
    pub async fn insert_user(
        &self,
        user: &User,
        sql_client: &Rbatis,
    ) -> Result<DBExecResult, rbatis::Error> {
        sql_client.save(user, &[]).await
    }

    /// Inserts user record without any checks
    /// id param we are updating. This should be from validated JWT.
    pub async fn update_user(
        &self,
        id: String,
        user: &User,
        sql_client: &Rbatis,
    ) -> Result<u64, Error> {
        let wrapper = sql_client.new_wrapper().eq("id", id);
        sql_client
            .update_by_wrapper(user, wrapper, &[Skip::Column("id")])
            .await
    }
}
