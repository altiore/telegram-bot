use std::ops::Not;

use crate::requests::*;
use crate::types::*;

/// Use this method to forward messages of any kind.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct ForwardMessage {
    chat_id: ChatRef,
    from_chat_id: ChatRef,
    #[serde(skip_serializing_if = "Not::not")]
    disable_notification: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    message_id: MessageId,
}

impl Request for ForwardMessage {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<Message>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("forwardMessage"), self)
    }
}

impl ForwardMessage {
    pub fn new<M, F, T>(message: M, from: F, to: T) -> Self
    where
        M: ToMessageId,
        F: ToChatRef,
        T: ToChatRef,
    {
        ForwardMessage {
            chat_id: to.to_chat_ref(),
            from_chat_id: from.to_chat_ref(),
            disable_notification: false,
            protect_content: None,
            message_id: message.to_message_id(),
        }
    }

    pub fn resend<M, C>(message: M, chat: C) -> Self
    where
        M: ToMessageId,
        C: ToChatRef,
    {
        ForwardMessage {
            chat_id: chat.to_chat_ref(),
            from_chat_id: chat.to_chat_ref(),
            disable_notification: false,
            protect_content: Some(true),
            message_id: message.to_message_id(),
        }
    }

    pub fn disable_notification(&mut self) -> &mut Self {
        self.disable_notification = true;
        self
    }

    pub fn protect_content(&mut self) -> &mut Self {
        self.protect_content = Some(true);
        self
    }
}

/// Forward message.
pub trait CanForwardMessage {
    fn forward<T>(&self, to: T) -> ForwardMessage
    where
        T: ToChatRef;
}

impl<M> CanForwardMessage for M
where
    M: ToMessageId + ToSourceChat,
{
    fn forward<T>(&self, to: T) -> ForwardMessage
    where
        T: ToChatRef,
    {
        ForwardMessage::new(self.to_message_id(), self.to_source_chat(), to)
    }
}
