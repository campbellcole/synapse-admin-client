use crate::prelude::*;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationToken {
    /// The registration token used to register
    pub token: String,
    /// The number of times this token can be used to register
    pub uses_allowed: Option<usize>,
    /// The number of users currently registering with this token
    pub pending: usize,
    /// The number of users who have completed registration using this token
    pub completed: usize,
    /// The time at which the token will expire and become useless
    #[serde_as(as = "Option<TimestampMilliSeconds<i64>>")]
    pub expiry_time: Option<SystemTime>,
}

impl RegistrationToken {
    pub fn is_valid(&self) -> bool {
        let expired = self
            .expiry_time
            .map_or(false, |exp| SystemTime::now() > exp);

        let usages_reached = self
            .uses_allowed
            .map_or(false, |uses| self.completed + self.pending >= uses);

        !expired && !usages_reached
    }
}

#[serde_as]
#[derive(Debug, Serialize, TypedBuilder)]
pub struct NewToken {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    uses_allowed: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<TimestampMilliSeconds<i64>>")]
    #[builder(default, setter(strip_option))]
    expiry_time: Option<SystemTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    length: Option<u8>,
}

#[serde_as]
#[derive(Debug, Default, Serialize, TypedBuilder)]
pub struct UpdateToken {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    uses_allowed: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<TimestampMilliSeconds<i64>>")]
    #[builder(default, setter(strip_option))]
    expiry_time: Option<SystemTime>,
}

impl SynapseClient {
    /// Get all registration tokens.
    ///
    /// If `valid` is `Some(true)`, only valid tokens will be returned.
    ///
    /// If `valid` is `Some(false)`, only invalid tokens will be returned.
    ///
    /// If `valid` is `None`, all tokens will be returned.
    pub async fn get_tokens(&self, valid: Option<bool>) -> Result<Vec<RegistrationToken>> {
        #[derive(Debug, Deserialize)]
        struct Response {
            registration_tokens: Vec<RegistrationToken>,
        }

        let mut req = self.inner.get(endpoint!(self "/registration_tokens"));

        if let Some(valid) = valid {
            req = req.query(&vec![("valid", valid.to_string())]);
        }

        execute!(req
            .send()
            .await?
            .json::<MatrixResult<Response>>()
            .await?;
            res => res.registration_tokens
        )
    }

    /// Get a registration token by its token (i.e. its code).
    ///
    /// ```rs
    /// let token = client.get_token("token").await?;
    /// assert_eq!(token.token, "token");
    /// ```
    pub async fn get_token(&self, token: &str) -> Result<RegistrationToken> {
        execute!(
            self.inner
                .get(endpoint!(self format!("/registration_tokens/{token}")))
                .send()
                .await?
                .json::<MatrixResult<RegistrationToken>>()
                .await?
        )
    }

    /// Create a new registration token.
    ///
    /// ```rs
    /// let new_token = NewToken::default()
    ///     .with_uses_allowed(5)
    ///     .with_length(32);
    /// let token = client.create_token(new_token).await?;
    /// assert_eq!(token.uses_allowed, Some(5));
    /// assert_eq!(token.token.len(), 32);
    /// ```
    pub async fn create_token(&self, token: NewToken) -> Result<RegistrationToken> {
        execute!(
            self.inner
                .post(endpoint!(self "/registration_tokens/new"))
                .json(&token)
                .send()
                .await?
                .json::<MatrixResult<RegistrationToken>>()
                .await?
        )
    }

    /// Update a registration token.
    ///
    /// ```rs
    /// let update = UpdateToken::default()
    ///    .with_uses_allowed(10);
    /// let token = client.update_token("token", update).await?;
    /// assert_eq!(token.uses_allowed, Some(10));
    /// ```
    pub async fn update_token(
        &self,
        token: &str,
        update: UpdateToken,
    ) -> Result<RegistrationToken> {
        execute!(
            self.inner
                .put(endpoint!(self format!("/registration_tokens/{token}")))
                .json(&update)
                .send()
                .await?
                .json::<MatrixResult<RegistrationToken>>()
                .await?
        )
    }

    /// Delete a registration token.
    pub async fn delete_token(&self, token: &str) -> Result<()> {
        execute!(self
            .inner
            .delete(endpoint!(self format!("/registration_tokens/{token}")))
            .send()
            .await?
            .json::<MatrixResult<EmptyObject>>()
            .await?;
            _r => ()
        )
    }
}

// pub async fn get_tokens(valid: Option<bool>) -> reqwest::Result<Vec<RegistrationToken>> {}
