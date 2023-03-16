use crate::prelude::*;

impl SynapseClient {
    pub async fn add_user_to_room(
        &self,
        room_id: &RoomId,
        user_id: &UserId,
    ) -> Result<OwnedRoomId> {
        #[derive(Serialize)]
        struct RequestBody {
            user_id: String,
        }
        #[derive(Deserialize)]
        struct Response {
            room_id: OwnedRoomId,
        }
        execute!(
            self.inner
                .post(endpoint!(self format!("/join/{room_id}")))
                .json(&RequestBody {
                    user_id: user_id.to_string(),
                })
                .send()
                .await?
                .json::<MatrixResult<Response>>()
                .await?;
            res => res.room_id
        )
    }
}
