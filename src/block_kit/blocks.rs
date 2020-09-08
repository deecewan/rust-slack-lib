use super::composition;
use super::elements;

#[allow(clippy::large_enum_variant)]
#[derive(Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum MessageBlock {
    Actions(Actions),
    Context(Context),
    Divider(Divider),
    Header(Header),
    Image(Image),
    Section(Section),
}

#[allow(clippy::large_enum_variant)]
#[derive(Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum ModalBlock {
    Actions(Actions),
    Context(Context),
    Divider(Divider),
    Header(Header),
    Image(Image),
    Input(Input),
    Section(Section),
}

#[allow(clippy::large_enum_variant)]
#[derive(Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum HomeTabBlock {
    Actions(Actions),
    Context(Context),
    Divider(Divider),
    Header(Header),
    Image(Image),
    Section(Section),
}

#[impl_for(MessageBlock, ModalBlock, HomeTabBlock)]
#[derive(TypedBuilder, Serialize)]
pub struct Actions {
    #[builder(setter(into, strip_option), default)]
    block_id: Option<String>,
    #[builder(setter(into))]
    elements: Vec<elements::ActionElements>,
}

#[impl_for(MessageBlock, ModalBlock, HomeTabBlock)]
#[derive(TypedBuilder, Serialize)]
pub struct Context {
    #[builder(setter(into, strip_option), default)]
    block_id: Option<String>,
    #[builder(setter(into))]
    elements: Vec<elements::ContextElements>,
}

#[impl_for(MessageBlock, ModalBlock, HomeTabBlock)]
#[derive(TypedBuilder, Serialize)]
pub struct Divider {
    #[builder(setter(into, strip_option), default)]
    block_id: Option<String>,
}

// TODO: Add `Files`, maybe. It can't actually be created by an end user (only
// added by Slack), so it's not helpful unless we deserialize.

#[impl_for(MessageBlock, ModalBlock, HomeTabBlock)]
#[derive(TypedBuilder, Serialize)]
pub struct Header {
    #[builder(setter(into, strip_option), default)]
    block_id: Option<String>,

    #[builder(setter(into))]
    text: composition::PlainText,
}

#[impl_for(MessageBlock, ModalBlock, HomeTabBlock)]
#[derive(TypedBuilder, Serialize)]
pub struct Image {
    #[builder(setter(into, strip_option), default)]
    block_id: Option<String>,

    #[builder(setter(into))]
    image_url: String,

    #[builder(setter(into))]
    alt_text: String,

    #[builder(setter(into, strip_option), default)]
    title: Option<composition::PlainText>,
}

#[impl_for(ModalBlock)]
#[derive(TypedBuilder, Serialize)]
pub struct Input {
    #[builder(setter(into, strip_option), default)]
    block_id: Option<String>,

    #[builder(setter(into))]
    label: composition::PlainText,

    #[builder(setter(into))]
    element: elements::InputElements,

    #[builder(setter(into, strip_option), default)]
    hint: Option<composition::PlainText>,

    #[builder(setter(into, strip_option), default)]
    optional: Option<bool>,
}

#[impl_for(MessageBlock, ModalBlock, HomeTabBlock)]
#[derive(TypedBuilder, Serialize)]
pub struct Section {
    #[builder(setter(into, strip_option), default)]
    block_id: Option<String>,

    #[builder(setter(into))]
    text: composition::Text,

    #[builder(setter(strip_option), default)]
    fields: Option<Vec<composition::Text>>,

    #[builder(setter(into, strip_option), default)]
    accessory: Option<elements::SectionElements>,
}
