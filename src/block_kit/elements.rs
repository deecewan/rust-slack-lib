use super::composition;

#[derive(Serialize)]
#[serde(untagged)]
pub enum SectionElements {
    Button(Button),
    Checkboxes(Checkboxes),
    Datepicker(Datepicker),
    Image(Image),
    MultiStaticSelect(MultiStaticSelect),
    MultiExternalSelect(MultiExternalSelect),
    MultiUsersSelect(MultiUsersSelect),
    MultiConversationsSelect(MultiConversationsSelect),
    MultiChannelsSelect(MultiChannelsSelect),
    Overflow(Overflow),
    PlainTextInput(PlainTextInput),
    RadioButtons(RadioButtons),
    StaticSelect(StaticSelect),
    ExternalSelect(ExternalSelect),
    UsersSelect(UsersSelect),
    ConversationsSelect(ConversationsSelect),
    ChannelsSelect(ChannelsSelect),
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ActionElements {
    Button(Button),
    Checkboxes(Checkboxes),
    Datepicker(Datepicker),
    Overflow(Overflow),
    PlainTextInput(PlainTextInput),
    RadioButtons(RadioButtons),
    StaticSelect(StaticSelect),
    ExternalSelect(ExternalSelect),
    UsersSelect(UsersSelect),
    ConversationsSelect(ConversationsSelect),
    ChannelsSelect(ChannelsSelect),
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum InputElements {
    Button(Button),
    Checkboxes(Checkboxes),
    Datepicker(Datepicker),
    MultiStaticSelect(MultiStaticSelect),
    MultiExternalSelect(MultiExternalSelect),
    MultiUsersSelect(MultiUsersSelect),
    MultiConversationsSelect(MultiConversationsSelect),
    MultiChannelsSelect(MultiChannelsSelect),
    PlainTextInput(PlainTextInput),
    RadioButtons(RadioButtons),
    StaticSelect(StaticSelect),
    ExternalSelect(ExternalSelect),
    UsersSelect(UsersSelect),
    ConversationsSelect(ConversationsSelect),
    ChannelsSelect(ChannelsSelect),
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ContextElements {
    Image(Image),
    Text(composition::Text),
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ButtonStyle {
    Primary,
    Danger,
    Default,
}

impl ButtonStyle {
    fn should_skip(item: &Option<ButtonStyle>) -> bool {
        match item {
            Some(style) => match style {
                ButtonStyle::Default => true,
                _ => false,
            },
            None => true,
        }
    }
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self::Default
    }
}

/// https://api.slack.com/reference/block-kit/block-elements#button
///
/// A regular interactive button
///
/// ```
/// # use assert_json_diff::assert_json_eq;
/// # use serde_json::json;
/// # use slack_lib::block_kit::elements::*;
/// let expected = json!({
///   "type": "button",
///   "text": {
///     "type": "plain_text",
///     "text": "Click Me"
///   },
///   "value": "click_me_123",
///   "action_id": "button"
/// });
///
/// let button = Button::builder()
///     .text("Click Me")
///     .value("click_me_123")
///     .action_id("button")
///     .build();
///
/// let output = serde_json::to_value(&button).unwrap();
///
/// assert_json_eq!(expected, output);
/// ```
///
/// A button with a `primary` `style` attribute:
///
/// ```
/// # use assert_json_diff::assert_json_eq;
/// # use serde_json::json;
/// # use slack_lib::block_kit::elements::*;
/// let expected = json!({
///   "type": "button",
///   "text": {
///     "type": "plain_text",
///     "text": "Save"
///   },
///   "style": "primary",
///   "value": "click_me_123",
///   "action_id": "button"
/// });
///
/// let button = Button::builder()
///     .text("Save")
///     .value("click_me_123")
///     .action_id("button")
///     .style(ButtonStyle::Primary)
///     .build();
///
/// let output = serde_json::to_value(&button).unwrap();
///
/// assert_json_eq!(expected, output);
/// ```
///
/// A link button:
///
/// ```
/// # use assert_json_diff::assert_json_eq;
/// # use serde_json::json;
/// # use slack_lib::block_kit::elements::*;
/// let expected = json!({
///   "type": "button",
///   "text": {
///     "type": "plain_text",
///     "text": "Link Button"
///   },
///   "action_id": "button",
///   "url": "https://api.slack.com/block-kit"
/// });
///
/// let button = Button::builder()
///     .text("Link Button")
///     .action_id("button")
///     .url("https://api.slack.com/block-kit")
///     .build();
///
/// let output = serde_json::to_value(&button).unwrap();
///
/// assert_json_eq!(expected, output);
/// ```
#[impl_for(SectionElements, ActionElements)]
#[derive(TypedBuilder, Serialize)]
pub struct Button {
    #[serde(rename = "type")]
    #[builder(setter(skip), default = String::from("button"))]
    block_type: String,

    #[builder(setter(into))]
    text: composition::PlainText,

    #[builder(setter(into))]
    action_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    value: Option<String>,

    #[serde(skip_serializing_if = "ButtonStyle::should_skip")]
    #[builder(setter(strip_option), default)]
    style: Option<ButtonStyle>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    confirm: Option<composition::Confirmation>,
}

/// https://api.slack.com/reference/block-kit/block-elements#checkboxes
///
/// ```
/// # use assert_json_diff::assert_json_eq;
/// # use serde_json::json;
/// # use slack_lib::block_kit::elements::*;
/// # use slack_lib::block_kit::composition::*;
/// let expected = json!({
///   "type": "checkboxes",
///   "action_id": "this_is_an_action_id",
///   "initial_options": [
///     {
///       "value": "A1",
///       "text": {
///         "type": "plain_text",
///         "text": "Checkbox 1"
///       }
///     }
///   ],
///   "options": [
///     {
///       "value": "A1",
///       "text": {
///         "type": "plain_text",
///         "text": "Checkbox 1"
///       }
///     },
///     {
///       "value": "A2",
///       "text": {
///         "type": "plain_text",
///         "text": "Checkbox 2"
///       }
///     }
///   ]
/// });
///
/// let checkboxes = Checkboxes::builder()
///     .action_id("this_is_an_action_id")
///     .options(vec![
///       PlainTextOptionItem::builder()
///         .text("Checkbox 1")
///         .value("A1")
///         .build()
///         .into(),
///       PlainTextOptionItem::builder()
///         .text("Checkbox 2")
///         .value("A2")
///         .build()
///         .into(),
///     ])
///     .initial_options(vec![
///       PlainTextOptionItem::builder()
///         .text("Checkbox 1")
///         .value("A1")
///         .build()
///         .into(), // `.into` is required when the vec can be multiple types
///         // ^ in this case, initial options can be PlainTextOptionItem or
///         // MarkdownOptionItem, so you need to `.into()` each element
///     ])
///     .build();
///
/// let output = serde_json::to_value(&checkboxes).unwrap();
///
/// assert_json_eq!(expected, output);
/// ```
#[impl_for(InputElements, SectionElements, ActionElements)]
#[derive(TypedBuilder, Serialize)]
pub struct Checkboxes {
    #[serde(rename = "type")]
    #[builder(setter(skip), default = String::from("checkboxes"))]
    block_type: String,

    #[builder(setter(into))]
    action_id: String,

    options: Vec<composition::OptionItem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    initial_options: Option<Vec<composition::OptionItem>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    confirm: Option<composition::Confirmation>,
}

/// https://api.slack.com/reference/block-kit/block-elements#datepicker
///
/// ```
/// # use assert_json_diff::assert_json_eq;
/// # use serde_json::json;
/// # use slack_lib::block_kit::elements::*;
/// # use slack_lib::block_kit::composition::*;
/// let expected = json!({
///   "type": "datepicker",
///   "action_id": "datepicker123",
///   "initial_date": "1990-04-28",
///   "placeholder": {
///     "type": "plain_text",
///     "text": "Select a date"
///   }
/// });
///
/// let datepicker = Datepicker::builder()
///     .action_id("datepicker123")
///     .initial_date("1990-04-28")
///     .placeholder("Select a date")
///     .build();
///
/// let output = serde_json::to_value(&datepicker).unwrap();
///
/// assert_json_eq!(expected, output);
/// ```
#[impl_for(InputElements, SectionElements, ActionElements)]
#[derive(TypedBuilder, Serialize)]
pub struct Datepicker {
    #[serde(rename = "type")]
    #[builder(setter(skip), default = String::from("datepicker"))]
    block_type: String,

    #[builder(setter(into))]
    action_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option, into), default)]
    placeholder: Option<composition::PlainText>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option, into), default)]
    initial_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    confirm: Option<composition::Confirmation>,
}

