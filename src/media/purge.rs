use std::time::UNIX_EPOCH;

use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurgeCacheResponse {
    pub deleted: usize,
}

impl SynapseClient {
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
