use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventReports {
    pub event_reports: Vec<EventReport>,
    pub next_token: Option<i32>,
    pub total: usize,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventReport {
    /// The ID of the event.
    pub id: i32,
    /// The time at which the event was reported.
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub received_ts: SystemTime,
    /// The room ID of the event.
    pub room_id: OwnedRoomId,
    /// The name of the room.
    pub name: String,
    /// The ID of the event.
    pub event_id: OwnedEventId,
    /// The ID of the user who reported the event and wrote the reason.
    pub user_id: OwnedUserId,
    /// The user specified reason for the report.
    pub reason: Option<String>,
    /// -100 is 'most offensive' and 0 is 'inoffensive'.
    pub score: Option<i8>,
    /// The user who sent the message that was reported.
    pub sender: OwnedUserId,
    /// The canonical alias of the room.
    pub canonical_alias: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SortDirection {
    /// Oldest first
    #[serde(rename = "f")]
    Ascending,
    /// Newest first
    #[serde(rename = "b")]
    Descending,
}

impl ToString for SortDirection {
    fn to_string(&self) -> String {
        match self {
            SortDirection::Ascending => "f".to_string(),
            SortDirection::Descending => "b".to_string(),
        }
    }
}

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
    // TODO: replace the arguments with a struct
    pub async fn get_event_reports(
        &self,
        limit: Option<usize>,
        from: Option<i32>,
        dir: Option<SortDirection>,
        user_id: Option<&UserId>,
        room_id: Option<&RoomId>,
    ) -> Result<EventReports> {
        let mut query = Vec::new();

        if let Some(limit) = limit {
            query.push(("limit", limit.to_string()));
        }

        if let Some(from) = from {
            query.push(("from", from.to_string()));
        }

        if let Some(dir) = dir {
            query.push(("dir", dir.to_string()));
        }

        if let Some(user_id) = user_id {
            query.push(("user_id", user_id.to_string()));
        }

        if let Some(room_id) = room_id {
            query.push(("room_id", room_id.to_string()));
        }

        execute!(
            self.inner
                .get(endpoint!(self "/event_reports"))
                .query(&query)
                .send()
                .await?
                .json::<MatrixResult<EventReports>>()
                .await?
        )
    }

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
