use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BackgroundUpdatesEnabled {
    pub enabled: bool,
}

impl SynapseClient {
    /// Allows pausing background updates
    ///
    /// https://matrix-org.github.io/synapse/latest/usage/administration/admin_api/background_updates.html#enabled
    pub async fn set_background_updates_enabled(&self, enabled: bool) -> Result<bool> {
        let background_updates_enabled = BackgroundUpdatesEnabled { enabled };

        execute!(
            self.inner
                .post(endpoint!(self "/_synapse/admin/v1/background_updates/enabled"))
                .json(&background_updates_enabled)
                .send()
                .await?
                .json::<MatrixResult<BackgroundUpdatesEnabled>>()
                .await?;
            res => res.enabled
        )
    }

    /// Gets the status of background updates
    ///
    /// https://matrix-org.github.io/synapse/latest/usage/administration/admin_api/background_updates.html#enabled
    pub async fn get_background_updates_enabled(&self) -> Result<bool> {
        execute!(
            self.inner
                .get(endpoint!(self "/_synapse/admin/v1/background_updates/enabled"))
                .send()
                .await?
                .json::<MatrixResult<BackgroundUpdatesEnabled>>()
                .await?;
            res => res.enabled
        )
    }
}
