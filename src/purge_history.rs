use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurgeHistoryResponse {
    pub purge_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurgeHistoryStatus {
    pub status: PurgeStatus,
    pub error: Option<String>,
}

impl SynapseClient {
    pub async fn purge_room_history(
        &self,
        room_id: &RoomId,
        event_id: Option<&EventId>,
        delete_local_events: Option<bool>,
    ) -> Result<PurgeHistoryResponse> {
        #[derive(Serialize)]
        struct RequestBody {
            pub delete_local_events: bool,
        }

        let mut req = self.inner.post(endpoint!(self format!(
            "/purge_history/{room_id}{}",
            event_id.map(|e| format!("/{}", e)).unwrap_or_default()
        )));

        if let Some(delete_local_events) = delete_local_events {
            req = req.json(&RequestBody {
                delete_local_events,
            });
        }

        execute!(
            req.send()
                .await?
                .json::<MatrixResult<PurgeHistoryResponse>>()
                .await?
        )
    }

    pub async fn get_purge_status(&self, purge_id: &str) -> Result<PurgeHistoryStatus> {
        execute!(
            self.inner
                .get(endpoint!(self format!("/purge_history_status/{purge_id}")))
                .send()
                .await?
                .json::<MatrixResult<PurgeHistoryStatus>>()
                .await?
        )
    }
}
