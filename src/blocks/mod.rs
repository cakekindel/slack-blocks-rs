use serde::{Deserialize, Serialize};

use crate::impl_from_contents;

pub mod actions;
pub mod context;
pub mod image;
pub mod section;

type ValidationResult = Result<(), validator::ValidationErrors>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Block {
    #[serde(rename = "section")]
    Section(section::Contents),

    /// # Divider Block
    ///
    /// _[slack api docs 🔗][divider_docs]_
    ///
    /// A content divider, like an `<hr>`,
    /// to split up different blocks inside of a message.
    ///
    /// The divider block is nice and neat, requiring no fields.
    ///
    /// [divider_docs]: https://api.slack.com/reference/block-kit/blocks#divider
    #[serde(rename = "divider")]
    Divider,

    #[serde(rename = "image")]
    Image(image::Contents),

    #[serde(rename = "actions")]
    Actions(actions::Contents),

    #[serde(rename = "context")]
    Context(context::Contents),

    /// # Input Block
    ///
    /// _[slack api docs 🔗][input_docs]_
    ///
    /// A block that collects information from users -
    /// it can hold one of:
    ///   - [a plain-text input element 🔗][input_element]
    ///   - [a select menu element 🔗][select_element]
    ///   - [a multi-select menu element 🔗][multi_select_element]
    ///   - [a datepicker 🔗][datepicker_element]
    ///
    /// Read [slack's guide to using modals 🔗][modal_guide]
    /// to learn how input blocks pass information to your app.
    ///
    /// [input_docs]: https://api.slack.com/reference/block-kit/blocks#input
    /// [input_element]: https://api.slack.com/reference/block-kit/block-elements#input
    /// [select_element]: https://api.slack.com/reference/block-kit/block-elements#select
    /// [multi_select_element]: https://api.slack.com/reference/block-kit/block-elements#multi_select
    /// [datepicker_element]: https://api.slack.com/reference/block-kit/block-elements#datepicker
    /// [modal_guide]: https://api.slack.com/surfaces/modals/using#gathering_input
    #[serde(rename = "input")]
    Input {},

    /// # File Block
    ///
    /// _[slack api docs 🔗][file_docs]_
    ///
    /// Displays a [remote file 🔗][remote_file]
    ///
    /// [file_docs]: https://api.slack.com/reference/block-kit/blocks#file
    /// [remote_file]: https://api.slack.com/messaging/files/remote
    #[serde(rename = "file")]
    File {},
}

use std::fmt;

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = match self {
            Block::Section { .. } => "Section",
            Block::Divider => "Divider",
            Block::Image { .. } => "Image",
            Block::Actions { .. } => "Actions",
            Block::Context { .. } => "Context",
            Block::Input { .. } => "Input",
            Block::File { .. } => "File",
        };

        write!(f, "{}", kind)
    }
}

impl Block {
    pub fn validate(&self) -> ValidationResult {
        use Block::*;

        match self {
            Section(contents) => contents.validate(),
            Image(contents) => contents.validate(),
            Actions(contents) => contents.validate(),
            Context(contents) => contents.validate(),
            other => todo!("validation not implemented for {}", other),
        }
    }
}

impl_from_contents!(Block, Section, section::Contents);
impl_from_contents!(Block, Actions, actions::Contents);
impl_from_contents!(Block, Context, context::Contents);

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(
        r#"{
            "type": "section",
            "text": {
                "type": "mrkdwn",
                "text": "my message"
            }
        }"#
        => matches Block::Section(section::Contents::Text(_));
        "section_text"
    )]
    #[test_case(
        r#"{
            "type": "section",
            "fields": [{
                "type": "mrkdwn",
                "text": "my message"
            }]
        }"#
        => matches Block::Section (section::Contents::Fields(_));
        "section_fields"
    )]
    pub fn block_should_deserialize(json: &str) -> Block {
        // arrange

        // act
        serde_json::from_str::<Block>(&json).expect("Failed to serialize")

        // assert
    }
}