/// https://api.slack.com/reference/block-kit/block-elements#image
///
/// ```
/// # use assert_json_diff::assert_json_eq;
/// # use serde_json::json;
/// # use slack_lib::block_kit::elements::*;
/// # use slack_lib::block_kit::composition::*;
/// let expected = json!({
///   "type": "image",
///   "image_url": "http://placekitten.com/700/500",
///   "alt_text": "Multiple cute kittens"
/// });
///
/// let image = Image::builder()
///     .image_url("http://placekitten.com/700/500")
///     .alt_text("Multiple cute kittens")
///     .build();
///
/// let output = serde_json::to_value(&image).unwrap();
///
/// assert_json_eq!(expected, output);
/// ```
#[impl_for(SectionElements, ContextElements)]
#[derive(TypedBuilder, Serialize)]
pub struct Image {
    #[serde(rename = "type")]
    #[builder(setter(skip), default = String::from("image"))]
    block_type: String,

    #[builder(setter(into))]
    image_url: String,
    #[builder(setter(into))]
    alt_text: String,
}

/// https://api.slack.com/reference/block-kit/block-elements#static_multi_select
///
/// ```
/// # use assert_json_diff::assert_json_eq;
/// # use serde_json::json;
/// # use slack_lib::block_kit::elements::*;
/// # use slack_lib::block_kit::composition::*;
///
/// let expected = json!({
///   "action_id": "text1234",
///   "type": "multi_static_select",
///   "placeholder": {
///     "type": "plain_text",
///     "text": "Select items"
///   },
///   "options": [
///     {
///       "text": {
///         "type": "plain_text",
///         "text": "*this is plain_text text*"
///       },
///       "value": "value-0"
///     },
///     {
///       "text": {
///         "type": "plain_text",
///         "text": "*this is plain_text text*"
///       },
///       "value": "value-1"
///     },
///     {
///       "text": {
///         "type": "plain_text",
///         "text": "*this is plain_text text*"
///       },
///       "value": "value-2"
///     }
///   ]
/// });
///
/// let select = MultiStaticSelect::builder()
///   .action_id("text1234")
///   .placeholder("Select items")
///   .options(vec![
///       PlainTextOptionItem::builder()
///         .text("*this is plain_text text*")
///         .value("value-0")
///         .build(),
///       PlainTextOptionItem::builder()
///         .text("*this is plain_text text*")
///         .value("value-1")
///         .build(),
///       PlainTextOptionItem::builder()
///         .text("*this is plain_text text*")
///         .value("value-2")
///         .build(),
///   ])
///   .build();
///
/// let output = serde_json::to_value(&select).unwrap();
///
/// assert_json_eq!(expected, output);
/// ```
#[impl_for(SectionElements, InputElements)]
#[derive(TypedBuilder, Serialize)]
pub struct MultiStaticSelect {
    #[serde(rename = "type")]
    #[builder(setter(skip), default = String::from("multi_static_select"))]
    block_type: String,

