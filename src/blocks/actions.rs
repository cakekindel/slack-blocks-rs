use std::convert::{TryFrom, TryInto};
use validator::Validate;
use serde::{Serialize, Deserialize};

use crate::val_helpr::ValidationResult;
use crate::block_elements;

#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq, Serialize, Validate)]
pub struct Contents {
    /// An array of interactive [element objects 🔗]
    /// - [buttons 🔗]
    /// - [select menus 🔗]
    /// - [overflow menus 🔗]
    /// - [date pickers 🔗]
    ///
    /// There is a maximum of 5 elements in each action block.
    ///
    /// [element objects 🔗]: https://api.slack.com/reference/messaging/block-elements
    /// [buttons 🔗]: https://api.slack.com/reference/messaging/block-elements#button
    /// [select menus 🔗]: https://api.slack.com/reference/messaging/block-elements#select
    /// [overflow menus 🔗]: https://api.slack.com/reference/messaging/block-elements#overflow
    /// [date pickers 🔗]: https://api.slack.com/reference/messaging/block-elements#datepicker
    #[validate(length(max = 5))]
    elements: Vec<BlockElement>,

    /// A string acting as a unique identifier for a block.
    ///
    /// You can use this `block_id` when you receive an
    /// interaction payload to [identify the source of the action 🔗].
    ///
    /// If not specified, a `block_id` will be generated.
    ///
    /// Maximum length for this field is 255 characters.
    ///
    /// [identify the source of the action 🔗]: https://api.slack.com/interactivity/handling#payloads
    block_id: Option<String>,
}

impl Contents {
    /// Create an empty Actions block
    pub fn new() -> Self {
        Default::default()
    }

    /// Populate an Actions block with a collection of `BlockElement`s that
    /// may not be supported by `Actions` blocks.
    ///
    /// For an infallible version of this conversion function, see `from_action_elements`.
    ///
    /// ### Errors
    /// Errors if the `block_elements::BlockElement` is one that is not supported by
    /// `Actions` blocks.
    ///
    /// For a list of `BlockElement` types that are, see `self::BlockElement`.
    ///
    /// ### Runtime Validation
    /// **only** validates that the block elements are compatible with `Actions`,
    /// for full runtime model validation see the `validate` method.
    pub fn from_elements<Els: Into<Vec<block_elements::BlockElement>>>(elements: Els) -> Result<Self, ()> {
        elements // Into<Vec>
            .into() // Vec
            .try_into() // Result<Vec>
    }

    /// Populate an Actions block with a collection of `BlockElement`s that
    /// are supported by `Actions` blocks.
    ///
    /// For slightly easier to use (but fallible) version of this conversion function,
    /// see `from_action_elements`.
    ///
    /// ### Runtime Validation
    /// **only** validates that the block elements are compatible with `Actions`,
    /// for full runtime model validation see the `validate` method.
    pub fn from_action_elements<Els: Into<Vec<self::BlockElement>>>(elements: Els) -> Self {
        elements // Into<Vec>
            .into() // Vec
            .into() // Contents
    }

    /// Validate the entire block and all of its
    /// elements against Slack's model requirements
    pub fn validate(&self) -> ValidationResult {
        Validate::validate(self)
    }
}

// TryFrom implementation backing `Contents::from_elements`
impl TryFrom<Vec<block_elements::BlockElement>> for Contents {
    type Error = ();
    fn try_from(elements: Vec<block_elements::BlockElement>) -> Result<Self, Self::Error> {
        elements.into_iter()
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
        use block_elements::BlockElement as El;
        use self::BlockElement::*;

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

