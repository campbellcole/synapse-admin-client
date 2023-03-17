use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerVersion {
    pub server_version: String,
    pub python_version: String,
}

impl SynapseClient {
    pub async fn get_server_version(&self) -> Result<ServerVersion> {
        execute!(
            self.inner
                .get(endpoint!(self "/server_version"))
                .send()
                .await?
                .json::<MatrixResult<ServerVersion>>()
                .await?
        )
    }
}