    #[builder(setter(into))]
    placeholder: composition::PlainText,

    #[builder(setter(into))]
    action_id: String,

    // Technically, one of the following two is required, but I can't model that
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    options: Option<Vec<composition::PlainTextOptionItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    option_groups: Option<Vec<composition::OptionGroup>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    initial_options: Option<Vec<composition::PlainTextOptionItem>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    confirm: Option<composition::Confirmation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    max_selected_items: Option<i32>,
}

/// https://api.slack.com/reference/block-kit/block-elements#external_multi_select
///
/// ```
/// # use assert_json_diff::assert_json_eq;
/// # use serde_json::json;
/// # use slack_lib::block_kit::elements::*;
/// # use slack_lib::block_kit::composition::*;
///
/// let expected = json!({
///   "action_id": "text1234",
///   "type": "multi_external_select",
///   "placeholder": {
///     "type": "plain_text",
///     "text": "Select items"
///   },
///   "min_query_length": 3
/// });
///
/// let select = MultiExternalSelect::builder()
///   .action_id("text1234")
///   .placeholder("Select items")
///   .min_query_length(3)
///   .build();
///
/// let output = serde_json::to_value(&select).unwrap();
///
/// assert_json_eq!(expected, output);
/// ```
#[impl_for(SectionElements, InputElements)]
#[derive(TypedBuilder, Serialize)]
pub struct MultiExternalSelect {
    #[serde(rename = "type")]
    #[builder(setter(skip), default = String::from("multi_external_select"))]
    block_type: String,

