use crate::prelude::*;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleEventReport {
    /// The ID of the event that was reported.
    pub event_id: OwnedEventId,
    // TODO: figure out if ruma can deserialize this into the appropriate type
    /// The event that was reported.
    pub event_json: serde_json::Value,
    /// The ID of the report.
    pub id: i32,
    /// The user specified reason for the report.
    pub reason: Option<String>,
    /// -100 is 'most offensive' and 0 is 'inoffensive'.
    pub score: Option<i8>,
    /// The time at which the event was reported.
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub received_ts: SystemTime,
    /// The canonical alias of the room.
    pub canonical_alias: Option<String>,
    /// The room ID of the event.
    pub room_id: OwnedRoomId,
    /// The name of the room.
    pub name: String,
    /// The user who sent the message that was reported.
    pub sender: OwnedUserId,
    /// The ID of the user who reported the event and wrote the reason.
    pub user_id: OwnedUserId,
}

impl SynapseClient {
    pub async fn get_event_report(&self, report_id: i32) -> Result<SingleEventReport> {
        execute!(
            self.inner
                .get(endpoint!(self format!("/event_reports/{}", report_id)))
                .send()
                .await?
                .json::<MatrixResult<SingleEventReport>>()
                .await?
        )
    }
}
