use mongodb::error::Error;
use mongodb::results::InsertOneResult;
use mongodb::{bson::doc, Client};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub avatar_url: String,
}

pub struct UserManager {}

impl UserManager {
    pub async fn get_user(&self, id: String, client: &Client) -> Result<Option<User>, Error> {
        let db = client.database("bevy-studio");
        let collection = db.collection::<User>("users");

        let filter = doc! {"id": id};

        collection.find_one(filter, None).await
    }

    pub async fn insert_user(
        &self,
        user: &User,
        client: &Client,
    ) -> Result<InsertOneResult, mongodb::error::Error> {
        let db = client.database("bevy-studio");
        let collection = db.collection::<User>("users");

        collection.insert_one(user, None).await
    }
}
