#[derive(Serialize)]
#[serde(untagged)]
pub enum Text {
    PlainText(PlainText),
    Markdown(Markdown),
}

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

impl Default for ConfirmationStyle {
    fn default() -> Self {
        Self::Primary
    }
}

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
    #[builder(setter(strip_option))]
    style: Option<ConfirmationStyle>,
}

// Used in select and multi-select
#[derive(TypedBuilder, Serialize)]
pub struct PlainTextOptionItem {
    #[builder(setter(into))]
    text: PlainText,
    #[builder(setter(into))]
    value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option))]
    description: Option<PlainText>,
}

// Used in Radio buttons and Checkboxes
#[derive(TypedBuilder, Serialize)]
pub struct MarkdownOptionItem {
    #[builder(setter(into))]
    text: Markdown,
    #[builder(setter(into))]
    value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option))]
    description: Option<PlainText>,
}

// Overflow options have this extra `url` option, otherwise they're the same as
// the PlainTextOptionItem
#[derive(TypedBuilder, Serialize)]
pub struct OverflowOptionItem {
    #[builder(setter(into))]
    text: PlainText,
    #[builder(setter(into))]
    value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option))]
    description: Option<PlainText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option))]
    url: Option<String>,
}

// These are type-accurate assuming that Option Groups continue to only be
// allowed in selects and multi-selects.
#[derive(TypedBuilder, Serialize)]
pub struct OptionGroup {
    #[builder(setter(into))]
    label: PlainText,
    options: Vec<PlainTextOptionItem>,
}

#[derive(Serialize)]
pub enum FilterInclusions {
    IM,
    MPIM,
    Private,
    Public,
}

// These are all technically optional, but at least one of the 3 must be set. I
// don't have a way to model this in the type system currently
#[derive(TypedBuilder, Serialize)]
pub struct Filter {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option))]
    include: Option<Vec<FilterInclusions>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option))]
    exclude_external_shared_channels: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option))]
    exclude_bot_users: Option<bool>,
}
