use matrix_sdk::ruma::events::AnySyncStateEvent;

use crate::prelude::*;

#[derive(Debug, Clone, Deserialize)]
pub struct RoomState {
    pub state: Vec<AnySyncStateEvent>,
}

impl SynapseClient {
    pub async fn get_room_state(&self, room_id: &RoomId) -> Result<RoomState> {
        execute!(
            self.inner
                .get(endpoint!(self format!("/rooms/{room_id}/state")))
                .send()
                .await?
                .json::<MatrixResult<RoomState>>()
                .await?
        )
    }
}
