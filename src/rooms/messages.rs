use matrix_sdk::ruma::{
    api::client::filter::RoomEventFilter,
    events::{AnyMessageLikeEvent, AnyStateEvent},
};

use crate::prelude::*;

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, TypedBuilder)]
pub struct GetRoomMessagesQuery<'a> {
    pub from: String,
    #[builder(default, setter(strip_option))]
    pub to: Option<String>,
    #[builder(default, setter(strip_option))]
    pub limit: Option<usize>,
    #[builder(default, setter(strip_option))]
    pub filter: Option<RoomEventFilter<'a>>,
    #[serde(rename = "dir")]
    pub direction: Option<SortDirection>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RoomMessages {
    pub chunk: Vec<AnyMessageLikeEvent>,
    pub start: String,
    pub end: Option<String>,
    pub state: Option<Vec<AnyStateEvent>>,
}

impl SynapseClient {
    pub async fn get_room_messages(
        &self,
        room_id: &RoomId,
        query: GetRoomMessagesQuery<'_>,
    ) -> Result<RoomMessages> {
        execute!(
            self.inner
                .get(endpoint!(self format!("/rooms/{room_id}/messages")))
                .query(&query)
                .send()
                .await?
                .json::<MatrixResult<RoomMessages>>()
                .await?
        )
    }
}
