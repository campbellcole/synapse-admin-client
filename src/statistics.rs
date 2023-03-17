use crate::prelude::*;

#[serde_as]
#[derive(Debug, Clone, Serialize, TypedBuilder)]
pub struct MediaStatisticsQuery<'a> {
    #[builder(default, setter(strip_option))]
    pub limit: Option<usize>,
    #[builder(default, setter(strip_option))]
    pub from: Option<usize>,
    #[builder(default, setter(strip_option))]
    pub order_by: Option<MediaOrderBy>,
    #[serde_as(as = "Option<TimestampMilliSeconds<i64>>")]
    #[builder(default, setter(strip_option))]
    pub from_ts: Option<SystemTime>,
    #[serde_as(as = "Option<TimestampMilliSeconds<i64>>")]
    #[builder(default, setter(strip_option))]
    pub until_ts: Option<SystemTime>,
    #[builder(default, setter(strip_option))]
    pub search_term: Option<&'a str>,
    #[builder(default, setter(strip_option))]
    pub dir: Option<SortDirection>,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaOrderBy {
    UserId,
    Displayname,
    MediaLength,
    MediaCount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaStatistics {
    pub users: Vec<UserMediaStatistics>,
    pub next_token: Option<String>,
    pub total: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMediaStatistics {
    pub displayname: String,
    pub media_count: usize,
    pub media_length: usize,
    pub user_id: OwnedUserId,
}

impl SynapseClient {
    pub async fn get_media_statistics(
        &self,
        query: &MediaStatisticsQuery<'_>,
    ) -> Result<MediaStatistics> {
        execute!(
            self.inner
                .get(endpoint!(self "/statistics/users/media"))
                .query(query)
                .send()
                .await?
                .json::<MatrixResult<MediaStatistics>>()
                .await?
        )
    }
}
