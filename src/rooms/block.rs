use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomBlocked {
    pub blocked: bool,
    pub user_id: Option<OwnedUserId>,
}

impl SynapseClient {
    pub async fn set_room_blocked(&self, room_id: &RoomId, blocked: bool) -> Result<bool> {
        #[derive(Serialize, Deserialize)]
        struct BlockedState {
            block: bool,
        }
        execute!(
            self.inner
                .put(endpoint!(self format!("/rooms/{room_id}/block")))
                .json(&BlockedState { block: blocked })
                .send()
                .await?
                .json::<MatrixResult<BlockedState>>()
                .await?;
            res => res.block
        )
    }

    pub async fn get_room_blocked(&self, room_id: &RoomId) -> Result<RoomBlocked> {
        execute!(
            self.inner
                .get(endpoint!(self format!("/rooms/{room_id}/block")))
                .send()
                .await?
                .json::<MatrixResult<RoomBlocked>>()
                .await?
        )
    }
}
