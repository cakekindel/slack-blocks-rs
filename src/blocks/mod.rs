use serde::{Deserialize, Serialize};

pub mod section;
pub mod image;

type ValidationResult = Result<(), validator::ValidationErrors>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Block {
    /// # Section Block
    ///
    /// _[slack api docs 🔗][section_docs]_
    ///
    /// Available in surfaces:
    ///  - [modals 🔗][modal_surface]
    ///  - [messages 🔗][message_surface]
    ///  - [home tabs 🔗][tab_surface]
    ///
    /// A `section` is one of the most flexible blocks available -
    /// it can be used as a simple text block,
    /// in combination with text fields,
    /// or side-by-side with any of the available [block elements 🔗][block_elements]
    ///
    /// [section_docs]: https://api.slack.com/reference/block-kit/blocks#section
    /// [modal_surface]: https://api.slack.com/surfaces/modals
    /// [message_surface]: https://api.slack.com/surfaces/messages
    /// [tab_surface]: https://api.slack.com/surfaces/tabs
    /// [block_elements]: https://api.slack.com/reference/messaging/block-elements
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

    /// # Image Block
    ///
    /// _[slack api docs 🔗][image_docs]_
    ///
    /// A simple image block, designed to make those cat photos really pop.
    ///
    /// [image_docs]: https://api.slack.com/reference/block-kit/blocks#image
    #[serde(rename = "image")]
    Image(image::Contents),

    /// # Actions Block
    ///
    /// _[slack api docs 🔗][action_docs]_
    ///
    /// A block that is used to hold interactive [elements 🔗][block_elements]
    ///
    /// [block_elements]: https://api.slack.com/reference/messaging/block-elements
    /// [section_docs]: https://api.slack.com/reference/block-kit/blocks#actions
    #[serde(rename = "actions")]
    Actions {},

    /// # Context Block
    ///
    /// _[slack api docs 🔗][context_docs]_
    ///
    /// Displays message context, which can include both images and text.
    ///
    /// [context_docs]: https://api.slack.com/reference/block-kit/blocks#context
    #[serde(rename = "context")]
    Context {},

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
            _ => todo!(),
        }
    }
}

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
        => matches Block::Section { .. };
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
        => matches Block::Section { .. };
        "section_fields"
    )]
    #[test_case(
        r#"{ "type": "context" }"#
        => matches Block::Context { .. };
        "context"
    )]
    #[test_case(
        r#"{ "type": "divider" }"#
        => matches Block::Divider;
        "divider"
    )]
    #[test_case(
        r#"{ "type": "image" }"#
        => matches Block::Image { .. };
        "image"
    )]
    #[test_case(
        r#"{ "type": "actions" }"#
        => matches Block::Actions { .. };
        "actions"
    )]
    #[test_case(
        r#"{ "type": "input" }"#
        => matches Block::Input { .. };
        "input"
    )]
    #[test_case(
        r#"{ "type": "file" }"#
        => matches Block::File { .. };
        "file"
    )]
    pub fn block_should_deserialize(json: &str) -> Block {
        // arrange

        // act
        serde_json::from_str::<Block>(&json).expect("Failed to serialize")

        // assert
    }
}
