use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, Clone, Serialize, TypedBuilder)]
pub struct ServerNoticeBody<'a, T: Serialize + Debug> {
    pub user_id: &'a UserId,
    pub content: T,
    #[serde(rename = "type")]
    pub notice_type: Option<&'a str>,
    pub state_key: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MessageContent<'a> {
    pub body: &'a str,
    pub msgtype: &'a str,
}

pub type MessageNotice<'a> = ServerNoticeBody<'a, MessageContent<'a>>;

#[derive(Deserialize)]
struct Response {
    pub event_id: OwnedEventId,
}

impl SynapseClient {
    pub async fn send_server_notice<T: Serialize + Debug>(
        &self,
        notice: ServerNoticeBody<'_, T>,
    ) -> Result<OwnedEventId> {
        execute!(
            self.inner
                .post(endpoint!(self format!("/send_server_notice")))
                .json(&notice)
                .send()
                .await?
                .json::<MatrixResult<Response>>()
                .await?;
            res => res.event_id
        )
    }

    pub async fn update_server_notice<T: Serialize + Debug>(
        &self,
        txn_id: &str,
        notice: ServerNoticeBody<'_, T>,
    ) -> Result<OwnedEventId> {
        execute!(
            self.inner
                .put(endpoint!(self format!("/send_server_notice/{txn_id}")))
                .json(&notice)
                .send()
                .await?
                .json::<MatrixResult<Response>>()
                .await?;
            res => res.event_id
        )
    }
}
