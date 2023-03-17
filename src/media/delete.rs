use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaDeletionResponse {
    pub deleted_media: Vec<String>,
    pub total: usize,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, TypedBuilder)]
pub struct DeleteMediaQuery {
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    before_ts: SystemTime,
    #[builder(default, setter(strip_option))]
    size_gt: Option<usize>,
    #[builder(default, setter(strip_option))]
    keep_profiles: Option<bool>,
}

impl SynapseClient {
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
        query: DeleteMediaQuery,
    ) -> Result<MediaDeletionResponse> {
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
}
