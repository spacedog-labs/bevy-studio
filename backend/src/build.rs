use bollard::container::{CreateContainerOptions, LogsOptions};
use bollard::Docker;
use rbatis::rbatis::Rbatis;
use rocket::futures::TryStreamExt;
use rocket::tokio::time::sleep;
use std::time;

pub async fn run_worker(sql_client: &Rbatis) -> Result<(), ()> {
    let poll_rate = time::Duration::from_millis(500);

    loop {
        sleep(poll_rate).await;
    }
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
