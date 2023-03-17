use std::time::UNIX_EPOCH;

use crate::prelude::*;

impl SynapseClient {
    pub async fn purge_media_cache(&self, before_ts: SystemTime) -> Result<usize> {
        // SAFETY: it is impossible for UNIX_EPOCH to be in the future
        let before_ts = before_ts.duration_since(UNIX_EPOCH).unwrap().as_millis();

        #[derive(Deserialize)]
        pub struct Response {
            pub deleted: usize,
        }

        execute!(
            self.inner
                .post(endpoint!(self "/purge_media_cache"))
                .query(&[("before_ts", before_ts.to_string())])
                .send()
                .await?
                .json::<MatrixResult<Response>>()
                .await?;
            res => res.deleted
        )
    }
}
