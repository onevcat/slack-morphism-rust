use rsb_derive::Builder;
use rvstruct::ValueStruct;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::common::*;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, ValueStruct)]
pub struct SlackBlockId(pub String);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackBlock {
    #[serde(rename = "section")]
    Section(SlackSectionBlock),
    #[serde(rename = "divider")]
    Divider(SlackDividerBlock),
    #[serde(rename = "image")]
    Image(SlackImageBlock),
    #[serde(rename = "actions")]
    Actions(SlackActionsBlock),
    #[serde(rename = "context")]
    Context(SlackContextBlock),
    #[serde(rename = "input")]
    Input(SlackInputBlock),
    #[serde(rename = "file")]
    File(SlackFileBlock),

    // This block is still undocumented, so we don't define any structure yet we can return it back,
    #[serde(rename = "rich_text")]
    RichText(serde_json::Value),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct SlackAttachment {
    pub color: String,
    pub blocks: Option<Vec<SlackBlock>>
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackSectionBlock {
    pub block_id: Option<SlackBlockId>,
    pub text: Option<SlackBlockText>,
    pub fields: Option<Vec<SlackBlockText>>,
    pub accessory: Option<SlackSectionBlockElement>,
}

impl From<SlackSectionBlock> for SlackBlock {
    fn from(block: SlackSectionBlock) -> Self {
        SlackBlock::Section(block)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackDividerBlock {
    pub block_id: Option<SlackBlockId>,
}

impl From<SlackDividerBlock> for SlackBlock {
    fn from(block: SlackDividerBlock) -> Self {
        SlackBlock::Divider(block)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackImageBlock {
    pub block_id: Option<SlackBlockId>,
    pub image_url: String,
    pub alt_text: String,
    pub title: Option<SlackBlockPlainText>,
}

impl From<SlackImageBlock> for SlackBlock {
    fn from(block: SlackImageBlock) -> Self {
        SlackBlock::Image(block)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackActionsBlock {
    pub block_id: Option<SlackBlockId>,
    pub elements: Vec<SlackActionBlockElement>,
}

impl From<SlackActionsBlock> for SlackBlock {
    fn from(block: SlackActionsBlock) -> Self {
        SlackBlock::Actions(block)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackContextBlock {
    pub block_id: Option<SlackBlockId>,
    pub elements: Vec<SlackContextBlockElement>,
}

impl From<SlackContextBlock> for SlackBlock {
    fn from(block: SlackContextBlock) -> Self {
        SlackBlock::Context(block)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackInputBlock {
    pub block_id: Option<SlackBlockId>,
    pub label: SlackBlockPlainText,
    pub element: SlackInputBlockElement,
    pub hint: Option<SlackBlockPlainText>,
    pub optional: Option<bool>,
}

impl From<SlackInputBlock> for SlackBlock {
    fn from(block: SlackInputBlock) -> Self {
        SlackBlock::Input(block)
    }
}

const SLACK_FILE_BLOCK_SOURCE_DEFAULT: &str = "remote";

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackFileBlock {
    pub block_id: Option<SlackBlockId>,
    pub external_id: String,
    #[default = "SLACK_FILE_BLOCK_SOURCE_DEFAULT.into()"]
    pub source: String,
}

impl From<SlackFileBlock> for SlackBlock {
    fn from(block: SlackFileBlock) -> Self {
        SlackBlock::File(block)
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackSectionBlockElement {
    #[serde(rename = "image")]
    Image(SlackBlockImageElement),
    #[serde(rename = "button")]
    Button(SlackBlockButtonElement),
    #[serde(rename = "static_select")]
    StaticSelect(SlackBlockStaticSelectElement),
    #[serde(rename = "multi_static_select")]
    MultiStaticSelect(SlackBlockMultiStaticSelectElement),
    #[serde(rename = "external_select")]
    ExternalSelect(SlackBlockExternalSelectElement),
    #[serde(rename = "multi_external_select")]
    MultiExternalSelect(SlackBlockMultiExternalSelectElement),
    #[serde(rename = "users_select")]
    UsersSelect(SlackBlockUsersSelectElement),
    #[serde(rename = "multi_users_select")]
    MultiUsersSelect(SlackBlockMultiUsersSelectElement),
    #[serde(rename = "conversations_select")]
    ConversationsSelect(SlackBlockConversationsSelectElement),
    #[serde(rename = "multi_conversations_select")]
    MultiConversationsSelect(SlackBlockMultiConversationsSelectElement),
    #[serde(rename = "channels_select")]
    ChannelsSelect(SlackBlockChannelsSelectElement),
    #[serde(rename = "multi_channels_select")]
    MultiChannelsSelect(SlackBlockMultiChannelsSelectElement),
    #[serde(rename = "overflow")]
    Overflow(SlackBlockOverflowElement),
    #[serde(rename = "datepicker")]
    DatePicker(SlackBlockDatePickerElement),
    #[serde(rename = "plain_text_input")]
    PlainTextInput(SlackBlockPlainTextInputElement),
    #[serde(rename = "radio_buttons")]
    RadioButtons(SlackBlockRadioButtonsElement),
    #[serde(rename = "checkboxes")]
    Checkboxes(SlackBlockCheckboxesElement),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackActionBlockElement {
    #[serde(rename = "button")]
    Button(SlackBlockButtonElement),
    #[serde(rename = "overflow")]
    Overflow(SlackBlockOverflowElement),
    #[serde(rename = "datepicker")]
    DatePicker(SlackBlockDatePickerElement),
    #[serde(rename = "plain_text_input")]
    PlainTextInput(SlackBlockPlainTextInputElement),
    #[serde(rename = "radio_buttons")]
    RadioButtons(SlackBlockRadioButtonsElement),
    #[serde(rename = "checkboxes")]
    Checkboxes(SlackBlockCheckboxesElement),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackContextBlockElement {
    #[serde(rename = "image")]
    Image(SlackBlockImageElement),
    #[serde(rename = "plain_text")]
    Plain(SlackBlockPlainText),
    #[serde(rename = "mrkdwn")]
    MarkDown(SlackBlockMarkDownText),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackInputBlockElement {
    #[serde(rename = "static_select")]
    StaticSelect(SlackBlockStaticSelectElement),
    #[serde(rename = "multi_static_select")]
    MultiStaticSelect(SlackBlockMultiStaticSelectElement),
    #[serde(rename = "external_select")]
    ExternalSelect(SlackBlockExternalSelectElement),
    #[serde(rename = "multi_external_select")]
    MultiExternalSelect(SlackBlockMultiExternalSelectElement),
    #[serde(rename = "users_select")]
    UsersSelect(SlackBlockUsersSelectElement),
    #[serde(rename = "multi_users_select")]
    MultiUsersSelect(SlackBlockMultiUsersSelectElement),
    #[serde(rename = "conversations_select")]
    ConversationsSelect(SlackBlockConversationsSelectElement),
    #[serde(rename = "multi_conversations_select")]
    MultiConversationsSelect(SlackBlockMultiConversationsSelectElement),
    #[serde(rename = "channels_select")]
    ChannelsSelect(SlackBlockChannelsSelectElement),
    #[serde(rename = "multi_channels_select")]
    MultiChannelsSelect(SlackBlockMultiChannelsSelectElement),
    #[serde(rename = "datepicker")]
    DatePicker(SlackBlockDatePickerElement),
    #[serde(rename = "plain_text_input")]
    PlainTextInput(SlackBlockPlainTextInputElement),
    #[serde(rename = "radio_buttons")]
    RadioButtons(SlackBlockRadioButtonsElement),
    #[serde(rename = "checkboxes")]
    Checkboxes(SlackBlockCheckboxesElement),
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockImageElement {
    pub image_url: String,
    pub alt_text: String,
}

impl From<SlackBlockImageElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockImageElement) -> Self {
        SlackSectionBlockElement::Image(element)
    }
}

impl From<SlackBlockImageElement> for SlackContextBlockElement {
    fn from(element: SlackBlockImageElement) -> Self {
        SlackContextBlockElement::Image(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockButtonElement {
    pub action_id: SlackActionId,
    pub text: SlackBlockPlainTextOnly,
    pub url: Option<String>,
    pub value: Option<String>,
    pub style: Option<String>,
    pub confirm: Option<SlackBlockConfirmItem>,
}

impl From<SlackBlockButtonElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockButtonElement) -> Self {
        SlackSectionBlockElement::Button(element)
    }
}

impl From<SlackBlockButtonElement> for SlackActionBlockElement {
    fn from(element: SlackBlockButtonElement) -> Self {
        SlackActionBlockElement::Button(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockConfirmItem {
    pub title: SlackBlockPlainTextOnly,
    pub text: SlackBlockText,
    pub confirm: SlackBlockPlainTextOnly,
    pub deny: SlackBlockPlainTextOnly,
    pub style: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockChoiceItem<T: Into<SlackBlockText>> {
    pub text: T,
    pub value: String,
    pub url: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockOptionGroup<T: Into<SlackBlockText>> {
    pub label: SlackBlockPlainTextOnly,
    pub options: Vec<SlackBlockChoiceItem<T>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockStaticSelectElement {
    pub action_id: SlackActionId,
    pub placeholder: SlackBlockPlainTextOnly,
    pub options: Option<Vec<SlackBlockChoiceItem<SlackBlockPlainTextOnly>>>,
    pub option_groups: Option<Vec<SlackBlockOptionGroup<SlackBlockPlainTextOnly>>>,
    pub initial_option: Option<SlackBlockChoiceItem<SlackBlockPlainTextOnly>>,
    pub confirm: Option<SlackBlockConfirmItem>,
}

impl From<SlackBlockStaticSelectElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockStaticSelectElement) -> Self {
        SlackSectionBlockElement::StaticSelect(element)
    }
}

impl From<SlackBlockStaticSelectElement> for SlackInputBlockElement {
    fn from(element: SlackBlockStaticSelectElement) -> Self {
        SlackInputBlockElement::StaticSelect(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockMultiStaticSelectElement {
    pub action_id: SlackActionId,
    pub placeholder: SlackBlockPlainTextOnly,
    pub options: Option<Vec<SlackBlockChoiceItem<SlackBlockPlainTextOnly>>>,
    pub option_groups: Option<Vec<SlackBlockOptionGroup<SlackBlockPlainTextOnly>>>,
    pub initial_options: Option<Vec<SlackBlockChoiceItem<SlackBlockPlainTextOnly>>>,
    pub confirm: Option<SlackBlockConfirmItem>,
    pub max_selected_items: Option<u64>,
}

impl From<SlackBlockMultiStaticSelectElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockMultiStaticSelectElement) -> Self {
        SlackSectionBlockElement::MultiStaticSelect(element)
    }
}

impl From<SlackBlockMultiStaticSelectElement> for SlackInputBlockElement {
    fn from(element: SlackBlockMultiStaticSelectElement) -> Self {
        SlackInputBlockElement::MultiStaticSelect(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockExternalSelectElement {
    pub action_id: SlackActionId,
    pub placeholder: SlackBlockPlainTextOnly,
    pub initial_option: Option<SlackBlockChoiceItem<SlackBlockPlainTextOnly>>,
    pub confirm: Option<SlackBlockConfirmItem>,
}

impl From<SlackBlockExternalSelectElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockExternalSelectElement) -> Self {
        SlackSectionBlockElement::ExternalSelect(element)
    }
}

impl From<SlackBlockExternalSelectElement> for SlackInputBlockElement {
    fn from(element: SlackBlockExternalSelectElement) -> Self {
        SlackInputBlockElement::ExternalSelect(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockMultiExternalSelectElement {
    pub action_id: SlackActionId,
    pub placeholder: SlackBlockPlainTextOnly,
    pub initial_options: Option<Vec<SlackBlockChoiceItem<SlackBlockPlainTextOnly>>>,
    pub confirm: Option<SlackBlockConfirmItem>,
    pub max_selected_items: Option<u64>,
}

impl From<SlackBlockMultiExternalSelectElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockMultiExternalSelectElement) -> Self {
        SlackSectionBlockElement::MultiExternalSelect(element)
    }
}

impl From<SlackBlockMultiExternalSelectElement> for SlackInputBlockElement {
    fn from(element: SlackBlockMultiExternalSelectElement) -> Self {
        SlackInputBlockElement::MultiExternalSelect(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockUsersSelectElement {
    pub action_id: SlackActionId,
    pub placeholder: SlackBlockPlainTextOnly,
    pub initial_user: Option<String>,
    pub confirm: Option<SlackBlockConfirmItem>,
}

impl From<SlackBlockUsersSelectElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockUsersSelectElement) -> Self {
        SlackSectionBlockElement::UsersSelect(element)
    }
}

impl From<SlackBlockUsersSelectElement> for SlackInputBlockElement {
    fn from(element: SlackBlockUsersSelectElement) -> Self {
        SlackInputBlockElement::UsersSelect(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockMultiUsersSelectElement {
    pub action_id: SlackActionId,
    pub placeholder: SlackBlockPlainTextOnly,
    pub initial_users: Option<Vec<String>>,
    pub confirm: Option<SlackBlockConfirmItem>,
    pub max_selected_items: Option<u64>,
}

impl From<SlackBlockMultiUsersSelectElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockMultiUsersSelectElement) -> Self {
        SlackSectionBlockElement::MultiUsersSelect(element)
    }
}

impl From<SlackBlockMultiUsersSelectElement> for SlackInputBlockElement {
    fn from(element: SlackBlockMultiUsersSelectElement) -> Self {
        SlackInputBlockElement::MultiUsersSelect(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockConversationsSelectElement {
    pub action_id: SlackActionId,
    pub placeholder: SlackBlockPlainTextOnly,
    pub initial_conversation: Option<SlackConversationId>,
    pub default_to_current_conversation: Option<bool>,
    pub confirm: Option<SlackBlockConfirmItem>,
    pub response_url_enabled: Option<bool>,
}

impl From<SlackBlockConversationsSelectElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockConversationsSelectElement) -> Self {
        SlackSectionBlockElement::ConversationsSelect(element)
    }
}

impl From<SlackBlockConversationsSelectElement> for SlackInputBlockElement {
    fn from(element: SlackBlockConversationsSelectElement) -> Self {
        SlackInputBlockElement::ConversationsSelect(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockMultiConversationsSelectElement {
    pub action_id: SlackActionId,
    pub placeholder: SlackBlockPlainTextOnly,
    pub initial_conversations: Option<Vec<SlackConversationId>>,
    pub default_to_current_conversation: Option<bool>,
    pub confirm: Option<SlackBlockConfirmItem>,
    pub max_selected_items: Option<u64>,
}

impl From<SlackBlockMultiConversationsSelectElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockMultiConversationsSelectElement) -> Self {
        SlackSectionBlockElement::MultiConversationsSelect(element)
    }
}

impl From<SlackBlockMultiConversationsSelectElement> for SlackInputBlockElement {
    fn from(element: SlackBlockMultiConversationsSelectElement) -> Self {
        SlackInputBlockElement::MultiConversationsSelect(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockChannelsSelectElement {
    pub action_id: SlackActionId,
    pub placeholder: SlackBlockPlainTextOnly,
    pub initial_channel: Option<SlackChannelId>,
    pub confirm: Option<SlackBlockConfirmItem>,
    pub response_url_enabled: Option<bool>,
}

impl From<SlackBlockChannelsSelectElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockChannelsSelectElement) -> Self {
        SlackSectionBlockElement::ChannelsSelect(element)
    }
}

impl From<SlackBlockChannelsSelectElement> for SlackInputBlockElement {
    fn from(element: SlackBlockChannelsSelectElement) -> Self {
        SlackInputBlockElement::ChannelsSelect(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockMultiChannelsSelectElement {
    pub action_id: SlackActionId,
    pub placeholder: SlackBlockPlainTextOnly,
    pub initial_channels: Option<Vec<SlackChannelId>>,
    pub confirm: Option<SlackBlockConfirmItem>,
    pub max_selected_items: Option<u64>,
}

impl From<SlackBlockMultiChannelsSelectElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockMultiChannelsSelectElement) -> Self {
        SlackSectionBlockElement::MultiChannelsSelect(element)
    }
}

impl From<SlackBlockMultiChannelsSelectElement> for SlackInputBlockElement {
    fn from(element: SlackBlockMultiChannelsSelectElement) -> Self {
        SlackInputBlockElement::MultiChannelsSelect(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockOverflowElement {
    pub action_id: SlackActionId,
    pub options: Vec<SlackBlockChoiceItem<SlackBlockPlainTextOnly>>,
    pub confirm: Option<SlackBlockConfirmItem>,
}

impl From<SlackBlockOverflowElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockOverflowElement) -> Self {
        SlackSectionBlockElement::Overflow(element)
    }
}

impl From<SlackBlockOverflowElement> for SlackActionBlockElement {
    fn from(element: SlackBlockOverflowElement) -> Self {
        SlackActionBlockElement::Overflow(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockDatePickerElement {
    pub action_id: SlackActionId,
    pub placeholder: SlackBlockPlainTextOnly,
    pub initial_date: Option<String>,
    pub confirm: Option<SlackBlockConfirmItem>,
}

impl From<SlackBlockDatePickerElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockDatePickerElement) -> Self {
        SlackSectionBlockElement::DatePicker(element)
    }
}

impl From<SlackBlockDatePickerElement> for SlackInputBlockElement {
    fn from(element: SlackBlockDatePickerElement) -> Self {
        SlackInputBlockElement::DatePicker(element)
    }
}

impl From<SlackBlockDatePickerElement> for SlackActionBlockElement {
    fn from(element: SlackBlockDatePickerElement) -> Self {
        SlackActionBlockElement::DatePicker(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockPlainTextInputElement {
    pub action_id: SlackActionId,
    pub placeholder: SlackBlockPlainTextOnly,
    pub initial_value: Option<String>,
    pub multiline: Option<bool>,
    pub min_length: Option<u64>,
    pub max_length: Option<u64>,
}

impl From<SlackBlockPlainTextInputElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockPlainTextInputElement) -> Self {
        SlackSectionBlockElement::PlainTextInput(element)
    }
}

impl From<SlackBlockPlainTextInputElement> for SlackInputBlockElement {
    fn from(element: SlackBlockPlainTextInputElement) -> Self {
        SlackInputBlockElement::PlainTextInput(element)
    }
}

impl From<SlackBlockPlainTextInputElement> for SlackActionBlockElement {
    fn from(element: SlackBlockPlainTextInputElement) -> Self {
        SlackActionBlockElement::PlainTextInput(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockRadioButtonsElement {
    pub action_id: SlackActionId,
    pub options: Vec<SlackBlockChoiceItem<SlackBlockText>>,
    pub initial_option: Option<SlackBlockChoiceItem<SlackBlockText>>,
    pub confirm: Option<SlackBlockConfirmItem>,
}

impl From<SlackBlockRadioButtonsElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockRadioButtonsElement) -> Self {
        SlackSectionBlockElement::RadioButtons(element)
    }
}

impl From<SlackBlockRadioButtonsElement> for SlackInputBlockElement {
    fn from(element: SlackBlockRadioButtonsElement) -> Self {
        SlackInputBlockElement::RadioButtons(element)
    }
}

impl From<SlackBlockRadioButtonsElement> for SlackActionBlockElement {
    fn from(element: SlackBlockRadioButtonsElement) -> Self {
        SlackActionBlockElement::RadioButtons(element)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockCheckboxesElement {
    pub action_id: SlackActionId,
    pub options: Vec<SlackBlockChoiceItem<SlackBlockText>>,
    pub initial_options: Option<Vec<SlackBlockChoiceItem<SlackBlockText>>>,
    pub confirm: Option<SlackBlockConfirmItem>,
}

impl From<SlackBlockCheckboxesElement> for SlackSectionBlockElement {
    fn from(element: SlackBlockCheckboxesElement) -> Self {
        SlackSectionBlockElement::Checkboxes(element)
    }
}

impl From<SlackBlockCheckboxesElement> for SlackInputBlockElement {
    fn from(element: SlackBlockCheckboxesElement) -> Self {
        SlackInputBlockElement::Checkboxes(element)
    }
}

impl From<SlackBlockCheckboxesElement> for SlackActionBlockElement {
    fn from(element: SlackBlockCheckboxesElement) -> Self {
        SlackActionBlockElement::Checkboxes(element)
    }
}

/**
* 'plain_text' type of https://api.slack.com/reference/block-kit/composition-objects#text
*/
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockPlainText {
    pub text: String,
    pub emoji: Option<bool>,
}

/**
 * 'mrkdwn' type of https://api.slack.com/reference/block-kit/composition-objects#text
 */
#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackBlockMarkDownText {
    pub text: String,
    pub verbatim: Option<bool>,
}

/**
 * https://api.slack.com/reference/block-kit/composition-objects#text
 */
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackBlockText {
    #[serde(rename = "plain_text")]
    Plain(SlackBlockPlainText),
    #[serde(rename = "mrkdwn")]
    MarkDown(SlackBlockMarkDownText),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "plain_text")]
pub struct SlackBlockPlainTextOnly {
    #[serde(flatten)]
    value: SlackBlockPlainText,
}

impl SlackBlockPlainText {
    pub fn as_block_text(&self) -> SlackBlockText {
        SlackBlockText::Plain(self.clone())
    }
}

impl From<String> for SlackBlockPlainText {
    fn from(value: String) -> Self {
        SlackBlockPlainText::new(value)
    }
}

impl From<&str> for SlackBlockPlainText {
    fn from(value: &str) -> Self {
        SlackBlockPlainText::new(String::from(value))
    }
}

impl SlackBlockMarkDownText {
    pub fn as_block_text(&self) -> SlackBlockText {
        SlackBlockText::MarkDown(self.clone())
    }
}

impl From<String> for SlackBlockMarkDownText {
    fn from(value: String) -> Self {
        SlackBlockMarkDownText::new(value)
    }
}

impl From<&str> for SlackBlockMarkDownText {
    fn from(value: &str) -> Self {
        SlackBlockMarkDownText::new(String::from(value))
    }
}

impl From<SlackBlockPlainText> for SlackBlockPlainTextOnly {
    fn from(pt: SlackBlockPlainText) -> Self {
        SlackBlockPlainTextOnly { value: pt }
    }
}

impl From<SlackBlockPlainText> for SlackBlockText {
    fn from(text: SlackBlockPlainText) -> Self {
        SlackBlockText::Plain(text)
    }
}

impl From<SlackBlockMarkDownText> for SlackBlockText {
    fn from(text: SlackBlockMarkDownText) -> Self {
        SlackBlockText::MarkDown(text)
    }
}

impl From<SlackBlockPlainText> for SlackContextBlockElement {
    fn from(text: SlackBlockPlainText) -> Self {
        SlackContextBlockElement::Plain(text)
    }
}

impl From<SlackBlockMarkDownText> for SlackContextBlockElement {
    fn from(text: SlackBlockMarkDownText) -> Self {
        SlackContextBlockElement::MarkDown(text)
    }
}

impl From<SlackBlockPlainTextOnly> for SlackBlockText {
    fn from(text: SlackBlockPlainTextOnly) -> Self {
        SlackBlockText::Plain(text.value)
    }
}

impl From<String> for SlackBlockPlainTextOnly {
    fn from(value: String) -> Self {
        SlackBlockPlainTextOnly {
            value: value.into(),
        }
    }
}

impl From<&str> for SlackBlockPlainTextOnly {
    fn from(value: &str) -> Self {
        SlackBlockPlainTextOnly {
            value: value.into(),
        }
    }
}
