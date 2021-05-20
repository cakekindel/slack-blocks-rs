use std::convert::{TryFrom, TryInto};

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{block_elements,
            block_elements::{select, Button, Overflow, Radio, TextInput},
            convert,
            val_helpr::ValidationResult};

/// # Actions Block
///
/// [slack api docs 🔗]
///
/// A block that is used to hold interactive [elements 🔗]
///
/// [slack api docs 🔗]: https://api.slack.com/reference/block-kit/blocks#actions
/// [elements 🔗]: https://api.slack.com/reference/messaging/block-elements
#[derive(Clone,
           Debug,
           Default,
           Deserialize,
           Hash,
           PartialEq,
           Serialize,
           Validate)]
pub struct Contents<'a> {
  #[validate(length(max = 5))]
  elements: Vec<BlockElement<'a>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[validate(length(max = 255))]
  block_id: Option<String>,
}

impl<'a> Contents<'a> {
  /// Create an empty Actions block (shorthand for `Default::default()`)
  ///
  /// # Example
  /// ```
  /// use slack_blocks::blocks::{actions, Block};
  ///
  /// let actions = actions::Contents::new();
  /// let block: Block = actions.into();
  /// // < send block to slack's API >
  /// ```
  pub fn new() -> Self {
    Default::default()
  }

  /// Set the `block_id` for interactions on an existing `actions::Contents`
  ///
  /// # Arguments
  /// - `block_id` - A string acting as a unique identifier for a block.
  ///     You can use this `block_id` when you receive an interaction payload
  ///     to [identify the source of the action 🔗].
  ///     If not specified, a `block_id` will be generated.
  ///     Maximum length for this field is 255 characters.
  ///
  /// [identify the source of the action 🔗]: https://api.slack.com/interactivity/handling#payloads
  ///
  /// # Example
  /// ```
  /// use slack_blocks::blocks::{actions, Block};
  ///
  /// let actions = actions::Contents::new().with_block_id("tally_ho");
  /// let block: Block = actions.into();
  /// // < send block to slack's API >
  /// ```
  pub fn with_block_id(mut self, block_id: impl ToString) -> Self {
    self.block_id = Some(block_id.to_string());
    self
  }

  /// Populate an Actions block with a collection of `block_elements::BlockElement`s,
  /// which may not be supported by `Actions` blocks.
  ///
  /// If you _can_ create a collection of `actions::BlockElement`,
  /// either by creating them directly or invoking `block_elements::BlockElement::into`,
  /// use `from_action_elements`.
  ///
  /// # Arguments
  /// - `elements` - An array of interactive [element objects 🔗]
  ///     For a list of `BlockElement` types that are, see `BlockElement`.
  ///     There is a maximum of 5 elements in each action block.
  ///
  /// [element objects 🔗]: https://api.slack.com/reference/messaging/block-elements
  ///
  /// # Errors
  /// Errors if the `block_elements::BlockElement` is one that is not supported by
  /// `Actions` blocks.
  ///
  /// For a list of `BlockElement` types that are supported, see `::blocks::actions::BlockElement`.
  ///
  /// # Runtime Validation
  ///
  /// **only** validates that the block elements are compatible with `Actions`,
  /// for full runtime model validation see the `validate` method.
  ///
  /// # Example
  /// ```
  /// use slack_blocks::{block_elements,
  ///                    blocks::{actions, Block},
  ///                    compose};
  ///
  /// # pub fn main() -> Result<(), ()> {
  /// let btn = block_elements::Button::from_text_and_action_id("Button", "123");
  /// let actions = actions::Contents::from_elements(vec![btn.into()])?;
  /// let block: Block = actions.into();
  /// // < send block to slack's API >
  /// # Ok(())
  /// # }
  /// ```
  pub fn from_elements<Iter>(elements: Iter) -> Result<Self, ()>
    where Iter: IntoIterator<Item = block_elements::BlockElement<'a>>
  {
    elements.into_iter().collect::<Vec<_>>().try_into()
  }

  /// Populate an Actions block with a collection of `BlockElement`s that
  /// are supported by `Actions` blocks.
  ///
  /// This also can be called via the `From<Vec<self::BlockElement>>` implementation.
  ///
  /// If you have a collection of elements that may not be supported,
  /// see `from_elements`.
  ///
  /// # Arguments
  /// - `elements` - An array of interactive [element objects 🔗]
  ///     For a list of `BlockElement` types that are supported, see `BlockElement`.
  ///     There is a maximum of 5 elements in each action block.
  ///     Note that if you only ever want 1 item you can choose to pass it `Some(element)` OR `std::iter::once(element)`
  ///     instead of a `Vec`, bypassing an expensive allocation.
  ///     [Iterator and Option implement IntoIterator 🔗].
  ///
  /// [element objects 🔗]: https://api.slack.com/reference/messaging/block-elements
  /// [Iterator and Option implement IntoIterator 🔗]: https://doc.rust-lang.org/std/iter/trait.IntoIterator.html#impl-IntoIterator-28
  ///
  /// # Errors
  /// Errors if the `block_elements::BlockElement` is one that is not supported by
  /// `Actions` blocks.
  ///
  /// # Runtime Validation
  /// **only** validates that the block elements are compatible with `Actions`,
  /// for full runtime model validation see the `validate` method.
  ///
  /// # Example
  /// ```
  /// use slack_blocks::{block_elements,
  ///                    blocks::{actions, Block},
  ///                    compose};
  ///
  /// # pub fn main() {
  /// let btn = block_elements::Button::from_text_and_action_id("Button", "123");
  /// let actions = actions::Contents::from_action_elements(vec![btn.into()]);
  /// let block: Block = actions.into();
  ///
  /// // < send block to slack's API >
  /// # }
  /// ```
  pub fn from_action_elements<Iter>(elements: Iter) -> Self
    where Iter: IntoIterator<Item = self::BlockElement<'a>>
  {
    elements.into_iter().collect::<Vec<_>>().into()
  }

