//! # Actions Block
//!
//! [slack api docs 🔗]
//!
//! A block that is used to hold interactive [elements 🔗]
//!
//! [slack api docs 🔗]: https://api.slack.com/reference/block-kit/blocks#actions
//! [elements 🔗]: https://api.slack.com/reference/messaging/block-elements

use std::{borrow::Cow,
          convert::{TryFrom, TryInto}};

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{convert,
            elems::{select,
                    BlockElement,
                    Button,
                    Checkboxes,
                    DatePicker,
                    Overflow,
                    Radio,
                    TextInput},
            val_helpr::*};

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
pub struct Actions<'a> {
  #[validate(length(max = 5))]
  elements: Vec<SupportedElement<'a>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[validate(custom = "super::validate_block_id")]
  block_id: Option<Cow<'a, str>>,
}

impl<'a> Actions<'a> {
  /// Build a new Actions block.
  ///
  /// For example, see docs for ActionsBuilder.
  pub fn builder() -> build::ActionsBuilderInit<'a> {
    build::ActionsBuilderInit::new()
  }

  /// Create an empty Actions block (shorthand for `Default::default()`)
  ///
  /// # Example
  /// ```
  /// use slack_blocks::blocks;
  ///
  /// let actions: blocks::Block = blocks::Actions::new().into();
  /// // < send block to slack's API >
  /// ```
  #[deprecated(since = "0.19.1", note = "use Actions::builder")]
  pub fn new() -> Self {
    Default::default()
  }

  /// Set the `block_id` for interactions on an existing `Actions`
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
  /// use slack_blocks::blocks::{Actions, Block};
  ///
  /// let actions = Actions::new().with_block_id("tally_ho");
  /// let block: Block = actions.into();
  /// // < send block to slack's API >
  /// ```
  #[deprecated(since = "0.19.1", note = "use Actions::builder")]
  pub fn with_block_id(mut self, block_id: impl Into<Cow<'a, str>>) -> Self {
    self.block_id = Some(block_id.into());
    self
  }

  /// Populate an Actions block with a collection of `BlockElement`s,
  /// which may not be supported by `Actions` blocks.
  ///
  /// If you _can_ create a collection of `actions::BlockElement`,
  /// either by creating them directly or invoking `BlockElement::into`,
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
  /// Errors if the `BlockElement` is one that is not supported by
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
  /// use slack_blocks::{blocks::{Actions, Block},
  ///                    compose,
  ///                    elems};
  ///
  /// # pub fn main() -> Result<(), ()> {
  /// let btn = elems::Button::from_text_and_action_id("Button", "123");
  /// let actions = Actions::from_elements(vec![btn.into()])?;
  /// let block: Block = actions.into();
  /// // < send block to slack's API >
  /// # Ok(())
  /// # }
  /// ```
  #[deprecated(since = "0.19.1", note = "use Actions::builder")]
  pub fn from_elements<Iter>(elements: Iter) -> Result<Self, ()>
    where Iter: IntoIterator<Item = BlockElement<'a>>
  {
    elements.into_iter().collect::<Vec<_>>().try_into()
  }

  /// Populate an Actions block with a collection of `BlockElement`s that
  /// are supported by `Actions` blocks.
  ///
  /// This also can be called via the `From<Vec<self::SupportedElement>>` implementation.
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
  /// Errors if the `BlockElement` is one that is not supported by
  /// `Actions` blocks.
  ///
  /// # Runtime Validation
  /// **only** validates that the block elements are compatible with `Actions`,
  /// for full runtime model validation see the `validate` method.
  ///
  /// # Example
  /// ```
  /// use slack_blocks::{blocks::{Actions, Block},
  ///                    compose,
  ///                    elems};
  ///
  /// # pub fn main() {
  /// let btn = elems::Button::from_text_and_action_id("Button", "123");
  /// let actions = Actions::from_action_elements(vec![btn.into()]);
  /// let block: Block = actions.into();
  ///
  /// // < send block to slack's API >
  /// # }
  /// ```
  #[deprecated(since = "0.19.1", note = "use Actions::builder")]
  pub fn from_action_elements<Iter>(elements: Iter) -> Self
    where Iter: IntoIterator<Item = self::SupportedElement<'a>>
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
  /// let block =
  ///   blocks::Actions::from_action_elements(vec![]).with_block_id(long_string);
  ///
  /// assert!(matches!(block.validate(), Err(_)));
  /// ```
  pub fn validate(&self) -> ValidationResult {
    Validate::validate(self)
  }
}

