use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct GetRoomsQuery {
    #[builder(default, setter(strip_option))]
    pub from: Option<usize>,
    #[builder(default, setter(strip_option))]
    pub limit: Option<usize>,
    #[builder(default, setter(strip_option))]
    pub order_by: Option<OrderRoomsBy>,
    #[builder(default, setter(strip_option))]
    #[serde(rename = "dir")]
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JoinRule {
    Public,
    Knock,
    Invite,
    Private,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GuestAccess {
    CanJoin,
    Forbidden,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoryVisibility {
    Invited,
    Joined,
    Shared,
    WorldReadable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomMembers {
    pub members: Vec<OwnedUserId>,
    pub total: usize,
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
