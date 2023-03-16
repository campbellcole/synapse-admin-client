use crate::prelude::*;

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct TimestampToEventQuery {
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    #[serde(rename = "ts")]
    pub timestamp: SystemTime,
    #[serde(rename = "dir")]
    pub direction: Option<SortDirection>,
}

impl SynapseClient {
    pub async fn timestamp_to_event(
        &self,
        room_id: &RoomId,
        query: TimestampToEventQuery,
    ) -> Result<Option<OwnedEventId>> {
        #[derive(Deserialize)]
        struct Response {
            event_id: Option<OwnedEventId>,
        }

        execute!(
            self.inner
                .get(endpoint!(self format!("/rooms/{room_id}/timestamp_to_event")))
                .query(&query)
                .send()
                .await?
                .json::<MatrixResult<Response>>()
                .await?;
            res => res.event_id
        )
    }
}
