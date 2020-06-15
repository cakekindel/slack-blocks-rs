use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::compose::Compose;
use crate::val_helpr::ValidationResult;

#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq, Serialize, Validate)]
pub struct Contents {
    /// A collection of [image elements 🔗] and [text objects 🔗].
    ///
    /// Maximum number of items is 10
    /// [image elements 🔗]: https://api.slack.com/reference/messaging/block-elements#image
    /// [text objects 🔗]: https://api.slack.com/reference/messaging/composition-objects#text
    #[validate(length(max = 10))]
    elements: Vec<Compose>,

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
    #[validate(length(max = 255))]
    block_id: Option<String>,
}

impl Contents {
    /// Construct a new empty Context block
    /// (uses `Default`)
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the `block_id` for interactions on an existing `context::Contents`
    ///
    /// ```
    /// use slack_blocks::blocks::{Block, context};
    ///
    /// pub fn main() {
    ///     let context = context::Contents::new().with_block_id("tally_ho");
    ///     let block: Block = context.into();
    ///     // < send block to slack's API >
    /// }
    /// ```
    pub fn with_block_id<StrIsh: AsRef<str>>(mut self, block_id: StrIsh) -> Self {
        self.block_id = Some(block_id.as_ref().to_string());
        self
    }

    /// Construct a new `context::Contents` from a bunch of
    /// composition objects
    ///
    /// ```
    /// use slack_blocks::blocks::{Block, context};
    /// use slack_blocks::compose;
    ///
    /// pub fn main() {
    ///     let text = compose::Text::markdown("*s i c k*");
    ///     let context = context::Contents::from_elements(vec![text]);
    ///     let block: Block = context.into();
    ///     // < send block to slack's API >
    /// }
    /// ```
    pub fn from_elements<Els: IntoIterator<Item = impl Into<Compose>>>(elements: Els) -> Self {
        elements
            .into_iter()
            .map(Into::<Compose>::into)
            .collect::<Vec<_>>()
            .into()
    }

    /// Validate that the model agrees with slack's validation
    /// requirements
    pub fn validate(&self) -> ValidationResult {
        Validate::validate(self)
    }
}

// From impl backing the `from_elements` constructor
impl From<Vec<Compose>> for Contents {
    fn from(elements: Vec<Compose>) -> Self {
        Self {
            elements,
            ..Default::default()
        }
    }
}
