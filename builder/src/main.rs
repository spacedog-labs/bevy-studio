use async_trait::async_trait;
use clap::Parser;
use rusoto_core::{HttpClient, Region};
use rusoto_credential::{AwsCredentials, ProvideAwsCredentials};
use rusoto_s3::{GetObjectRequest, ListObjectsV2Request, S3Client, S3};
use std::{
    io::{self, Write},
    path::Path,
    process::Command,
};
use tokio::io::AsyncReadExt;

const BUILD_DIR: &str = "build";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    secret: String,

    #[clap(short, long)]
    project_id: String,

    #[clap(short, long)]
    build_number: Option<String>,

    #[clap(short, long)]
    output_folder: Option<String>,
}

struct DOCredentials(pub String);

#[async_trait]
impl ProvideAwsCredentials for DOCredentials {
    async fn credentials(
        &self,
    ) -> Result<rusoto_credential::AwsCredentials, rusoto_credential::CredentialsError> {
        Ok(AwsCredentials::new(
            "WMHTWNDNW2IRNI6FWPV5",
            &self.0,
            None,
            None,
        ))
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let client = S3Client::new_with(
        HttpClient::new().unwrap(),
        DOCredentials(args.secret),
        Region::Custom {
            name: "sfo3".to_string(),
            endpoint: "sfo3.digitaloceanspaces.com".to_string(),
        },
    );

    setup_build_dir().unwrap();

    let request = ListObjectsV2Request {
        bucket: "bevy-studio-projects".to_string(),
        prefix: Some(args.project_id),
        continuation_token: None,
        delimiter: None,
        encoding_type: None,
        expected_bucket_owner: None,
        fetch_owner: None,
        max_keys: None,
        request_payer: None,
        start_after: None,
    };

    let files = &client.list_objects_v2(request).await.unwrap();

    for file in files.contents.clone().unwrap().iter() {
        let file_key = file.key.clone().unwrap();
        println!("{}", file_key);
        let file = download_file(&file_key, &client).await;
        create_file(file, &file_key).unwrap();
    }

    let output = Command::new("cargo")
        .arg("build")
        .arg("--manifest-path=build/Cargo.toml")
        .output()
        .expect("failed to execute process");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

fn setup_build_dir() -> std::io::Result<()> {
    if Path::new(BUILD_DIR).exists() {
        std::fs::remove_dir_all(BUILD_DIR)?;
    }

    std::fs::create_dir(BUILD_DIR)?;

    Ok(())
}

async fn download_file(file: &String, client: &S3Client) -> Vec<u8> {
    let get_output = client
        .get_object(GetObjectRequest {
            bucket: "bevy-studio-projects".to_string(),
            key: file.clone(),
            ..Default::default()
        })
        .await
        .unwrap();

    let mut buf: Vec<u8> = Vec::new();

    let _size = get_output
        .body
        .unwrap()
        .into_async_read()
        .read_to_end(&mut buf)
        .await;
    buf
}

fn create_file(bytes: Vec<u8>, file_name: &String) -> std::io::Result<()> {
    let mut directories: Vec<&str> = file_name.split('/').collect();
    directories.remove(0);
    let actual_file = directories.join("/");
    directories.pop();
    ensure_file_directory(directories).unwrap();

    println!("{}", actual_file);
    let mut file = std::fs::File::create(format!("{}/{}", BUILD_DIR, actual_file))?;
    file.write_all(&bytes[..])?;
    Ok(())
}

fn ensure_file_directory(directories: Vec<&str>) -> std::io::Result<()> {
    let mut cur_path = BUILD_DIR.to_string();

    for dir in directories {
        println!("{}", cur_path);
        let path_to_check = format!("{}/{}", cur_path, dir);

        if !Path::new(&path_to_check).exists() {
            std::fs::create_dir(path_to_check.clone())?;
        }

        cur_path = path_to_check;
    }

    Ok(())
}