    #[builder(setter(into))]
    placeholder: composition::PlainText,

    #[builder(setter(into))]
    action_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    min_query_length: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    initial_options: Option<Vec<composition::PlainTextOptionItem>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    confirm: Option<composition::Confirmation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    max_selected_items: Option<i32>,
}

/// https://api.slack.com/reference/block-kit/block-elements#users_multi_select
///
/// ```
/// # use assert_json_diff::assert_json_eq;
/// # use serde_json::json;
/// # use slack_lib::block_kit::elements::*;
/// # use slack_lib::block_kit::composition::*;
///
/// let expected = json!({
///   "action_id": "text1234",
///   "type": "multi_users_select",
///   "placeholder": {
///     "type": "plain_text",
///     "text": "Select users"
///   }
/// });
///
/// let select = MultiUsersSelect::builder()
///   .action_id("text1234")
///   .placeholder("Select users")
///   .build();
///
/// let output = serde_json::to_value(&select).unwrap();
///
/// assert_json_eq!(expected, output);
/// ```
#[impl_for(SectionElements, InputElements)]
#[derive(TypedBuilder, Serialize)]
pub struct MultiUsersSelect {
    #[serde(rename = "type")]
    #[builder(setter(skip), default = String::from("multi_users_select"))]
    block_type: String,

    #[builder(setter(into))]
    placeholder: composition::PlainText,

    #[builder(setter(into))]
    action_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    initial_users: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    confirm: Option<composition::Confirmation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    max_selected_items: Option<i32>,
}

/// https://api.slack.com/reference/block-kit/block-elements#conversation_multi_select
///
/// ```
/// # use assert_json_diff::assert_json_eq;
/// # use serde_json::json;
/// # use slack_lib::block_kit::elements::*;
/// # use slack_lib::block_kit::composition::*;
///
/// let expected = json!({
///   "action_id": "text1234",
///   "type": "multi_conversations_select",
///   "placeholder": {
///     "type": "plain_text",
///     "text": "Select conversations"
///   }
/// });
///
/// let select = MultiConversationsSelect::builder()
///   .action_id("text1234")
///   .placeholder("Select conversations")
///   .build();
///
/// let output = serde_json::to_value(&select).unwrap();
///
/// assert_json_eq!(expected, output);
/// ```
#[impl_for(SectionElements, InputElements)]
#[derive(TypedBuilder, Serialize)]
pub struct MultiConversationsSelect {
    #[serde(rename = "type")]
    #[builder(setter(skip), default = String::from("multi_conversations_select"))]
    block_type: String,

    #[builder(setter(into))]
    placeholder: composition::PlainText,

    #[builder(setter(into))]
    action_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    initial_conversations: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    default_to_current_conversation: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    confirm: Option<composition::Confirmation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    max_selected_items: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    filter: Option<composition::Filter>,
}

/// https://api.slack.com/reference/block-kit/block-elements#channel_multi_select
///
/// ```
/// # use assert_json_diff::assert_json_eq;
/// # use serde_json::json;
/// # use slack_lib::block_kit::elements::*;
/// # use slack_lib::block_kit::composition::*;
///
/// let expected = json!({
///   "action_id": "text1234",
///   "type": "multi_channels_select",
///   "placeholder": {
///     "type": "plain_text",
///     "text": "Select channels"
///   }
/// });
///
/// let select = MultiChannelsSelect::builder()
///   .action_id("text1234")
///   .placeholder("Select channels")
///   .build();
///
/// let output = serde_json::to_value(&select).unwrap();
///
/// assert_json_eq!(expected, output);
/// ```
#[impl_for(SectionElements, InputElements)]
#[derive(TypedBuilder, Serialize)]
pub struct MultiChannelsSelect {
    #[serde(rename = "type")]
    #[builder(setter(skip), default = String::from("multi_channels_select"))]
    block_type: String,

    #[builder(setter(into))]
    placeholder: composition::PlainText,

    #[builder(setter(into))]
    action_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    initial_channels: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    confirm: Option<composition::Confirmation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    max_selected_items: Option<i32>,
}

