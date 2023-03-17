use crate::prelude::*;

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct UpdateAccountValidityBody {
    pub user_id: OwnedUserId,
    #[serde_as(as = "Option<TimestampMilliSeconds<i64>>")]
    #[builder(default, setter(strip_option))]
    pub expiration_ts: Option<SystemTime>,
    #[builder(default, setter(strip_option))]
    pub enable_renewal_emails: Option<bool>,
}

impl SynapseClient {
    /// Update a user's account validity. Returns the expiration timestamp.
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
    ) -> Result<SystemTime> {
        #[serde_as]
        #[derive(Deserialize)]
        struct Response {
            #[serde_as(as = "TimestampMilliSeconds<i64>")]
            pub expiration_ts: SystemTime,
        }
        execute!(
            self.inner
                .post(endpoint!(self "/account_validity/validity"))
                .json(&body)
                .send()
                .await?
                .json::<MatrixResult<Response>>()
                .await?;
            res => res.expiration_ts
        )
    }
}
