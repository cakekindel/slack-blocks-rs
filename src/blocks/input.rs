//! # Input Block
//!
//! [slack api docs 🔗]
//!
//! A block that collects information from users -
//!
//! Read [slack's guide to using modals 🔗]
//! to learn how input blocks pass information to your app.
//!
//! [slack api docs 🔗]: https://api.slack.com/reference/block-kit/blocks#input
//! [slack's guide to using modals 🔗]: https://api.slack.com/surfaces/modals/using#gathering_input

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{compose::text,
            convert,
            elems::{select, Radio, TextInput},
            val_helpr::ValidationResult};

/// # Input Block
///
/// [slack api docs 🔗]
///
/// A block that collects information from users -
///
/// Read [slack's guide to using modals 🔗]
/// to learn how input blocks pass information to your app.
///
/// [slack api docs 🔗]: https://api.slack.com/reference/block-kit/blocks#input
/// [slack's guide to using modals 🔗]: https://api.slack.com/surfaces/modals/using#gathering_input
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize, Validate)]
pub struct Contents<'a> {
  #[validate(custom = "validate::label")]
  label: text::Text,

  element: InputElement<'a>,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[validate(length(max = 255))]
  block_id: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[validate(custom = "validate::hint")]
  hint: Option<text::Text>,

  #[serde(skip_serializing_if = "Option::is_none")]
  dispatch_action: Option<bool>,

  #[serde(skip_serializing_if = "Option::is_none")]
  optional: Option<bool>,
}

