use crate::blocks::*;
use crate::common::*;
use crate::events::SlackMessageEventType;
use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

mod templates;

pub use templates::*;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackMessageOrigin {
    pub ts: SlackTs,
    pub channel: Option<SlackChannelId>,
    pub channel_type: Option<SlackChannelType>,
    pub thread_ts: Option<SlackTs>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackMessageContent {
    pub text: Option<String>,
    pub blocks: Option<Vec<SlackBlock>>,
    pub attachments: Option<Vec<SlackAttachment>>
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackMessageSender {
    pub user: Option<SlackUserId>,
    pub bot_id: Option<SlackBotId>,
    pub username: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackMessage {
    #[serde(flatten)]
    pub origin: SlackMessageOrigin,
    #[serde(flatten)]
    pub content: SlackMessageContent,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackHistoryMessage {
    #[serde(flatten)]
    pub origin: SlackMessageOrigin,
    #[serde(flatten)]
    pub content: SlackMessageContent,
    #[serde(flatten)]
    pub sender: SlackMessageSender,
    pub subtype: Option<SlackMessageEventType>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum SlackMessageResponseType {
    #[serde(rename = "in_channel")]
    InChannel,
    #[serde(rename = "ephemeral")]
    Ephemeral,
}
