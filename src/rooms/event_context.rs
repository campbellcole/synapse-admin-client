use matrix_sdk::ruma::{
    api::client::filter::RoomEventFilter,
    events::{AnyStateEvent, AnyTimelineEvent},
};

use crate::prelude::*;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, TypedBuilder)]
pub struct EventContextQuery<'a> {
    #[builder(default, setter(strip_option))]
    pub limit: Option<usize>,
    #[builder(default, setter(strip_option))]
    pub filter: Option<RoomEventFilter<'a>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EventContext {
    pub start: String,
    pub end: Option<String>,
    pub events_before: Vec<AnyTimelineEvent>,
    pub events_after: Vec<AnyTimelineEvent>,
    pub state: Vec<AnyStateEvent>,
}

impl SynapseClient {
    pub async fn get_room_event_context(
        &self,
        room_id: &RoomId,
        event_id: &EventId,
    ) -> Result<EventContext> {
        execute!(
            self.inner
                .get(endpoint!(self format!("/rooms/{room_id}/context/{event_id}")))
                .send()
                .await?
                .json::<MatrixResult<EventContext>>()
                .await?
        )
    }
}
