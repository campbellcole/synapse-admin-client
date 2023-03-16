use std::collections::HashMap;

use crate::prelude::*;

use super::BackgroundUpdateStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundUpdateStatuses {
    pub enabled: bool,
    pub current_updates: HashMap<String, Vec<BackgroundUpdateStatus>>,
}

impl SynapseClient {
    /// Get the statuses of any currently running background updates
    ///
    /// https://matrix-org.github.io/synapse/latest/usage/administration/admin_api/background_updates.html#status
    pub async fn get_background_update_statuses(&self) -> Result<BackgroundUpdateStatuses> {
        execute!(
            self.inner
                .get(endpoint!(self "/_synapse/admin/v1/background_updates/status"))
                .send()
                .await?
                .json::<MatrixResult<BackgroundUpdateStatuses>>()
                .await?
        )
    }
}