/// https://api.slack.com/reference/block-kit/block-elements#overflow
///
/// ```
/// # use assert_json_diff::assert_json_eq;
/// # use serde_json::json;
/// # use slack_lib::block_kit::elements::*;
/// # use slack_lib::block_kit::composition::*;
///
/// let expected = json!({
///   "type": "overflow",
///   "options": [
///     {
///       "text": {
///         "type": "plain_text",
///         "text": "*this is plain_text text*"
///       },
///       "value": "value-0"
///     },
///     {
///       "text": {
///         "type": "plain_text",
///         "text": "*this is plain_text text*"
///       },
///       "value": "value-1"
///     },
///     {
///       "text": {
///         "type": "plain_text",
///         "text": "*this is plain_text text*"
///       },
///       "value": "value-2"
///     },
///     {
///       "text": {
///         "type": "plain_text",
///         "text": "*this is plain_text text*"
///       },
///       "value": "value-3"
///     },
///     {
///       "text": {
///         "type": "plain_text",
///         "text": "*this is plain_text text*"
///       },
///       "value": "value-4"
///     }
///   ],
///   "action_id": "overflow"
/// });
///
/// let select = Overflow::builder()
///   .action_id("overflow")
///   .options(vec![
///       OverflowOptionItem::builder()
///         .text("*this is plain_text text*")
///         .value("value-0")
///         .build(),
///       OverflowOptionItem::builder()
///         .text("*this is plain_text text*")
///         .value("value-1")
///         .build(),
///       OverflowOptionItem::builder()
///         .text("*this is plain_text text*")
///         .value("value-2")
///         .build(),
///       OverflowOptionItem::builder()
///         .text("*this is plain_text text*")
///         .value("value-3")
///         .build(),
///       OverflowOptionItem::builder()
///         .text("*this is plain_text text*")
///         .value("value-4")
///         .build(),
///   ])
///   .build();
///
/// let output = serde_json::to_value(&select).unwrap();
///
/// assert_json_eq!(expected, output);
/// ```
#[impl_for(SectionElements, ActionElements)]
#[derive(TypedBuilder, Serialize)]
pub struct Overflow {
    #[serde(rename = "type")]
    #[builder(setter(skip), default = String::from("overflow"))]
    block_type: String,

    #[builder(setter(into))]
    action_id: String,

    options: Vec<composition::OverflowOptionItem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    confirm: Option<composition::Confirmation>,
}

/// https://api.slack.com/reference/block-kit/block-elements#input
///
/// ```
/// # use assert_json_diff::assert_json_eq;
/// # use serde_json::json;
/// # use slack_lib::block_kit::elements::*;
/// # use slack_lib::block_kit::composition::*;
///
/// let expected = json!({
///   "type": "plain_text_input",
///   "action_id": "plain_input",
///   "placeholder": {
///     "type": "plain_text",
///     "text": "Enter some plain text"
///   }
/// });
///
/// let input = PlainTextInput::builder()
///   .action_id("plain_input")
///   .placeholder("Enter some plain text")
///   .build();
///
/// let output = serde_json::to_value(&input).unwrap();
///
/// assert_json_eq!(expected, output);
/// ```
#[impl_for(SectionElements, ActionElements, InputElements)]
#[derive(TypedBuilder, Serialize)]
pub struct PlainTextInput {
    #[serde(rename = "type")]
    #[builder(setter(skip), default = String::from("plain_text_input"))]
    block_type: String,

    #[builder(setter(into))]
    action_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    placeholder: Option<composition::PlainText>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    initial_value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    multiline: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    min_length: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    max_length: Option<i32>,
}

