use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MassQuarantineResponse {
    pub num_quarantined: usize,
}

impl SynapseClient {
    pub async fn quarantine_media(&self, homeserver: &str, media_id: &str) -> Result<()> {
        execute!(
            self.inner
                .post(endpoint!(self format!("/media/quarantine/{homeserver}/{media_id}")))
                .send()
                .await?
                .json::<MatrixResult<EmptyObject>>()
                .await?;
            _r => ()
        )
    }

    pub async fn unquarantine_media(&self, homeserver: &str, media_id: &str) -> Result<()> {
        execute!(
            self.inner
                .post(endpoint!(self format!("/media/unquarantine/{homeserver}/{media_id}")))
                .send()
                .await?
                .json::<MatrixResult<EmptyObject>>()
                .await?;
            _r => ()
        )
    }

    pub async fn quarantine_all_media_in_room(&self, room_id: &RoomId) -> Result<usize> {
        execute!(
            self.inner
                .post(endpoint!(self format!("/room/{room_id}/media/quarantine")))
                .send()
                .await?
                .json::<MatrixResult<MassQuarantineResponse>>()
                .await?;
            r => r.num_quarantined
        )
    }

    // Should this be in the `user` mod? Thinking about grouping these functions by endpoint
    // rather than how the documentation has them grouped.
    pub async fn quarantine_all_media_from_user(&self, user_id: &UserId) -> Result<usize> {
        execute!(
            self.inner
                .post(endpoint!(self format!("/user/{user_id}/media/quarantine")))
                .send()
                .await?
                .json::<MatrixResult<MassQuarantineResponse>>()
                .await?;
            r => r.num_quarantined
        )
    }

    pub async fn protect_media_from_quarantine(&self, media_id: &str) -> Result<()> {
        execute!(
            self.inner
                .post(endpoint!(self format!("/media/protect/{media_id}")))
                .send()
                .await?
                .json::<MatrixResult<EmptyObject>>()
                .await?;
            _r => ()
        )
    }

    pub async fn unprotect_media_from_quarantine(&self, media_id: &str) -> Result<()> {
        execute!(
            self.inner
                .post(endpoint!(self format!("/media/unprotect/{media_id}")))
                .send()
                .await?
                .json::<MatrixResult<EmptyObject>>()
                .await?;
            _r => ()
        )
    }
}
