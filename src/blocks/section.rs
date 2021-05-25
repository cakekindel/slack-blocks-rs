//! # Section Block
//!
//! _[slack api docs 🔗]_
//!
//! Available in surfaces:
//!  - [modals 🔗]
//!  - [messages 🔗]
//!  - [home tabs 🔗]
//!
//! A `section` is one of the most flexible blocks available -
//! it can be used as a simple text block,
//! in combination with text fields,
//! or side-by-side with any of the available [block elements 🔗]
//!
//! [slack api docs 🔗]: https://api.slack.com/reference/block-kit/blocks#section
//! [modals 🔗]: https://api.slack.com/surfaces/modals
//! [messages 🔗]: https://api.slack.com/surfaces/messages
//! [home tabs 🔗]: https://api.slack.com/surfaces/tabs
//! [block elements 🔗]: https://api.slack.com/reference/messaging/block-elements

use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{compose::text, elems::BlockElement, val_helpr::ValidationResult};

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
pub struct Contents<'a> {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[validate(custom = "validate::fields")]
  fields: Option<Cow<'a, [text::Text]>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[validate(custom = "validate::text")]
  text: Option<text::Text>,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[validate(custom = "validate::block_id")]
  block_id: Option<Cow<'a, str>>,

  /// One of the available [element objects 🔗][element_objects].
  ///
  /// [element_objects]: https://api.slack.com/reference/messaging/block-elements
  #[serde(skip_serializing_if = "Option::is_none")]
  accessory: Option<BlockElement<'a>>,
}

impl<'a> Contents<'a> {
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
  /// use slack_blocks::{blocks, compose::text};
  ///
  /// # use std::error::Error;
  /// # pub fn main() -> Result<(), Box<dyn Error>> {
  /// let fields = vec!["Left column", "Right column"].into_iter()
  ///                                                 .map(|s: &str| -> text::Text {
  ///                                                   text::Plain::from(s).into()
  ///                                                 })
  ///                                                 .collect::<Vec<_>>();
  ///
  /// let block = blocks::Section::from_fields(&fields);
  ///
  /// // < send to slack API >
  /// # Ok(())
  /// # }
  /// ```
  pub fn from_fields<I>(fields: I) -> Self
    where I: Into<Cow<'a, [text::Text]>>
  {
    let fields = Some(fields.into());

    Self { fields,
           text: None,
           block_id: None,
           accessory: None }
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
  /// use slack_blocks::{blocks, compose::text};
  ///
  /// let block =
  ///   blocks::section::Contents::from_text(text::Plain::from("I am a section!"));
  ///
  /// // < send to slack API >
  /// ```
  pub fn from_text(text: impl Into<text::Text>) -> Self {
    Self { text: Some(text.into()),
           fields: None,
           block_id: None,
           accessory: None }
  }

  /// Set a unique `block_id` to identify this instance of an Section Block.
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
  pub fn with_block_id(mut self, block_id: impl Into<Cow<'a, str>>) -> Self {
    self.block_id = Some(block_id.into());
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
  /// use slack_blocks::{blocks, compose::text};
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
  use super::*;
  use crate::{compose::text,
              val_helpr::{below_len, ValidatorResult}};

  pub(super) fn text(text: &text::Text) -> ValidatorResult {
    below_len("Section.text", 3000, text.as_ref())
  }

  pub(super) fn block_id(text: &Cow<str>) -> ValidatorResult {
    below_len("Section.block_id", 255, text.as_ref())
  }

  pub(super) fn fields(texts: &Cow<[text::Text]>) -> ValidatorResult {
    below_len("Section.fields", 10, texts.as_ref()).and(
                                                        texts.iter()
                                                             .map(|text| {
                                                               below_len(
             "Section.fields",
             2000,
             text.as_ref())
                                                             })
                                                             .collect(),
    )
  }
}
