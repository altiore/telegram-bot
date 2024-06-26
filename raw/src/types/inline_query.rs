use crate::types::*;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct InlineQuery {
    pub id: InlineQueryId,
    pub from: User,
    pub chat_type: String,
    pub query: String,
    pub offset: String,
    pub location: Option<Location>,
}

impl Into<InlineQueryId> for InlineQuery {
    fn into(self) -> InlineQueryId {
        self.id
    }
}
