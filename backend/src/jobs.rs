use mongodb::error::Error;
use mongodb::results::InsertOneResult;
use mongodb::{bson::doc, Client};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
}

pub struct JobManager {}

impl JobManager {
    pub async fn get(&self, id: String, client: &Client) -> Result<Option<Job>, Error> {
        let db = client.database("bevy-studio");
        let collection = db.collection::<Job>("jobs");

        let filter = doc! {"id": id};

        collection.find_one(filter, None).await
    }

    pub async fn insert(
        &self,
        job: &Job,
        client: &Client,
    ) -> Result<InsertOneResult, mongodb::error::Error> {
        let db = client.database("bevy-studio");
        let collection = db.collection::<Job>("jobs");

        collection.insert_one(job, None).await
    }
}
