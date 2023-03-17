use matrix_sdk::ruma::events::room::{
    guest_access::GuestAccess, history_visibility::HistoryVisibility, join_rules::JoinRule,
};

use crate::prelude::*;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct GetRoomsQuery {
    #[builder(default, setter(strip_option))]
    pub from: Option<usize>,
    #[builder(default, setter(strip_option))]
    pub limit: Option<usize>,
    #[builder(default, setter(strip_option))]
    pub order_by: Option<OrderRoomsBy>,
    #[serde(rename = "dir")]
    #[builder(default, setter(strip_option))]
    pub direction: Option<SortDirection>,
    #[builder(default, setter(strip_option))]
    pub search_term: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderRoomsBy {
    Name,
    CanonicalAlias,
    JoinedMembers,
    JoinedLocalMembers,
    Version,
    Creator,
    Encryption,
    Federatable,
    Public,
    JoinRules,
    GuestAccess,
    HistoryVisibility,
    StateEvents,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomsQueryResponse {
    pub rooms: Vec<Room>,
    pub offset: usize,
    pub total_rooms: usize,
    pub next_batch: Option<usize>,
    pub prev_batch: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub room_id: OwnedRoomId,
    pub name: String,
    pub canonical_alias: Option<String>,
    pub joined_members: usize,
    pub joined_local_members: usize,
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
}

impl SynapseClient {
    pub async fn get_rooms(&self, query: GetRoomsQuery) -> Result<RoomsQueryResponse> {
        execute!(
            self.inner
                .get(endpoint!(self "/rooms"))
                .query(&query)
                .send()
                .await?
                .json::<MatrixResult<RoomsQueryResponse>>()
                .await?
        )
    }
}
