use super::composition;

#[derive(Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
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
#[serde(rename_all = "snake_case", tag = "type")]
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
#[serde(rename_all = "snake_case", tag = "type")]
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
#[serde(rename_all = "snake_case", tag = "type")]
pub enum ContextElements {
    Image(Image),
    Text(composition::Text),
}

#[derive(Serialize)]
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

#[impl_for(SectionElements, ActionElements)]
#[derive(TypedBuilder, Serialize)]
pub struct Button {
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

#[impl_for(InputElements, SectionElements, ActionElements)]
#[derive(TypedBuilder, Serialize)]
pub struct Checkboxes {
    #[builder(setter(into))]
    action_id: String,

    options: Vec<composition::MarkdownOptionItem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    initial_options: Option<Vec<composition::MarkdownOptionItem>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    confirm: Option<composition::Confirmation>,
}

#[impl_for(InputElements, SectionElements, ActionElements)]
#[derive(TypedBuilder, Serialize)]
pub struct Datepicker {
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

#[impl_for(SectionElements, ContextElements)]
#[derive(TypedBuilder, Serialize)]
pub struct Image {
    #[builder(setter(into))]
    image_url: String,
    #[builder(setter(into))]
    alt_text: String,
}

#[impl_for(SectionElements, InputElements)]
#[derive(TypedBuilder, Serialize)]
pub struct MultiStaticSelect {
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

#[impl_for(SectionElements, InputElements)]
#[derive(TypedBuilder, Serialize)]
pub struct MultiExternalSelect {
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

#[impl_for(SectionElements, InputElements)]
#[derive(TypedBuilder, Serialize)]
pub struct MultiUsersSelect {
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

#[impl_for(SectionElements, InputElements)]
#[derive(TypedBuilder, Serialize)]
pub struct MultiConversationsSelect {
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

#[impl_for(SectionElements, InputElements)]
#[derive(TypedBuilder, Serialize)]
pub struct MultiChannelsSelect {
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

#[impl_for(SectionElements, ActionElements)]
#[derive(TypedBuilder, Serialize)]
pub struct Overflow {
    #[builder(setter(into))]
    action_id: String,

    options: Vec<composition::OverflowOptionItem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    confirm: Option<composition::Confirmation>,
}

#[impl_for(SectionElements, ActionElements, InputElements)]
#[derive(TypedBuilder, Serialize)]
pub struct PlainTextInput {
    #[builder(setter(into))]
    action_id: String,

    #[builder(setter(into, strip_option))]
    placeholder: Option<composition::PlainText>,

    #[builder(setter(into, strip_option))]
    initial_value: Option<String>,

    #[builder(setter(strip_option))]
    multiline: Option<bool>,

    #[builder(setter(strip_option))]
    min_length: Option<i32>,

    #[builder(setter(strip_option))]
    max_length: Option<i32>,
}

#[impl_for(SectionElements, ActionElements, InputElements)]
#[derive(TypedBuilder, Serialize)]
pub struct RadioButtons {
    #[builder(setter(into))]
    action_id: String,

    options: Vec<composition::MarkdownOptionItem>,

    #[builder(setter(strip_option))]
    initial_option: Option<composition::MarkdownOptionItem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(strip_option), default)]
    confirm: Option<composition::Confirmation>,
}

#[impl_for(SectionElements, ActionElements, InputElements)]
#[derive(TypedBuilder, Serialize)]
pub struct StaticSelect {
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

#[impl_for(SectionElements, ActionElements, InputElements)]
#[derive(TypedBuilder, Serialize)]
pub struct ExternalSelect {
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

#[impl_for(SectionElements, ActionElements, InputElements)]
#[derive(TypedBuilder, Serialize)]
pub struct UsersSelect {
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

#[impl_for(SectionElements, ActionElements, InputElements)]
#[derive(TypedBuilder, Serialize)]
pub struct ConversationsSelect {
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

#[impl_for(SectionElements, ActionElements, InputElements)]
#[derive(TypedBuilder, Serialize)]
pub struct ChannelsSelect {
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
