use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::compose::text;
use crate::val_helpr::ValidationResult;

/// # Section Block
///
/// _[slack api docs 🔗]_
///
/// Available in surfaces:
///  - [modals 🔗]
///  - [messages 🔗]
///  - [home tabs 🔗]
///
/// A `section` is one of the most flexible blocks available -
/// it can be used as a simple text block,
/// in combination with text fields,
/// or side-by-side with any of the available [block elements 🔗]
///
/// [slack api docs 🔗]: https://api.slack.com/reference/block-kit/blocks#section
/// [modals 🔗]: https://api.slack.com/surfaces/modals
/// [messages 🔗]: https://api.slack.com/surfaces/messages
/// [home tabs 🔗]: https://api.slack.com/surfaces/tabs
/// [block elements 🔗]: https://api.slack.com/reference/messaging/block-elements
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize, Validate)]
pub struct Contents {
    #[validate(length(max = 10))]
    #[validate(custom = "validate::fields")]
    fields: Option<Vec<text::Text>>,


    #[validate(custom = "validate::text")]
    text: Option<text::Text>,

    #[validate(length(max = 255))]
    block_id: Option<String>,

    /// One of the available [element objects 🔗][element_objects].
    ///
    /// [element_objects]: https://api.slack.com/reference/messaging/block-elements
    accessory: Option<()>,
}

impl Contents {
    /// Construct a Section block from a collection of text objects
    ///
    /// # Arguments
    /// - `fields` - A collection of [text objects 🔗].
    ///     Any text objects included with fields will be
    ///     rendered in a compact format that allows for
    ///     2 columns of side-by-side text.
    ///     Maximum number of items is 10.
    ///     Maximum length for the text in each item is 2000 characters.
    ///
    /// [text objects 🔗]: https://api.slack.com/reference/messaging/composition-objects#text
    ///
    /// # Errors
    /// Doesn't error. To validate your model against the length requirements,
    /// use the `validate` method.
    ///
    /// # Example
    /// ```
    /// use slack_blocks::blocks;
    /// use slack_blocks::compose::text;
    ///
    /// # use std::error::Error;
    /// # pub fn main() -> Result<(), Box<dyn Error>> {
    /// let fields = vec![
    ///     text::Plain::from("Left column"),
    ///     text::Plain::from("Right column"),
    /// ];
    ///
    /// let block = blocks::section
    ///     ::Contents
    ///     ::from_fields(fields);
    ///
    /// // < send to slack API >
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_fields<FieldIter: IntoIterator<Item = impl Into<text::Text>>>(fields: FieldIter) -> Self {
        let fields = Some(fields.into_iter().map(|f| f.into()).collect());

        Self {
            fields,
            text: None,
            block_id: None,
            accessory: None,
        }
    }

    /// Construct a Section block from a text object
    ///
    /// # Arguments
    /// - `text` - The text for the block, in the form of a [text object 🔗].
    ///     Maximum length for the text in this field is 3000 characters.
    ///
    /// [text object 🔗]: https://api.slack.com/reference/messaging/composition-objects#text
    ///
    /// # Errors
    /// Doesn't error. To validate your model against the length requirements,
    /// use the `validate` method.
    ///
    /// # Example
    /// ```
    /// use slack_blocks::blocks;
    /// use slack_blocks::compose::text;
    ///
    /// let block = blocks::section
    ///     ::Contents
    ///     ::from_text(text::Plain::from("I am a section!"));
    ///
    /// // < send to slack API >
    /// ```
    pub fn from_text(text: impl Into<text::Text>) -> Self {
        Self {
            text: Some(text.into()),
            fields: None,
            block_id: None,
            accessory: None,
        }
    }

    /// Set a unique `block_id` to identify this instance of an File Block.
    ///
    /// # Arguments
    ///
    /// - `block_id` - A string acting as a unique identifier for a block.
    ///     You can use this `block_id` when you receive an interaction
    ///     payload to [identify the source of the action 🔗].
    ///     If not specified, one will be generated.
    ///     Maximum length for this field is 255 characters.
    ///     `block_id` should be unique for each message and each iteration of a message.
    ///     If a message is updated, use a new `block_id`.
    ///
    /// [identify the source of the action 🔗]: https://api.slack.com/interactivity/handling#payloads
    ///
    /// # example
    /// ```
    /// use slack_blocks::blocks;
    ///
    /// # fn upload_file_to_slack(s: &str) -> String { String::new() }
    /// # use std::error::Error;
    /// # pub fn main() -> Result<(), Box<dyn Error>> {
    /// let file_id = upload_file_to_slack("https://www.cheese.com/cheese-wheel.png");
    ///
    /// let block = blocks::file::Contents::from_external_id(file_id)
    ///     .with_block_id("my_file_in_a_block_1234");
    ///
    /// // < send to slack API >
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_block_id(mut self, block_id: impl AsRef<str>) -> Self {
        self.block_id = Some(block_id.as_ref().to_string());
        self
    }

    /// Validate that this Section block agrees with Slack's model requirements
    ///
    /// # Errors
    /// - If `from_fields` was called with more than 10 fields,
    ///     or one of the fields contains text longer than
    ///     2000 chars
    /// - If `from_fields` was called with one of the fields
    ///     containing text longer than 2000 chars
    /// - If `from_text` was called with text longer than
    ///     3000 chars
    /// - If `with_block_id` was called with a block id longer
    ///     than 255 chars
    ///
    /// # Example
    /// ```
    /// use slack_blocks::blocks;
    /// use slack_blocks::compose::text;
    ///
    /// let long_string = std::iter::repeat(' ').take(256).collect::<String>();
    ///
    /// let block = blocks::section
    ///     ::Contents
    ///     ::from_text(text::Plain::from("file_id"))
    ///     .with_block_id(long_string);
    ///
    /// assert_eq!(true, matches!(block.validate(), Err(_)));
    ///
    /// // < send to slack API >
    /// ```
    pub fn validate(&self) -> ValidationResult {
        Validate::validate(self)
    }
}

mod validate {
    use crate::compose::text;
    use crate::val_helpr::{ValidatorResult, below_len};

    pub fn text(text: &text::Text) -> ValidatorResult {
        below_len("Section Text", 3000, text.as_ref())
    }

    pub fn fields(texts: &Vec<text::Text>) -> ValidatorResult {
        texts
            .iter()
            .map(|text| {
                below_len("Section Field", 2000, text.as_ref())
            })
            .collect()
    }
}