  /// Validate that this Section block agrees with Slack's model requirements
  ///
  /// # Errors
  /// - If `with_block_id` was called with a block id longer
  ///     than 255 chars
  /// - If `from_elements` or `from_action_elements` was
  ///     called with more than 5 elements.
  ///
  /// # Example
  /// ```
  /// use slack_blocks::{blocks, compose};
  ///
  /// let long_string = std::iter::repeat(' ').take(256).collect::<String>();
  ///
  /// let block = blocks::actions
  ///     ::Contents
  ///     ::from_action_elements(vec![])
  ///     .with_block_id(long_string);
  ///
  /// assert!(matches!(block.validate(), Err(_)));
  /// ```
  pub fn validate(&self) -> ValidationResult {
    Validate::validate(self)
  }
}

/// The Block Elements supported in an Action Block.
///
/// This list was pulled from the docs for all [block elements 🔗],
/// where each declares the blocks it is usable in.
///
/// [block elements 🔗]: https://api.slack.com/reference/block-kit/block-elements
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub enum BlockElement<'a> {
  Button(Button),
  Checkboxes,
  DatePicker,
  Overflow(Overflow<'a>),
  TextInput(TextInput<'a>),
  RadioButtons(Radio<'a>),

  /// All Select types are supported.
  SelectPublicChannel(select::PublicChannel<'a>),

  /// All Select types are supported.
  SelectConversation(select::Conversation<'a>),

  /// All Select types are supported.
  SelectUser(select::User<'a>),

  /// All Select types are supported.
  SelectExternal(select::External<'a>),

  /// All Select types are supported.
  SelectStatic(select::Static<'a>),
}

convert!(impl<'a> From<Vec<self::BlockElement<'a>>> for Contents<'a>
    => |elements| Self {
        elements,
        ..Default::default()
    }
);

impl<'a> TryFrom<block_elements::BlockElement<'a>> for Contents<'a> {
  type Error = ();
  fn try_from(element: block_elements::BlockElement<'a>)
              -> Result<Self, Self::Error> {
    self::BlockElement::<'a>::try_from(element)
      .map(|el| Self::from_action_elements(std::iter::once(el)))
  }
}

impl<'a> TryFrom<Vec<block_elements::BlockElement<'a>>> for Contents<'a> {
  type Error = ();
  fn try_from(elements: Vec<block_elements::BlockElement<'a>>)
              -> Result<Self, Self::Error> {
    elements.into_iter()
            .map(self::BlockElement::<'a>::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map(self::Contents::<'a>::from)
  }
}

impl<'a> TryFrom<block_elements::BlockElement<'a>> for self::BlockElement<'a> {
  type Error = ();
  fn try_from(el: block_elements::BlockElement<'a>)
              -> Result<Self, Self::Error> {
    use block_elements::BlockElement as El;

    use self::BlockElement::*;

    match el {
      | El::SelectPublicChannel(sel) => Ok(SelectPublicChannel(sel)),
      | El::SelectConversation(sel) => Ok(SelectConversation(sel)),
      | El::SelectExternal(sel) => Ok(SelectExternal(sel)),
      | El::SelectStatic(sel) => Ok(SelectStatic(sel)),
      | El::SelectUser(sel) => Ok(SelectUser(sel)),
      | El::Overflow(o) => Ok(Overflow(o)),
      | El::RadioButtons(r) => Ok(RadioButtons(r)),
      | El::Button(cts) => Ok(Button(cts)),
      | El::TextInput(t) => Ok(TextInput(t)),
      | El::Checkboxes => Ok(Checkboxes),
      | El::DatePicker => Ok(DatePicker),
      | _ => Err(()),
    }
  }
}

convert!(impl<'a> From<select::PublicChannel<'a>> for self::BlockElement<'a> => |s| self::BlockElement::SelectPublicChannel(s));
convert!(impl<'a> From<select::Conversation<'a>> for self::BlockElement<'a>  => |s| self::BlockElement::SelectConversation(s));
convert!(impl<'a> From<select::User<'a>> for self::BlockElement<'a>  => |s| self::BlockElement::SelectUser(s));
convert!(impl<'a> From<select::External<'a>> for self::BlockElement<'a>  => |s| self::BlockElement::SelectExternal(s));
convert!(impl<'a> From<select::Static<'a>> for self::BlockElement<'a>  => |s| self::BlockElement::SelectStatic(s));
convert!(impl     From<Button> for self::BlockElement<'static> => |b| self::BlockElement::Button(b));
convert!(impl<'a> From<Radio<'a>> for self::BlockElement<'a> => |b| self::BlockElement::RadioButtons(b));
convert!(impl<'a> From<TextInput<'a>> for self::BlockElement<'a> => |t| self::BlockElement::TextInput(t));
