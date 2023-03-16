use std::time::UNIX_EPOCH;

use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaMXCs {
    pub local: Vec<String>,
    pub remote: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MassQuarantineResponse {
    pub num_quarantined: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaDeletionResponse {
    pub deleted_media: Vec<String>,
    pub total: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurgeCacheResponse {
    pub deleted: usize,
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

    pub async fn quarantine_all_media_in_room(
        &self,
        room_id: &RoomId,
    ) -> Result<MassQuarantineResponse> {
        execute!(
            self.inner
                .post(endpoint!(self format!("/room/{room_id}/media/quarantine")))
                .send()
                .await?
                .json::<MatrixResult<MassQuarantineResponse>>()
                .await?
        )
    }

    // Should this be in the `user` mod? Thinking about grouping these functions by endpoint
    // rather than how the documentation has them grouped.
    pub async fn quarantine_all_media_from_user(
        &self,
        user_id: &UserId,
    ) -> Result<MassQuarantineResponse> {
        execute!(
            self.inner
                .post(endpoint!(self format!("/user/{user_id}/media/quarantine")))
                .send()
                .await?
                .json::<MatrixResult<MassQuarantineResponse>>()
                .await?
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

    pub async fn delete_media(
        &self,
        homeserver: &str,
        media_id: &str,
    ) -> Result<MediaDeletionResponse> {
        execute!(
            self.inner
                .delete(endpoint!(self format!("/media/{homeserver}/{media_id}")))
                .send()
                .await?
                .json::<MatrixResult<MediaDeletionResponse>>()
                .await?
        )
    }

    pub async fn delete_media_where(
        &self,
        before_ts: SystemTime,
        size_gt: Option<usize>,
        keep_profiles: Option<bool>,
    ) -> Result<MediaDeletionResponse> {
        // SAFETY: it is impossible for UNIX_EPOCH to be in the future
        let before_ts = before_ts.duration_since(UNIX_EPOCH).unwrap().as_millis();

        let mut query = vec![("before_ts", before_ts.to_string())];

        if let Some(size_gt) = size_gt {
            query.push(("size_gt", size_gt.to_string()));
        }

        if let Some(keep_profiles) = keep_profiles {
            query.push(("keep_profiles", keep_profiles.to_string()));
        }

        execute!(
            self.inner
                .post(endpoint!(self "/media/delete"))
                .query(&query)
                .send()
                .await?
                .json::<MatrixResult<MediaDeletionResponse>>()
                .await?
        )
    }

    pub async fn delete_media_from_user(&self, _user_id: &UserId) -> Result<MediaDeletionResponse> {
        // this will be an alias to the function in the `user` mod
        todo!()
    }

    pub async fn purge_media_cache(&self, before_ts: SystemTime) -> Result<PurgeCacheResponse> {
        // SAFETY: it is impossible for UNIX_EPOCH to be in the future
        let before_ts = before_ts.duration_since(UNIX_EPOCH).unwrap().as_millis();

        execute!(
            self.inner
                .post(endpoint!(self "/purge_media_cache"))
                .query(&[("before_ts", before_ts.to_string())])
                .send()
                .await?
                .json::<MatrixResult<PurgeCacheResponse>>()
                .await?
        )
    }
}