/// Actions block builder
pub mod build {
  use std::marker::PhantomData;

  use super::*;
  use crate::build::*;

  /// Compile-time markers for builder methods
  #[allow(non_camel_case_types)]
  pub mod method {
    /// ActionsBuilder.elements
    #[derive(Clone, Copy, Debug)]
    pub struct elements;
  }

  /// Initial state for `ActionsBuilder`
  pub type ActionsBuilderInit<'a> =
    ActionsBuilder<'a, RequiredMethodNotCalled<method::elements>>;

  /// Build an Actions block
  ///
  /// Allows you to construct safely, with compile-time checks
  /// on required setter methods.
  ///
  /// # Required Methods
  /// `ActionsBuilder::build()` is only available if these methods have been called:
  ///  - `element`
  ///
  /// # Example
  /// ```
  /// use slack_blocks::{blocks::Actions, elems::Button};
  ///
  /// let block = Actions::builder().element(Button::builder().text("Click me!")
  ///                                                         .action_id("clicked")
  ///                                                         .build())
  ///                               .build();
  /// ```
  #[derive(Debug)]
  pub struct ActionsBuilder<'a, Elements> {
    elements: Option<Vec<SupportedElement<'a>>>,
    block_id: Option<Cow<'a, str>>,
    state: PhantomData<Elements>,
  }

  impl<'a, E> ActionsBuilder<'a, E> {
    /// Create a new ActionsBuilder
    pub fn new() -> Self {
      Self { elements: None,
             block_id: None,
             state: PhantomData::<_> }
    }

    /// Add an `element` (**Required**, can be called many times)
    ///
    /// Add an interactive [element object 🔗]
    ///
    /// For a list of `BlockElement` types that are supported, see `slack_blocks::blocks::actions::SupportedElement`.
    ///
    /// There is a maximum of 5 elements in each action block.
    ///
    /// [element object 🔗]: https://api.slack.com/reference/messaging/block-elements
    pub fn element<El>(self,
                       element: El)
                       -> ActionsBuilder<'a, Set<method::elements>>
      where El: Into<SupportedElement<'a>>
    {
      let mut elements = self.elements.unwrap_or_default();
      elements.push(element.into());

      ActionsBuilder { block_id: self.block_id,
                       elements: Some(elements),
                       state: PhantomData::<_> }
    }

    /// Set `block_id` (Optional)
    ///
    /// A string acting as a unique identifier for a block.
    ///
    /// You can use this `block_id` when you receive an interaction payload
    /// to [identify the source of the action 🔗].
    ///
    /// If not specified, a `block_id` will be generated.
    ///
    /// Maximum length for this field is 255 characters.
    ///
    /// [identify the source of the action 🔗]: https://api.slack.com/interactivity/handling#payloads
    pub fn block_id<S>(mut self, block_id: S) -> Self
      where S: Into<Cow<'a, str>>
    {
      self.block_id = Some(block_id.into());
      self
    }
  }

  impl<'a> ActionsBuilder<'a, Set<method::elements>> {
    /// All done building, now give me a darn actions block!
    ///
    /// > `no method name 'build' found for struct 'ActionsBuilder<...>'`?
    /// Make sure all required setter methods have been called. See docs for `ActionsBuilder`.
    ///
    /// ```compile_fail
    /// use slack_blocks::blocks::Actions;
    ///
    /// let foo = Actions::builder().build(); // Won't compile!
    /// ```
    ///
    /// ```
    /// use slack_blocks::{blocks::Actions, elems::Button};
    ///
    /// let block = Actions::builder().element(Button::builder().text("Click me!")
    ///                                                         .action_id("clicked")
    ///                                                         .build())
    ///                               .build();
    /// ```
    pub fn build(self) -> Actions<'a> {
      Actions { elements: self.elements.unwrap(),
                block_id: self.block_id }
    }
  }
}

