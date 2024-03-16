use crate::requests::*;
use crate::types::*;

/// Use this method to send text messages.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct CopyMessage {
    chat_id: ChatRef,
    from_chat_id: ChatRef,
    message_id: MessageId,
}

impl Request for CopyMessage {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<MessageIdRes>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("copyMessage"), self)
    }
}

impl CopyMessage {
    pub fn resend<C, T>(chat: C, msg: T) -> Self
    where
        C: ToChatRef,
        T: ToMessageId,
    {
        CopyMessage {
            chat_id: chat.to_chat_ref(),
            from_chat_id: chat.to_chat_ref(),
            message_id: msg.to_message_id(),
        }
    }
}