impl<'a> Contents<'a> {
  /// Create an Input Block from a text Label and interactive element.
  ///
  /// # Arguments
  ///
  /// - `label` - A label that appears above an input element in the form of
  ///     a [text object 🔗] that must have type of `plain_text`.
  ///     Maximum length for the text in this field is 2000 characters.
  ///
  /// - `element` - An interactive `block_element` that will be used to gather
  ///     the input for this block.
  ///     For the kinds of Elements supported by
  ///     Input blocks, see the `InputElement` enum.
  ///     For info about Block Elements in general,
  ///     see the `elems` module.
  ///
  /// [text object 🔗]: https://api.slack.com/reference/messaging/composition-objects#text
  ///
  /// # Example
  /// ```
  /// use slack_blocks::elems::Select;
  /// use slack_blocks::blocks;
  ///
  /// let label = "On a scale from 1 - 5, how angsty are you?";
  /// let input = Select::from_placeholder_and_action_id("Pick a channel...", "ABC123")
  ///     .choose_from_public_channels();
  ///
  /// let block = blocks::input::Contents::from_label_and_element(label, input);
  ///
  /// // < send to slack API >
  /// ```
  pub fn from_label_and_element(label: impl Into<text::Plain>,
                                element: impl Into<InputElement<'a>>)
                                -> Self {
    Contents { label: label.into().into(),
               element: element.into(),
               block_id: None,
               hint: None,
               dispatch_action: None,
               optional: None }
  }

  /// Set a unique `block_id` to identify this instance of an Input Block.
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
  /// # Example
  /// ```
  /// use slack_blocks::elems::Select;
  /// use slack_blocks::blocks;
  ///
  /// let label = "On a scale from 1 - 5, how angsty are you?";
  /// let input = Select::from_placeholder_and_action_id("Pick a channel...", "ABC123")
  ///     .choose_from_public_channels();
  ///
  /// let block = blocks::input
  ///     ::Contents
  ///     ::from_label_and_element(label, input)
  ///     .with_block_id("angst_rating_12345");
  ///
  /// // < send to slack API >
  /// ```
  pub fn with_block_id(mut self, block_id: impl ToString) -> Self {
    self.block_id = Some(block_id.to_string());
    self
  }

  /// Set the `hint` on this Input Block that appears below
  /// an input element in a lighter grey.
  ///
  /// # Arguments
  ///
  /// - `hint` - An optional hint that appears below an input element
  ///     in a lighter grey.
  ///     It must be a a [text object 🔗] with a `type` of `plain_text`.
  ///     Maximum length for the `text` in this field is 2000 characters.
  ///
  /// [text object 🔗]: https://api.slack.com/reference/messaging/composition-objects#text
  ///
  /// # Example
  /// ```
  /// use slack_blocks::elems::Select;
  /// use slack_blocks::blocks;
  ///
  /// # use std::error::Error;
  /// # pub fn main() -> Result<(), Box<dyn Error>> {
  /// let label = "On a scale from 1 - 5, how angsty are you?";
  /// let input = Select::from_placeholder_and_action_id("Pick a channel...", "ABC123")
  ///     .choose_from_public_channels();
  ///
  /// let block = blocks::input
  ///     ::Contents
  ///     ::from_label_and_element(label, input)
  ///     .with_hint("PSST hey! Don't let them know how angsty you are!");
  ///
  /// // < send to slack API >
  /// # Ok(())
  /// # }
  /// ```
  pub fn with_hint(mut self, hint: impl Into<text::Plain>) -> Self {
    self.hint = Some(hint.into().into());
    self
  }

  /// Set whether or not this input is Optional.
  ///
  /// # Arguments
  /// - `optionality` - A boolean that indicates whether the input
  ///     element may be empty when a user submits the modal.
  ///     Defaults to false.
  ///
  /// # Example
  /// ```
  /// use slack_blocks::elems::Select;
  /// use slack_blocks::blocks;
  ///
  /// let label = "On a scale from 1 - 5, how angsty are you?";
  /// let input = Select::from_placeholder_and_action_id("Pick a channel...", "ABC123")
  ///     .choose_from_public_channels();
  ///
  /// let block = blocks::input
  ///     ::Contents
  ///     ::from_label_and_element(label, input)
  ///     .with_hint("PSST hey! Don't even answer that!")
  ///     .with_optional(true);
  ///
  /// // < send to slack API >
  /// ```
  pub fn with_optional(mut self, optionality: bool) -> Self {
    self.optional = Some(optionality);
    self
  }

  /// Will allow the elements in this block to dispatch block_actions payloads. Defaults to false.
  pub fn dispatch_block_actions(mut self) -> Self {
    self.dispatch_action = Some(true);
    self
  }

  /// Validate that this Input block agrees with Slack's model requirements
  ///
  /// # Errors
  /// - If `from_label_and_element` was passed a Text object longer
  ///     than 2000 chars
  /// - If `with_hint` was called with a block id longer
  ///     than 2000 chars
  /// - If `with_block_id` was called with a block id longer
  ///     than 256 chars
  ///
  /// # Example
  /// ```
  /// use slack_blocks::elems::Select;
  /// use slack_blocks::blocks;
  ///
  /// let label = "On a scale from 1 - 5, how angsty are you?";
  /// let input = Select::from_placeholder_and_action_id("Pick a channel...", "ABC123")
  ///     .choose_from_public_channels();
  /// let long_string = std::iter::repeat(' ').take(2001).collect::<String>();
  ///
  /// let block = blocks::input
  ///     ::Contents
  ///     ::from_label_and_element(label, input)
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

/// Enum representing the [`BlockElement` 🔗] types
/// supported by InputElement.
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum InputElement<'a> {
  #[serde(rename = "channels_select")]
  SelectPublicChannel(select::PublicChannel<'a>),
  Checkboxes,
  DatePicker,
  MultiSelect,
  TextInput(TextInput<'a>),
  Radio(Radio<'a>),
}

use select::PublicChannel as SelectPublicChannel;
convert! {
    impl<'_> From<SelectPublicChannel> for InputElement
        => |contents| InputElement::SelectPublicChannel(contents)
}

convert!(impl<'a> From<Radio<'a>> for InputElement<'a> => |r| InputElement::Radio(r));
convert!(impl<'a> From<TextInput<'a>> for InputElement<'a> => |r| InputElement::TextInput(r));

mod validate {
  use crate::{compose::text,
              val_helpr::{below_len, ValidatorResult}};

  pub(super) fn label(text: &text::Text) -> ValidatorResult {
    below_len("Input Label", 2000, text.as_ref())
  }

  pub(super) fn hint(text: &text::Text) -> ValidatorResult {
    below_len("Input Hint", 2000, text.as_ref())
  }
}
