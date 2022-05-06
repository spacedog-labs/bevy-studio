use rbatis::{
    crud::{Skip, CRUD},
    db::DBExecResult,
    rbatis::Rbatis,
    Error,
};
use rbson::Bson::Null;
use uuid::Uuid;

pub const QUEUED: usize = 1;
pub const RUNNING: usize = 2;
pub const FINISHED: usize = 3;

#[crud_table]
#[derive(Clone, Debug, Default)]
pub struct Job {
    pub id: String,
    pub status: usize,
    pub project_id: String,
    pub log_name: String,
    pub claim_id: String,
}

pub struct JobManager {}

impl JobManager {
    pub async fn dequeue_job(&self, sql_client: &Rbatis) -> Option<Vec<Job>> {
        let claim_id = Uuid::new_v4().to_string();

        let w = sql_client.new_wrapper().eq("status", QUEUED);
        let update_res = sql_client
            .update_by_wrapper(
                &Job {
                    status: RUNNING,
                    claim_id: claim_id,
                    ..Default::default()
                },
                w,
                &[Skip::Value(Null)],
            )
            .await;

        match update_res {
            Ok(num_affected) => {
                if num_affected > 0 {
                    Some(
                        sql_client
                            .fetch_list_by_column("claim_id", &["1"])
                            .await
                            .unwrap(),
                    )
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }
}
