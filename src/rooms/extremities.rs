use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomExtremities {
    pub count: usize,
    pub results: Vec<RoomExtremity>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomExtremity {
    pub event_id: OwnedEventId,
    pub state_group: i64,
    pub depth: i64,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub received_ts: SystemTime,
}

impl SynapseClient {
    pub async fn get_room_extremities(&self, room_id: &RoomId) -> Result<RoomExtremities> {
        execute!(
            self.inner
                .get(endpoint!(self format!("/rooms/{room_id}/forward_extremities")))
                .send()
                .await?
                .json::<MatrixResult<RoomExtremities>>()
                .await?
        )
    }

    pub async fn delete_room_extremities(&self, room_id: &RoomId) -> Result<usize> {
        #[derive(Deserialize)]
        struct Response {
            deleted: usize,
        }
        execute!(
            self.inner
                .delete(endpoint!(self format!("/rooms/{room_id}/forward_extremities")))
                .send()
                .await?
                .json::<MatrixResult<Response>>()
                .await?;
            res => res.deleted
        )
    }
}
