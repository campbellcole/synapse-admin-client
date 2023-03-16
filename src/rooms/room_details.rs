use matrix_sdk::ruma::events::room::{
    guest_access::GuestAccess, history_visibility::HistoryVisibility, join_rules::JoinRule,
};

use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomDetails {
    pub room_id: OwnedRoomId,
    pub name: String,
    pub topic: Option<String>,
    pub avatar: Option<String>,
    pub canonical_alias: Option<String>,
    pub joined_members: usize,
    pub joined_local_members: usize,
    pub joined_local_devices: usize,
    pub version: String,
    pub creator: OwnedUserId,
    pub encryption: Option<String>,
    pub federatable: bool,
    pub public: bool,
    pub join_rules: JoinRule,
    pub guest_access: Option<GuestAccess>,
    pub history_visibility: HistoryVisibility,
    pub state_events: usize,
    pub room_type: Option<String>,
    pub forgotten: bool,
}

impl SynapseClient {
    pub async fn get_room(&self, room_id: &RoomId) -> Result<RoomDetails> {
        execute!(
            self.inner
                .get(endpoint!(self format!("/rooms/{room_id}")))
                .send()
                .await?
                .json::<MatrixResult<RoomDetails>>()
                .await?
        )
    }
}
