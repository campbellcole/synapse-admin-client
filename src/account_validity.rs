use crate::prelude::*;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAccountValidity {
    pub user_id: OwnedUserId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<TimestampMilliSeconds<i64>>")]
    pub expiration_ts: Option<SystemTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_renewal_emails: Option<bool>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountValidityResponse {
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub expiration_ts: SystemTime,
}

impl SynapseClient {
    pub async fn update_account_validity(
        &self,
        user_id: OwnedUserId,
        expiration_ts: Option<SystemTime>,
        enable_renewal_emails: Option<bool>,
    ) -> Result<AccountValidityResponse> {
        let update_account_validity = UpdateAccountValidity {
            user_id,
            expiration_ts,
            enable_renewal_emails,
        };

        execute!(
            self.inner
                .post(endpoint!(self "/account_validity/validity"))
                .json(&update_account_validity)
                .send()
                .await?
                .json::<MatrixResult<AccountValidityResponse>>()
                .await?
        )
    }
}