/// https://api.slack.com/reference/block-kit/block-elements#radio
///
/// ```
/// # use assert_json_diff::assert_json_eq;
/// # use serde_json::json;
/// # use slack_lib::block_kit::elements::*;
/// # use slack_lib::block_kit::composition::*;
///
/// let expected = json!({
///   "type": "radio_buttons",
///   "action_id": "this_is_an_action_id",
///   "initial_option": {
///     "value": "A1",
///     "text": {
///       "type": "plain_text",
///       "text": "Radio 1"
///     }
///   },
///   "options": [
///     {
///       "value": "A1",
///       "text": {
///         "type": "plain_text",
///         "text": "Radio 1"
///       }
///     },
///     {
///       "value": "A2",
///       "text": {
///         "type": "plain_text",
///         "text": "Radio 2"
///       }
///     }
///   ]
/// });
///
/// let input = RadioButtons::builder()
///   .initial_option(
///     PlainTextOptionItem::builder()
///       .text("Radio 1")
///       .value("A1")
///       .build()
///       .into()
///   )
///   .options(vec![
///     PlainTextOptionItem::builder()
///       .text("Radio 1")
///       .value("A1")
///       .build()
///       .into(),
///     PlainTextOptionItem::builder()
///       .text("Radio 2")
///       .value("A2")
///       .build()
///       .into(),
///   ])
///   .action_id("this_is_an_action_id")
///   .build();
///
/// let output = serde_json::to_value(&input).unwrap();
///
/// assert_json_eq!(expected, output);
/// ```
#[impl_for(SectionElements, ActionElements, InputElements)]
#[derive(TypedBuilder, Serialize)]
pub struct RadioButtons {
    #[serde(rename = "type")]
    #[builder(setter(skip), default = String::from("radio_buttons"))]
    block_type: String,

    #[builder(setter(into))]
    action_id: String,

    options: Vec<composition::OptionItem>,

    #[builder(setter(strip_option))]
    initial_option: Option<composition::OptionItem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    confirm: Option<composition::Confirmation>,
}

/// https://api.slack.com/reference/block-kit/block-elements#static_select
///
/// ```
/// # use assert_json_diff::assert_json_eq;
/// # use serde_json::json;
/// # use slack_lib::block_kit::elements::*;
/// # use slack_lib::block_kit::composition::*;
///
/// let expected = json!({
///   "action_id": "text1234",
///   "type": "static_select",
///   "placeholder": {
///     "type": "plain_text",
///     "text": "Select an item"
///   },
///   "options": [
///     {
///       "text": {
///         "type": "plain_text",
///         "text": "*this is plain_text text*"
///       },
///       "value": "value-0"
///     },
///     {
///       "text": {
///         "type": "plain_text",
///         "text": "*this is plain_text text*"
///       },
///       "value": "value-1"
///     },
///     {
///       "text": {
///         "type": "plain_text",
///         "text": "*this is plain_text text*"
///       },
///       "value": "value-2"
///     }
///   ]
/// });
///
/// let select = StaticSelect::builder()
///   .action_id("text1234")
///   .placeholder("Select an item")
///   .options(vec![
///       PlainTextOptionItem::builder()
///         .text("*this is plain_text text*")
///         .value("value-0")
///         .build(),
///       PlainTextOptionItem::builder()
///         .text("*this is plain_text text*")
///         .value("value-1")
///         .build(),
///       PlainTextOptionItem::builder()
///         .text("*this is plain_text text*")
///         .value("value-2")
///         .build(),
///   ])
///   .build();
///
/// let output = serde_json::to_value(&select).unwrap();
///
/// assert_json_eq!(expected, output);
/// ```
#[impl_for(SectionElements, ActionElements, InputElements)]
#[derive(TypedBuilder, Serialize)]
pub struct StaticSelect {
    #[serde(rename = "type")]
    #[builder(setter(skip), default = String::from("static_select"))]
    block_type: String,

    #[builder(setter(into))]
    placeholder: composition::PlainText,

    #[builder(setter(into))]
    action_id: String,

    // Technically, one of the following two is required, but I can't model that
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    options: Option<Vec<composition::PlainTextOptionItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    option_groups: Option<Vec<composition::OptionGroup>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    initial_option: Option<composition::PlainTextOptionItem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    confirm: Option<composition::Confirmation>,
}

/// https://api.slack.com/reference/block-kit/block-elements#external_select
///
/// ```
/// # use assert_json_diff::assert_json_eq;
/// # use serde_json::json;
/// # use slack_lib::block_kit::elements::*;
/// # use slack_lib::block_kit::composition::*;
///
/// let expected = json!({
///   "action_id": "text1234",
///   "type": "external_select",
///   "placeholder": {
///     "type": "plain_text",
///     "text": "Select an item"
///   },
///   "min_query_length": 3
/// });
///
/// let select = ExternalSelect::builder()
///   .action_id("text1234")
///   .placeholder("Select an item")
///   .min_query_length(3)
///   .build();
///
/// let output = serde_json::to_value(&select).unwrap();
///
/// assert_json_eq!(expected, output);
/// ```
#[impl_for(SectionElements, ActionElements, InputElements)]
#[derive(TypedBuilder, Serialize)]
pub struct ExternalSelect {
    #[serde(rename = "type")]
    #[builder(setter(skip), default = String::from("external_select"))]
    block_type: String,

