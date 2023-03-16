use std::collections::HashMap;

use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundUpdatesStatus {
    pub enabled: bool,
    pub current_updates: HashMap<String, Vec<BackgroundUpdateStatus>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundUpdateStatus {
    pub name: String,
    pub total_item_count: u64,
    pub total_duration_ms: f64,
    pub average_items_per_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundUpdatesEnabled {
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BackgroundUpdateJob {
    #[serde(rename = "populate_stats_process_rooms")]
    PopulateStatsProcessRooms,
    #[serde(rename = "regenerate_directory")]
    RegenerateDirectory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartBackgroundUpdateJob {
    pub job_name: BackgroundUpdateJob,
}

impl SynapseClient {
    pub async fn get_background_updates_status(&self) -> Result<BackgroundUpdatesStatus> {
        execute!(
            self.inner
                .get(endpoint!(self "/_synapse/admin/v1/background_updates/status"))
                .send()
                .await?
                .json::<MatrixResult<BackgroundUpdatesStatus>>()
                .await?
        )
    }

    pub async fn set_background_updates_enabled(
        &self,
        enabled: bool,
    ) -> Result<BackgroundUpdatesEnabled> {
        let background_updates_enabled = BackgroundUpdatesEnabled { enabled };

        execute!(
            self.inner
                .post(endpoint!(self "/_synapse/admin/v1/background_updates/enabled"))
                .json(&background_updates_enabled)
                .send()
                .await?
                .json::<MatrixResult<BackgroundUpdatesEnabled>>()
                .await?
        )
    }

    pub async fn get_background_updates_enabled(&self) -> Result<BackgroundUpdatesEnabled> {
        execute!(
            self.inner
                .get(endpoint!(self "/_synapse/admin/v1/background_updates/enabled"))
                .send()
                .await?
                .json::<MatrixResult<BackgroundUpdatesEnabled>>()
                .await?
        )
    }

    /// The documentation does not say what this endpoint returns, so I am guessing it returns the
    /// same as `get_background_updates_status`. I could use the `EmptyObject` type, but that wouldn't
    /// fail if the endpoint returned something else. I want this to fail if the endpoint returns
    /// something else.
    pub async fn start_background_update_job(
        &self,
        job_name: BackgroundUpdateJob,
    ) -> Result<BackgroundUpdateStatus> {
        let start_background_update_job = StartBackgroundUpdateJob { job_name };

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
