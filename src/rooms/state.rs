use matrix_sdk::ruma::events::AnyStateEvent;

use crate::prelude::*;

impl SynapseClient {
    pub async fn get_room_state(&self, room_id: &RoomId) -> Result<Vec<AnyStateEvent>> {
        #[derive(Deserialize)]
        pub struct Response {
            pub state: Vec<AnyStateEvent>,
        }
        execute!(
            self.inner
                .get(endpoint!(self format!("/rooms/{room_id}/state")))
                .send()
                .await?
                .json::<MatrixResult<Response>>()
                .await?;
            res => res.state
        )
    }
}
