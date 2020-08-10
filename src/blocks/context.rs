use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::convert;
use crate::text;
use crate::val_helpr::ValidationResult;

/// # Context Block
///
/// _[slack api docs 🔗][context_docs]_
///
/// Displays message context, which can include both images and text.
///
/// [context_docs]: https://api.slack.com/reference/block-kit/blocks#context
#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq, Serialize, Validate)]
pub struct Contents {
    #[validate(length(max = 10))]
    elements: Vec<Compose>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 255))]
    block_id: Option<String>,
}

impl Contents {
    /// Create an empty Context block (shorthand for `Default::default()`)
    ///
    /// # Example
    /// ```
    /// use slack_blocks::blocks::{Block, context};
    /// use slack_blocks::text;
    ///
    /// let context = context::Contents::new()
    ///     .with_element(text::Plain::from("my unformatted text"));
    ///
    /// let block: Block = context.into();
    /// // < send block to slack's API >
    /// ```
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the `block_id` for interactions on an existing `context::Contents`
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
    /// use slack_blocks::blocks::{Block, context};
    /// use slack_blocks::text;
    ///
    /// let text = text::Mrkdwn::from("_flavor_ *text*");
    /// let context: Block = context::Contents::new()
    ///     .with_element(text)
    ///     .with_block_id("msg_id_12346")
    ///     .into();
    ///
    /// // < send block to slack's API >
    /// ```
    pub fn with_block_id(mut self, block_id: impl ToString) -> Self {
        self.block_id = Some(block_id.to_string());
        self
    }

    /// Add a composition object to a context block.
    ///
    /// This is chainable, and can be used to easily
    /// populate the elements of a context block
    /// right after invoking `new`.
    ///
    /// # Arguments
    /// - `element` - A composition object;
    ///     Must be image elements or text objects.
    ///     Maximum number of items is 10.
    ///
    /// # Example
    /// ```
    /// use slack_blocks::blocks::{Block, context};
    /// use slack_blocks::text;
    ///
    /// let context = context::Contents::new()
    ///     .with_element(text::Plain::from("my unformatted text"));
    ///
    /// let block: Block = context.into();
    /// // < send block to slack's API >
    /// ```
    pub fn with_element(mut self, element: impl Into<self::Compose>) -> Self {
        self.elements.push(element.into());
        self
    }

    /// Construct a new `context::Contents` from a collection of
    /// composition objects that are may not be supported by Context
    /// Blocks.
    ///
    /// If you _can't_ guarantee that a collection only contains image
    /// or text objects, `from_elements` may be more ergonomic for you.
    ///
    /// # Arguments
    /// - `elements` - An array of composition objects;
    ///     Must be image elements or text objects.
    ///     Maximum number of items is 10.
    ///
    /// # Examples
    /// ```
    /// use slack_blocks::blocks::{Block, context};
    /// use slack_blocks::text;
    ///
    /// pub fn main() {
    ///     let objs: Vec<text::Mrkdwn> = vec![
    ///         text::Mrkdwn::from("*s i c k*"),
    ///         text::Mrkdwn::from("*t i g h t*"),
    ///     ];
    ///     let context = context::Contents::from_context_elements(objs);
    ///     let block: Block = context.into();
    ///     // < send block to slack's API >
    /// }
    /// ```
    pub fn from_context_elements(elements: impl IntoIterator<Item = impl Into<Compose>>) -> Self {
        elements
            .into_iter()
            .map(Into::<self::Compose>::into)
            .collect::<Vec<_>>()
            .into()
    }

    /// Validate that this Context block agrees with Slack's model requirements
    ///
    /// # Errors
    /// - If `with_block_id` was called with a block id longer
    ///     than 255 chars
    /// - If `from_elements`, `from_context_elements`, or `with_element` was called with
    ///     more than 10 objects
    ///
    /// # Example
    /// ```
    /// use slack_blocks::blocks;
    ///
    /// let long_string = std::iter::repeat(' ').take(256).collect::<String>();
    ///
    /// let block = blocks::context
    ///     ::Contents
    ///     ::new()
    ///     .with_block_id(long_string);
    ///
    /// assert_eq!(true, matches!(block.validate(), Err(_)));
    /// ```
    pub fn validate(&self) -> ValidationResult {
        Validate::validate(self)
    }
}

impl From<Vec<Compose>> for Contents {
    fn from(elements: Vec<Compose>) -> Self {
        Self {
            elements,
            ..Default::default()
        }
    }
}

/// The Composition objects supported by this block
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub enum Compose {
    Text(text::Text),
    Image,
}

convert!(impl From<text::Text> for Compose => |txt| Compose::Text(txt));

impl From<text::plain::Contents> for Compose {
    fn from(text: text::plain::Contents) -> Self {
        Into::<text::Text>::into(text).into()
    }
}

impl From<text::Mrkdwn> for Compose {
    fn from(text: text::Mrkdwn) -> Self {
        Into::<text::Text>::into(text).into()
    }
}
