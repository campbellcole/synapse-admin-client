use crate::prelude::*;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct UpdateAccountValidityBody {
    pub user_id: OwnedUserId,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<TimestampMilliSeconds<i64>>")]
    #[builder(default, setter(strip_option))]
    pub expiration_ts: Option<SystemTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub enable_renewal_emails: Option<bool>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountValidityResponse {
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub expiration_ts: SystemTime,
}

impl SynapseClient {
    /// Update a user's account validity
    ///
    /// ```rs
    /// let some_user_id = "@user:homeserver.org".parse().unwrap();
    /// let body = UpdateAccountValidity::builder()
    ///     .user_id(some_user_id)
    ///     .build();
    /// let client.update_account_validity(body).await?;
    /// ```
    pub async fn update_account_validity(
        &self,
        body: UpdateAccountValidityBody,
    ) -> Result<AccountValidityResponse> {
        execute!(
            self.inner
                .post(endpoint!(self "/account_validity/validity"))
                .json(&body)
                .send()
                .await?
                .json::<MatrixResult<AccountValidityResponse>>()
                .await?
        )
    }
}
