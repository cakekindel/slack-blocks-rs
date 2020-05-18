use serde::{Deserialize, Serialize};

use crate::validation::ValidationResult;

mod section;
use section::SectionContents;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Block {
    /// # Section Block
    ///
    /// _[slack api docs 🔗](https://api.slack.com/reference/block-kit/blocks#section)_
    ///
    /// A `section` is one of the most flexible blocks available -
    /// it can be used as a simple text block,
    /// in combination with text fields,
    /// or side-by-side with any of the available [block elements 🔗](https://api.slack.com/reference/messaging/block-elements)
    #[serde(rename = "section")]
    Section(SectionContents),

    /// # Divider Block
    ///
    /// _[slack api docs 🔗](https://api.slack.com/reference/block-kit/blocks#divider)_
    ///
    /// A content divider, like an `<hr>`,
    /// to split up different blocks inside of a message.
    ///
    /// The divider block is nice and neat, requiring no fields.
    #[serde(rename = "divider")]
    Divider,

    /// # Image Block
    ///
    /// _[slack api docs 🔗](https://api.slack.com/reference/block-kit/blocks#image)_
    ///
    /// A simple image block, designed to make those cat photos really pop.
    #[serde(rename = "image")]
    Image {},

    /// # Actions Block
    ///
    /// _[slack api docs 🔗](https://api.slack.com/reference/block-kit/blocks#actions)_
    ///
    /// A block that is used to hold interactive [elements 🔗](https://api.slack.com/reference/messaging/block-elements)
    #[serde(rename = "actions")]
    Actions {},

    /// # Context Block
    ///
    /// _[slack api docs 🔗](https://api.slack.com/reference/block-kit/blocks#context)_
    ///
    /// Displays message context, which can include both images and text.
    #[serde(rename = "context")]
    Context {},

    /// # Input Block
    ///
    /// _[slack api docs 🔗](https://api.slack.com/reference/block-kit/blocks#input)_
    ///
    /// A block that collects information from users -
    /// it can hold one of:
    ///   - [a plain-text input element 🔗](https://api.slack.com/reference/block-kit/block-elements#input)
    ///   - [a select menu element 🔗](https://api.slack.com/reference/block-kit/block-elements#select)
    ///   - [a multi-select menu element 🔗](https://api.slack.com/reference/block-kit/block-elements#multi_select)
    ///   - [a datepicker 🔗](https://api.slack.com/reference/block-kit/block-elements#datepicker)
    ///
    /// Read [slack's guide to using modals 🔗](https://api.slack.com/surfaces/modals/using#gathering_input)
    /// to learn how input blocks pass information to your app.
    #[serde(rename = "input")]
    Input {},

    /// # File Block
    ///
    /// _[slack api docs 🔗](https://api.slack.com/reference/block-kit/blocks#file)_
    ///
    /// Displays a [remote file 🔗](https://api.slack.com/messaging/files/remote)
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
    use std::iter::repeat;
    use test_case::test_case;

    use super::*;
    use crate::compose::{Text};
    use crate::validation::{ValidationError, ValidationResult};

    //#[test_case(
    //    Block::Section { text: Text::markdown(string_of_len(3001)) }
    //        => matches Err(ValidationError::Text(TextValidationError::ExceedsMaxLen { .. }));
    //    "fail_when_section_text_longer_than_3k_chars"
    //)]
    pub fn block_validation_should(block: Block) -> ValidationResult {
        // arrange

        // act
        block.validate()

        // assert - handled by right hand of `=>` expression in test_case macro
    }

    #[test]
    pub fn section_should_deserialize() {
        // arrange
        let json = r#"{
            "type": "section",
            "text": {
                "type": "mrkdwn",
                "text": "my message"
            }
        }"#;

        // act
        let block = serde_json::from_str::<Block>(&json).expect("Failed to serialize");

        // assert
        assert!(matches!(block, Block::Section { .. }))
    }

    #[test]
    pub fn context_should_deserialize() {
        // arrange
        let json = r#"{ "type": "context" }"#;

        // act
        let block = serde_json::from_str::<Block>(&json).expect("Failed to serialize");

        // assert
        assert!(matches!(block, Block::Context { .. }))
    }

    #[test]
    pub fn divider_should_deserialize() {
        // arrange
        let json = r#"{ "type": "divider" }"#;

        // act
        let block = serde_json::from_str::<Block>(&json).expect("Failed to serialize");

        // assert
        assert!(matches!(block, Block::Divider { .. }))
    }

    #[test]
    pub fn image_should_deserialize() {
        // arrange
        let json = r#"{ "type": "image" }"#;

        // act
        let block = serde_json::from_str::<Block>(&json).expect("Failed to serialize");

        // assert
        assert!(matches!(block, Block::Image { .. }))
    }

    #[test]
    pub fn actions_should_deserialize() {
        // arrange
        let json = r#"{ "type": "actions" }"#;

        // act
        let block = serde_json::from_str::<Block>(&json).expect("Failed to serialize");

        // assert
        assert!(matches!(block, Block::Actions { .. }))
    }

    #[test]
    pub fn input_should_deserialize() {
        // arrange
        let json = r#"{ "type": "actions" }"#;

        // act
        let block = serde_json::from_str::<Block>(&json).expect("Failed to serialize");

        // assert
        assert!(matches!(block, Block::Actions { .. }))
    }

    #[test]
    pub fn file_should_deserialize() {
        // arrange
        let json = r#"{ "type": "file" }"#;

        // act
        let block = serde_json::from_str::<Block>(&json).expect("Failed to serialize");

        // assert
        assert!(matches!(block, Block::File { .. }))
    }
}
