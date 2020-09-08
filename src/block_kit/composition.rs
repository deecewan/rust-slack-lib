#[derive(Serialize)]
#[serde(untagged)]
pub enum Text {
    PlainText(PlainText),
    Markdown(Markdown),
}

/// [https://api.slack.com/reference/block-kit/composition-objects#text](https://api.slack.com/reference/block-kit/composition-objects#text)
///
/// ```
/// # use assert_json_diff::assert_json_eq;
/// # use serde_json::json;
/// # use slack_lib::block_kit::composition::*;
/// let expected = json!({
///   "type": "plain_text",
///   "text": "This is some plain text"
/// });
///
/// let text = PlainText::builder().text("This is some plain text").build();
/// let output = serde_json::to_value(&text).unwrap();
/// assert_json_eq!(&expected, output);
///
/// // Also implements From<&str>
/// let text: PlainText = "This is some plain text".into();
/// let output = serde_json::to_value(&text).unwrap();
/// assert_json_eq!(&expected, output);
/// ```
#[impl_for(Text)]
#[derive(TypedBuilder, Serialize)]
pub struct PlainText {
    #[serde(rename = "type")]
    #[builder(setter(skip), default = String::from("plain_text"))]
    block_type: String,
    #[builder(setter(into))]
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default = Option::None)]
    emoji: Option<bool>,
}

impl From<&str> for PlainText {
    fn from(item: &str) -> Self {
        Self::builder().text(item).build()
    }
}

/// [https://api.slack.com/reference/block-kit/composition-objects#text](https://api.slack.com/reference/block-kit/composition-objects#text)
///
/// ```
/// # use assert_json_diff::assert_json_eq;
/// # use serde_json::json;
/// # use slack_lib::block_kit::composition::*;
/// let expected = json!({
///   "type": "mrkdwn",
///   "text": "*Some* _formatted_ ~text~"
/// });
///
/// let text = Markdown::builder().text("*Some* _formatted_ ~text~").build();
/// let output = serde_json::to_value(&text).unwrap();
/// assert_json_eq!(&expected, output);
///
/// // Also implements From<&str>
/// let text: Markdown = "*Some* _formatted_ ~text~".into();
/// let output = serde_json::to_value(&text).unwrap();
/// assert_json_eq!(&expected, output);
/// ```
#[impl_for(Text)]
#[derive(TypedBuilder, Serialize)]
pub struct Markdown {
    #[serde(rename = "type")]
    #[builder(setter(skip), default = String::from("mrkdwn"))]
    block_type: String,
    #[builder(setter(into))]
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default = Option::None)]
    verbatim: Option<bool>,
}

impl From<&str> for Markdown {
    fn from(item: &str) -> Self {
        Self::builder().text(item).build()
    }
}

#[derive(Serialize)]
pub enum ConfirmationStyle {
    Primary,
    Danger,
}

