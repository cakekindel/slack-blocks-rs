use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};
use validator::Validate;

use crate::block_elements;
use crate::impl_from_contents;
use crate::val_helpr::ValidationResult;

/// # Actions Block
///
/// [slack api docs 🔗]
///
/// A block that is used to hold interactive [elements 🔗]
///
/// [slack api docs 🔗]: https://api.slack.com/reference/block-kit/blocks#actions
/// [elements 🔗]: https://api.slack.com/reference/messaging/block-elements
#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq, Serialize, Validate)]
pub struct Contents {
    #[validate(length(max = 5))]
    elements: Vec<BlockElement>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 255))]
    block_id: Option<String>,
}

impl Contents {
    /// Create an empty Actions block (shorthand for `Default::default()`)
    ///
    /// # Example
    /// ```
    /// use slack_blocks::blocks::{Block, actions};
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
    /// use slack_blocks::blocks::{Block, actions};
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
    /// For a list of `BlockElement` types that are, see `BlockElement`.
    ///
    /// # Runtime Validation
    ///
    /// **only** validates that the block elements are compatible with `Actions`,
    /// for full runtime model validation see the `validate` method.
    ///
    /// # Example
    /// ```
    /// use slack_blocks::blocks::{Block, actions};
    /// use slack_blocks::compose;
    /// use slack_blocks::block_elements;
    ///
    /// # pub fn main() -> Result<(), ()> {
    /// let btn = block_elements::Button::from_text_and_action_id("Button", "123");
    /// let actions = actions::Contents::from_elements(vec![btn.into()])?;
    /// let block: Block = actions.into();
    /// // < send block to slack's API >
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_elements(
        elements: impl IntoIterator<Item = block_elements::BlockElement>,
    ) -> Result<Self, ()> {
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
    ///
    /// [element objects 🔗]: https://api.slack.com/reference/messaging/block-elements
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
    /// use slack_blocks::blocks::{Block, actions};
    /// use slack_blocks::compose;
    /// use slack_blocks::block_elements;
    ///
    /// # pub fn main() {
    /// let btn = block_elements::Button::from_text_and_action_id("Button", "123");
    /// let actions = actions::Contents::from_action_elements(vec![btn.into()]);
    /// let block: Block = actions.into();
    ///
    /// // < send block to slack's API >
    /// # }
    /// ```
    pub fn from_action_elements(elements: impl IntoIterator<Item = self::BlockElement>) -> Self {
        elements
            .into_iter()
            .map(Into::<self::BlockElement>::into)
            .collect::<Vec<_>>()
            .into()
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
    /// use slack_blocks::blocks;
    /// use slack_blocks::compose;
    ///
    /// let long_string = std::iter::repeat(' ').take(256).collect::<String>();
    ///
    /// let block = blocks::actions
    ///     ::Contents
    ///     ::from_action_elements(vec![])
    ///     .with_block_id(long_string);
    ///
    /// assert_eq!(true, matches!(block.validate(), Err(_)));
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
pub enum BlockElement {
    Button(block_elements::Button),
    Checkboxes,
    DatePicker,
    OverflowMenu,
    PlainInput,
    RadioButtons,
    /// All Select types are supported.
    Select(block_elements::select::Contents),
}

impl From<Vec<self::BlockElement>> for Contents {
    fn from(elements: Vec<self::BlockElement>) -> Self {
        Self {
            elements,
            ..Default::default()
        }
    }
}

impl TryFrom<Vec<block_elements::BlockElement>> for Contents {
    type Error = ();
    fn try_from(elements: Vec<block_elements::BlockElement>) -> Result<Self, Self::Error> {
        elements
            .into_iter()
            .map(TryInto::<self::BlockElement>::try_into)
            .collect::<Result<Vec<_>, _>>()
            .map(Into::<self::Contents>::into)
    }
}

impl TryFrom<block_elements::BlockElement> for self::BlockElement {
    type Error = ();
    fn try_from(el: block_elements::BlockElement) -> Result<Self, Self::Error> {
        use self::BlockElement::*;
        use block_elements::BlockElement as El;

        match el {
            El::Button(cts) => Ok(Button(cts)),
            El::Checkboxes => Ok(Checkboxes),
            El::DatePicker => Ok(DatePicker),
            El::OverflowMenu => Ok(OverflowMenu),
            El::PlainInput => Ok(PlainInput),
            El::RadioButtons => Ok(RadioButtons),
            El::Select(contents) => Ok(Select(contents)),
            _ => Err(()),
        }
    }
}

impl_from_contents!(BlockElement, Button, block_elements::Button);
