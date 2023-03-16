use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaMXCs {
    pub local: Vec<String>,
    pub remote: Vec<String>,
}

impl SynapseClient {
    pub async fn get_media_for_room(&self, room_id: &RoomId) -> Result<MediaMXCs> {
        execute!(
            self.inner
                .get(endpoint!(self format!("/room/{room_id}/media")))
                .send()
                .await?
                .json::<MatrixResult<MediaMXCs>>()
                .await?
        )
    }
}