/// [https://api.slack.com/reference/block-kit/composition-objects#confirm](https://api.slack.com/reference/block-kit/composition-objects#confirm)
///
/// ```
/// # use assert_json_diff::assert_json_eq;
/// # use serde_json::json;
/// # use slack_lib::block_kit::composition::*;
/// let confirm = Confirmation::builder()
///   // this is a `PlainText`, but we got `impl From<&str>`
///   .title("Are you sure?")
///   .text(Markdown::builder().text("Wouldn't you prefer a good game of _chess_?").build())
///   .confirm("Do it")
///   .deny("Stop, I've changed my mind!")
///   // if you need to, you can specify a style:
///   // .style(ConfirmationStyle::Primary)
///   .build();
///   let expected = json!({
///     "title": {
///         "type": "plain_text",
///         "text": "Are you sure?"
///     },
///     "text": {
///         "type": "mrkdwn",
///         "text": "Wouldn't you prefer a good game of _chess_?"
///     },
///     "confirm": {
///         "type": "plain_text",
///         "text": "Do it"
///     },
///     "deny": {
///         "type": "plain_text",
///         "text": "Stop, I've changed my mind!"
///     }
///   });
/// let output = serde_json::to_value(&confirm).unwrap();
/// assert_json_eq!(output, expected);
/// ```
#[derive(TypedBuilder, Serialize)]
pub struct Confirmation {
    #[builder(setter(into))]
    title: PlainText,
    #[builder(setter(into))]
    text: Text,
    #[builder(setter(into))]
    confirm: PlainText,
    #[builder(setter(into))]
    deny: PlainText,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    style: Option<ConfirmationStyle>,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum OptionItem {
    PlainTextOptionItem(PlainTextOptionItem),
    MarkdownOptionItem(MarkdownOptionItem),
}

/// [https://api.slack.com/reference/block-kit/composition-objects#option](https://api.slack.com/reference/block-kit/composition-objects#option)
///
/// ```
/// # use assert_json_diff::assert_json_eq;
/// # use serde_json::json;
/// # use slack_lib::block_kit::composition::*;
/// let expected = json!({
///   "text": {
///       "type": "plain_text",
///       "text": "Maru"
///   },
///   "value": "maru"
/// });
///
/// let option = PlainTextOptionItem::builder()
///     .text("Maru")
///     .value("maru")
///     .build();
///
/// let output = serde_json::to_value(&option).unwrap();
/// assert_json_eq!(expected, output);
/// ```
// Used in select and multi-select
#[impl_for(OptionItem)]
#[derive(TypedBuilder, Serialize)]
pub struct PlainTextOptionItem {
    #[builder(setter(into))]
    text: PlainText,
    #[builder(setter(into))]
    value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    description: Option<PlainText>,
}

/// [https://api.slack.com/reference/block-kit/composition-objects#option](https://api.slack.com/reference/block-kit/composition-objects#option)
///
/// For use in Radio buttons and Checkboxes
///
/// ```
/// # use assert_json_diff::assert_json_eq;
/// # use serde_json::json;
/// # use slack_lib::block_kit::composition::*;
/// let expected = json!({
///   "text": {
///       "type": "mrkdwn",
///       "text": "Maru"
///   },
///   "value": "maru"
/// });
///
/// let option = MarkdownOptionItem::builder()
///     .text("Maru")
///     .value("maru")
///     .build();
///
/// let output = serde_json::to_value(&option).unwrap();
/// assert_json_eq!(expected, output);
/// ```
#[impl_for(OptionItem)]
#[derive(TypedBuilder, Serialize)]
pub struct MarkdownOptionItem {
    #[builder(setter(into))]
    text: Markdown,
    #[builder(setter(into))]
    value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    description: Option<PlainText>,
}

/// [https://api.slack.com/reference/block-kit/composition-objects#option](https://api.slack.com/reference/block-kit/composition-objects#option)
///
/// Overflow options have this extra `url` option, otherwise they're the same as
/// the PlainTextOptionItem
///
/// ```
/// # use assert_json_diff::assert_json_eq;
/// # use serde_json::json;
/// # use slack_lib::block_kit::composition::*;
/// let expected = json!({
///   "text": {
///       "type": "plain_text",
///       "text": "Maru"
///   },
///   "value": "maru",
///   "url": "https://example.com"
/// });
///
/// let option = OverflowOptionItem::builder()
///     .text("Maru")
///     .value("maru")
///     .url("https://example.com")
///     .build();
///
/// let output = serde_json::to_value(&option).unwrap();
/// assert_json_eq!(expected, output);
/// ```
#[derive(TypedBuilder, Serialize)]
pub struct OverflowOptionItem {
    #[builder(setter(into))]
    text: PlainText,
    #[builder(setter(into))]
    value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    description: Option<PlainText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    url: Option<String>,
}

/// [https://api.slack.com/reference/block-kit/composition-objects#option_group](https://api.slack.com/reference/block-kit/composition-objects#option_group)
///
/// These only work for select + multi select, so they only support plaintext
/// `options`.
///
/// ```
/// # use assert_json_diff::assert_json_eq;
/// # use serde_json::json;
/// # use slack_lib::block_kit::composition::*;
/// // This one's pretty long :grimacing:
/// let expected = json!([
///   {
///     "label": {
///       "type": "plain_text",
///       "text": "Group 1"
///     },
///     "options": [
///       {
///         "text": {
///             "type": "plain_text",
///             "text": "*this is plain_text text*"
///         },
///         "value": "value-0"
///       },
///       {
///         "text": {
///             "type": "plain_text",
///             "text": "*this is plain_text text*"
///         },
///         "value": "value-1"
///       },
///       {
///         "text": {
///             "type": "plain_text",
///             "text": "*this is plain_text text*"
///         },
///         "value": "value-2"
///       }
///     ]
///   },
///   {
///     "label": {
///         "type": "plain_text",
///         "text": "Group 2"
///     },
///     "options": [
///       {
///         "text": {
///             "type": "plain_text",
///             "text": "*this is plain_text text*"
///         },
///         "value": "value-3"
///       }
///     ]
///   }
/// ]);
///
/// let option = vec![
///   OptionGroup::builder()
///     .label("Group 1")
///     .options(vec![
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
///     ])
///     .build(),
///   OptionGroup::builder()
///     .label("Group 2")
///     .options(vec![
///       PlainTextOptionItem::builder()
///         .text("*this is plain_text text*")
///         .value("value-3")
///         .build(),
///     ])
///     .build(),
/// ];
///
/// let output = serde_json::to_value(&option).unwrap();
/// assert_json_eq!(expected, output);
/// ```
#[derive(TypedBuilder, Serialize)]
pub struct OptionGroup {
    #[builder(setter(into))]
    label: PlainText,
    options: Vec<PlainTextOptionItem>,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FilterInclusions {
    IM,
    MPIM,
    Private,
    Public,
}

/// [https://api.slack.com/reference/block-kit/composition-objects#filter_conversations](https://api.slack.com/reference/block-kit/composition-objects#filter_conversations)
///
/// These are all technically optional, but at least one of the 3 must be set. I
/// don't have a way to model this in the type system currently
///
/// ```
/// # use assert_json_diff::assert_json_eq;
/// # use serde_json::json;
/// # use slack_lib::block_kit::composition::*;
/// let expected = json!({
///   "include": ["public", "mpim"],
///   "exclude_bot_users": true
/// });
///
/// let filter = Filter::builder()
///     .include(vec![FilterInclusions::Public, FilterInclusions::MPIM])
///     .exclude_bot_users(true)
///     .build();
///
/// let output = serde_json::to_value(&filter).unwrap();
/// assert_json_eq!(expected, output);
/// ```
#[derive(TypedBuilder, Serialize)]
pub struct Filter {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    include: Option<Vec<FilterInclusions>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    exclude_external_shared_channels: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    exclude_bot_users: Option<bool>,
}
