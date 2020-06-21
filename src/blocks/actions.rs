use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};
use validator::Validate;

use crate::block_elements;
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

    #[validate(length(max = 255))]
    block_id: Option<String>,
}

impl Contents {
    /// Create an empty Actions block (shorthand for `Default::default()`)
    ///
    /// # Example
    /// ```
    /// # use slack_blocks::blocks::{Block, actions};
    ///
    /// # pub fn main() {
    /// let actions = actions::Contents::new();
    /// let block: Block = actions.into();
    /// // < send block to slack's API >
    /// # }
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
    /// # use slack_blocks::blocks::{Block, actions};
    ///
    /// # pub fn main() {
    /// let actions = actions::Contents::new().with_block_id("tally_ho");
    /// let block: Block = actions.into();
    /// // < send block to slack's API >
    /// # }
    /// ```
    pub fn with_block_id<StrIsh: AsRef<str>>(mut self, block_id: StrIsh) -> Self {
        self.block_id = Some(block_id.as_ref().to_string());
        self
    }

    /// Populate an Actions block with a collection of `BlockElement`s that
    /// may not be supported by `Actions` blocks.
    ///
    /// For an infallible version of this conversion function, see `from_action_elements`.
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
    /// pub fn main() -> Result<(), ()> {
    ///     let btn = block_elements::BlockElement::Button;
    ///     let actions = actions::Contents::from_elements(vec![btn])?;
    ///     let block: Block = actions.into();
    ///     // < send block to slack's API >
    ///     Ok(())
    /// }
    /// ```
    pub fn from_elements(
        elements: impl IntoIterator<Item = block_elements::BlockElement>,
    ) -> Result<Self, ()> {
        elements
            .into_iter()
            .collect::<Vec<_>>()
            .try_into()
    }

    /// Populate an Actions block with a collection of `BlockElement`s that
    /// are supported by `Actions` blocks.
    ///
    /// For slightly easier to use (but fallible) version of this conversion function,
    /// see `from_action_elements`.
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
    /// pub fn main() {
    ///     let btn = actions::BlockElement::Button;
    ///     let actions = actions::Contents::from_action_elements(vec![btn]);
    ///     let block: Block = actions.into();
    ///     // < send block to slack's API >
    /// }
    /// ```
    pub fn from_action_elements<Els: IntoIterator<Item = self::BlockElement>>(
        elements: Els,
    ) -> Self {
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
    ///     than 256 chars
    /// - If `from_elements` or `from_action_elements` was
    ///     called with a block id longer than 256 chars
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

// TryFrom implementation backing `Contents::from_elements`
impl TryFrom<Vec<block_elements::BlockElement>> for Contents {
    type Error = ();
    fn try_from(elements: Vec<block_elements::BlockElement>) -> Result<Self, Self::Error> {
        elements
            .into_iter()
            // Try to convert the bag of "any block element" to "block element supported by Actions"
            .map(TryInto::try_into)
            // If we hit one that is not supported, stop and continue with err
            .collect::<Result<_, _>>()
            // If it went ok, convert the supported elements into Contents
            .map(|els: Vec<self::BlockElement>| -> Self { els.into() })
    }
}

// From implementation backing `Contents::from_action_elements`
impl From<Vec<self::BlockElement>> for Contents {
    fn from(elements: Vec<self::BlockElement>) -> Self {
        Self {
            elements,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub enum BlockElement {
    Button,
    Checkboxes,
    DatePicker,
    OverflowMenu,
    PlainInput,
    RadioButtons,
    Select(block_elements::select::Contents),
}

impl TryFrom<block_elements::BlockElement> for self::BlockElement {
    type Error = ();
    fn try_from(el: block_elements::BlockElement) -> Result<Self, Self::Error> {
        use self::BlockElement::*;
        use block_elements::BlockElement as El;

        match el {
            El::Button => Ok(Button),
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
