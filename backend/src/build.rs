use bollard::container::{CreateContainerOptions, LogsOptions, StatsOptions};
use bollard::Docker;
use mongodb::bson::{doc, Document};
use mongodb::change_stream::event::ChangeNamespace;
use mongodb::options::ChangeStreamOptions;
use mongodb::options::FullDocumentType;
use mongodb::options::SelectionCriteria;
use mongodb::{options::ClientOptions, Client};
use rocket::futures::{StreamExt, TryStreamExt};
use rocket::tokio;

use crate::users::User;

pub async fn run_worker(client: Client) -> Result<(), mongodb::error::Error> {
    let db = client.database("bevy-studio");
    let collection = db.collection::<User>("users");

    loop {}
}

async fn run_container() -> String {
    let docker = Docker::connect_with_local_defaults().unwrap();
    let container_id = docker
        .create_container(
            Some(CreateContainerOptions {
                name: "test_container",
            }),
            bollard::container::Config {
                image: Some("test"),
                ..Default::default()
            },
        )
        .await
        .unwrap()
        .id;
    docker
        .start_container::<String>(&container_id, None)
        .await
        .unwrap();

    let _vec = &docker
        .logs(
            &container_id,
            Some(LogsOptions {
                follow: true,
                stdout: true,
                stderr: false,
                tail: "all".to_string(),
                ..Default::default()
            }),
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    let vec = &docker
        .logs(
            &container_id,
            Some(LogsOptions {
                follow: true,
                stdout: true,
                stderr: false,
                tail: "all".to_string(),
                ..Default::default()
            }),
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    let value = vec.get(0).unwrap();
    println!("{}", value.to_string());

    "working".to_string()
}
