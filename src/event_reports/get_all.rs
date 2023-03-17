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

#[skip_serializing_none]
#[derive(Debug, Serialize, TypedBuilder)]
pub struct GetEventReportsQuery<'a> {
    #[builder(default, setter(strip_option))]
    limit: Option<usize>,
    #[builder(default, setter(strip_option))]
    from: Option<i32>,
    #[builder(default, setter(strip_option))]
    dir: Option<SortDirection>,
    #[builder(default, setter(strip_option))]
    user_id: Option<&'a UserId>,
    #[builder(default, setter(strip_option))]
    room_id: Option<&'a RoomId>,
}

impl SynapseClient {
    pub async fn get_event_reports(&self, query: GetEventReportsQuery<'_>) -> Result<EventReports> {
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
}
