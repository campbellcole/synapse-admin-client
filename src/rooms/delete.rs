use crate::prelude::*;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, TypedBuilder)]
pub struct DeleteRoomBody<'a> {
    #[builder(default, setter(strip_option))]
    pub new_room_user_id: Option<&'a UserId>,
    #[builder(default, setter(strip_option))]
    pub room_name: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    pub message: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    pub block: Option<bool>,
    #[builder(default, setter(strip_option))]
    pub purge: Option<bool>,
    #[builder(default, setter(strip_option))]
    pub force_purge: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRoomV1Response {
    pub kicked_users: Vec<OwnedUserId>,
    pub failed_to_kick_users: Vec<OwnedUserId>,
    pub local_aliases: Vec<OwnedRoomId>,
    pub new_room_id: Option<OwnedRoomId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRoomPurgeStatus {
    /// This field is always `Some(_)` if the status was retrieved by room ID,
    /// and always `None` if the status was retrieved by purge ID.
    pub delete_id: Option<String>,
    pub status: PurgeStatus,
    pub error: Option<String>,
    pub shutdown_room: Option<DeleteRoomV1Response>,
}

impl SynapseClient {
    #[deprecated]
    pub async fn delete_room_v1(
        &self,
        room_id: &RoomId,
        body: DeleteRoomBody<'_>,
    ) -> Result<DeleteRoomV1Response> {
        execute!(
            self.inner
                .delete(endpoint!(self format!("/rooms/{room_id}")))
                .json(&body)
                .send()
                .await?
                .json::<MatrixResult<DeleteRoomV1Response>>()
                .await?
        )
    }

    /// Returns the purge ID for deleting the room. Check the purge status with `get_purge_status` or `get_room_delete_status`.
    pub async fn delete_room_v2(
        &self,
        room_id: &RoomId,
        body: DeleteRoomBody<'_>,
    ) -> Result<String> {
        #[derive(Deserialize)]
        pub struct Response {
            pub delete_id: String,
        }
        execute!(
            self.inner
                .delete(endpoint!(v2 self format!("/rooms/{room_id}")))
                .json(&body)
                .send()
                .await?
                .json::<MatrixResult<Response>>()
                .await?;
            res => res.delete_id
        )
    }

    pub async fn get_room_delete_status(
        &self,
        room_id: &RoomId,
    ) -> Result<Vec<DeleteRoomPurgeStatus>> {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Response {
            pub results: Vec<DeleteRoomPurgeStatus>,
        }
        execute!(
            self.inner
                .get(endpoint!(v2 self format!("/rooms/{room_id}/delete_status")))
                .send()
                .await?
                .json::<MatrixResult<Response>>()
                .await?;
            res => res.results
        )
    }

    pub async fn get_delete_status_by_id(&self, delete_id: &str) -> Result<DeleteRoomPurgeStatus> {
        execute!(
            self.inner
                .get(endpoint!(v2 self format!("/rooms/delete_status/{delete_id}")))
                .send()
                .await?
                .json::<MatrixResult<DeleteRoomPurgeStatus>>()
                .await?
        )
    }
}