/// The Block Elements supported in an Action Block.
///
/// Supports:
/// - Overflow
/// - RadioButtons
/// - Button
/// - TextInput
/// - Checkboxes
/// - DatePicker
/// - Select Menus:
///   - PublicChannel
///   - Conversation
///   - External
///   - Static
///   - User
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct SupportedElement<'a>(BlockElement<'a>);

convert!(impl<'a> From<Vec<self::SupportedElement<'a>>> for Actions<'a>
    => |elements| Self {
        elements,
        ..Default::default()
    }
);

impl<'a> TryFrom<BlockElement<'a>> for Actions<'a> {
  type Error = ();
  fn try_from(element: BlockElement<'a>) -> Result<Self, Self::Error> {
    self::SupportedElement::<'a>::try_from(element)
      .map(|el| Self::from_action_elements(std::iter::once(el)))
  }
}

impl<'a> TryFrom<Vec<BlockElement<'a>>> for Actions<'a> {
  type Error = ();
  fn try_from(elements: Vec<BlockElement<'a>>) -> Result<Self, Self::Error> {
    elements.into_iter()
            .map(self::SupportedElement::<'a>::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map(self::Actions::<'a>::from)
  }
}

impl<'a> TryFrom<BlockElement<'a>> for self::SupportedElement<'a> {
  type Error = ();
  fn try_from(el: BlockElement<'a>) -> Result<Self, Self::Error> {
    use BlockElement as El;

    match el {
      | El::SelectPublicChannel(_)
      | El::SelectConversation(_)
      | El::SelectExternal(_)
      | El::SelectStatic(_)
      | El::SelectUser(_)
      | El::Overflow(_)
      | El::RadioButtons(_)
      | El::Button(_)
      | El::TextInput(_)
      | El::Checkboxes(_)
      | El::DatePicker(_) => Ok(SupportedElement(el)),
      | _ => Err(()),
    }
  }
}

convert!(impl<'a> From<select::PublicChannel<'a>> for self::SupportedElement<'a> => |s| self::SupportedElement(BlockElement::from(s)));
convert!(impl<'a> From<select::Conversation<'a>> for self::SupportedElement<'a>  => |s| self::SupportedElement(BlockElement::from(s)));
convert!(impl<'a> From<select::User<'a>> for self::SupportedElement<'a>  => |s| self::SupportedElement(BlockElement::from(s)));
convert!(impl<'a> From<select::External<'a>> for self::SupportedElement<'a>  => |s| self::SupportedElement(BlockElement::from(s)));
convert!(impl<'a> From<select::Static<'a>> for self::SupportedElement<'a>  => |s| self::SupportedElement(BlockElement::from(s)));
convert!(impl<'a> From<Button<'a>> for self::SupportedElement<'a> => |b| self::SupportedElement(BlockElement::from(b)));
convert!(impl<'a> From<Radio<'a>> for self::SupportedElement<'a> => |b| self::SupportedElement(BlockElement::from(b)));
convert!(impl<'a> From<TextInput<'a>> for self::SupportedElement<'a> => |t| self::SupportedElement(BlockElement::from(t)));
convert!(impl<'a> From<DatePicker<'a>> for self::SupportedElement<'a> => |t| self::SupportedElement(BlockElement::from(t)));
convert!(impl<'a> From<Checkboxes<'a>> for self::SupportedElement<'a> => |t| self::SupportedElement(BlockElement::from(t)));
convert!(impl<'a> From<Overflow<'a>> for self::SupportedElement<'a> => |t| self::SupportedElement(BlockElement::from(t)));