    #[builder(setter(into))]
    placeholder: composition::PlainText,

    #[builder(setter(into))]
    action_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    min_query_length: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    initial_option: Option<composition::PlainTextOptionItem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    confirm: Option<composition::Confirmation>,
}

/// https://api.slack.com/reference/block-kit/block-elements#users_select
///
/// ```
/// # use assert_json_diff::assert_json_eq;
/// # use serde_json::json;
/// # use slack_lib::block_kit::elements::*;
/// # use slack_lib::block_kit::composition::*;
///
/// let expected = json!({
///   "action_id": "text1234",
///   "type": "users_select",
///   "placeholder": {
///     "type": "plain_text",
///     "text": "Select an item"
///   }
/// });
///
/// let select = UsersSelect::builder()
///   .action_id("text1234")
///   .placeholder("Select an item")
///   .build();
///
/// let output = serde_json::to_value(&select).unwrap();
///
/// assert_json_eq!(expected, output);
/// ```
#[impl_for(SectionElements, ActionElements, InputElements)]
#[derive(TypedBuilder, Serialize)]
pub struct UsersSelect {
    #[serde(rename = "type")]
    #[builder(setter(skip), default = String::from("users_select"))]
    block_type: String,

    #[builder(setter(into))]
    placeholder: composition::PlainText,

    #[builder(setter(into))]
    action_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    initial_user: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    confirm: Option<composition::Confirmation>,
}

/// https://api.slack.com/reference/block-kit/block-elements#conversation_select
///
/// ```
/// # use assert_json_diff::assert_json_eq;
/// # use serde_json::json;
/// # use slack_lib::block_kit::elements::*;
/// # use slack_lib::block_kit::composition::*;
///
/// let expected = json!({
///   "action_id": "text1234",
///   "type": "conversations_select",
///   "placeholder": {
///     "type": "plain_text",
///     "text": "Select an item"
///   }
/// });
///
/// let select = ConversationsSelect::builder()
///   .action_id("text1234")
///   .placeholder("Select an item")
///   .build();
///
/// let output = serde_json::to_value(&select).unwrap();
///
/// assert_json_eq!(expected, output);
/// ```
#[impl_for(SectionElements, ActionElements, InputElements)]
#[derive(TypedBuilder, Serialize)]
pub struct ConversationsSelect {
    #[serde(rename = "type")]
    #[builder(setter(skip), default = String::from("conversations_select"))]
    block_type: String,

    #[builder(setter(into))]
    placeholder: composition::PlainText,

    #[builder(setter(into))]
    action_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    initial_conversation: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    default_to_current_conversation: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    confirm: Option<composition::Confirmation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    filter: Option<composition::Filter>,
    // TODO: response_url_enabled
}

/// https://api.slack.com/reference/block-kit/block-elements#channel_select
///
/// ```
/// # use assert_json_diff::assert_json_eq;
/// # use serde_json::json;
/// # use slack_lib::block_kit::elements::*;
/// # use slack_lib::block_kit::composition::*;
///
/// let expected = json!({
///   "action_id": "text1234",
///   "type": "channels_select",
///   "placeholder": {
///     "type": "plain_text",
///     "text": "Select an item"
///   }
/// });
///
/// let select = ChannelsSelect::builder()
///   .action_id("text1234")
///   .placeholder("Select an item")
///   .build();
///
/// let output = serde_json::to_value(&select).unwrap();
///
/// assert_json_eq!(expected, output);
/// ```
#[impl_for(SectionElements, ActionElements, InputElements)]
#[derive(TypedBuilder, Serialize)]
pub struct ChannelsSelect {
    #[serde(rename = "type")]
    #[builder(setter(skip), default = String::from("channels_select"))]
    block_type: String,

    #[builder(setter(into))]
    placeholder: composition::PlainText,

    #[builder(setter(into))]
    action_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    initial_channel: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    confirm: Option<composition::Confirmation>,
    // TODO: response_url_enabled
}
