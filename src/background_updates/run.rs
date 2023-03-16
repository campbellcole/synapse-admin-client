use crate::prelude::*;

use super::BackgroundUpdateStatus;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BackgroundUpdateJob {
    #[serde(rename = "populate_stats_process_rooms")]
    PopulateStatsProcessRooms,
    #[serde(rename = "regenerate_directory")]
    RegenerateDirectory,
}

impl SynapseClient {
    /// WARNING: This endpoint has not been checked yet!
    /// I am guessing the return type, please report an issue if this does not work!
    ///
    /// Start a background job
    ///
    /// https://matrix-org.github.io/synapse/latest/usage/administration/admin_api/background_updates.html#run
    pub async fn start_background_update_job(
        &self,
        job_name: BackgroundUpdateJob,
    ) -> Result<BackgroundUpdateStatus> {
        #[derive(Serialize)]
        pub struct Body {
            job_name: BackgroundUpdateJob,
        }

        let start_background_update_job = Body { job_name };

        execute!(
            self.inner
                .post(endpoint!(self "/_synapse/admin/v1/background_updates/start_job"))
                .json(&start_background_update_job)
                .send()
                .await?
                .json::<MatrixResult<BackgroundUpdateStatus>>()
                .await?
        )
    }
}
