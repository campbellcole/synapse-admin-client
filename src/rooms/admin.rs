use crate::prelude::*;

impl SynapseClient {
    /// Add either the current user or `user_id` as a room admin.
    pub async fn make_room_admin(&self, room_id: &RoomId, user_id: Option<&UserId>) -> Result<()> {
        #[derive(Serialize)]
        struct Body<'a> {
            user_id: &'a UserId,
        }

        let mut req = self
            .inner
            .post(endpoint!(self format!("/rooms/{room_id}/make_room_admin")));

        if let Some(user_id) = user_id {
            req = req.json(&Body { user_id });
        }

        execute!(
            req.send()
                .await?
                .json::<MatrixResult<EmptyObject>>()
                .await?;
            _r => ()
        )
    }
}
