use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomMembers {
    pub members: Vec<OwnedUserId>,
    pub total: usize,
}

impl SynapseClient {
    pub async fn get_room_members(&self, room_id: &RoomId) -> Result<RoomMembers> {
        execute!(
            self.inner
                .get(endpoint!(self format!("/rooms/{room_id}/members")))
                .send()
                .await?
                .json::<MatrixResult<RoomMembers>>()
                .await?
        )
    }
}
