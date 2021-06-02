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

use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{compose::text,
            convert,
            elems,
            elems::{select, BlockElement},
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
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
#[cfg_attr(feature = "validation", derive(Validate))]
pub struct Input<'a> {
  #[cfg_attr(feature = "validation", validate(custom = "validate::label"))]
  label: text::Text,

  element: SupportedElement<'a>,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[cfg_attr(feature = "validation",
             validate(custom = "super::validate_block_id"))]
  block_id: Option<Cow<'a, str>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[cfg_attr(feature = "validation", validate(custom = "validate::hint"))]
  hint: Option<text::Text>,

  #[serde(skip_serializing_if = "Option::is_none")]
  dispatch_action: Option<bool>,

  #[serde(skip_serializing_if = "Option::is_none")]
  optional: Option<bool>,
}

impl<'a> Input<'a> {
  /// Build a new input block
  ///
  /// For example, see `blocks::input::build::InputBuilder`.
  pub fn builder() -> build::InputBuilderInit<'a> {
    build::InputBuilderInit::new()
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
  /// use slack_blocks::{blocks, elems::select};
  ///
  /// let select =
  ///   select::PublicChannel::builder().placeholder("Pick a channel...")
  ///                                   .action_id("ABC123")
  ///                                   .build();
  ///
  /// let long_string = std::iter::repeat(' ').take(2001).collect::<String>();
  ///
  /// let block = blocks::Input
  ///     ::builder()
  ///     .label("On a scale from 1 - 5, how angsty are you?")
  ///     .element(select)
  ///     .block_id(long_string)
  ///     .build();
  ///
  /// assert_eq!(true, matches!(block.validate(), Err(_)));
  ///
  /// // < send to slack API >
  /// ```
  #[cfg(feature = "validation")]
  #[cfg_attr(docsrs, doc(cfg(feature = "validation")))]
  pub fn validate(&self) -> ValidationResult {
    Validate::validate(self)
  }
}

/// Input block builder
pub mod build {
  use std::marker::PhantomData;

  use super::*;
  use crate::build::*;

  /// Compile-time markers for builder methods
  #[allow(non_camel_case_types)]
  pub mod method {
    /// InputBuilder.element
    #[derive(Clone, Copy, Debug)]
    pub struct element;

    /// InputBuilder.label
    #[derive(Clone, Copy, Debug)]
    pub struct label;
  }

  /// Initial state for `InputBuilder`
  pub type InputBuilderInit<'a> =
    InputBuilder<'a,
                 RequiredMethodNotCalled<method::element>,
                 RequiredMethodNotCalled<method::label>>;

  /// Build an Input block
  ///
  /// Allows you to construct safely, with compile-time checks
  /// on required setter methods.
  ///
  /// # Required Methods
  /// `InputBuilder::build()` is only available if these methods have been called:
  ///  - `element`
  ///
  /// # Example
  /// ```
  /// use slack_blocks::{blocks::Input,
  ///                    compose::text::ToSlackPlaintext,
  ///                    elems::TextInput};
  ///
  /// let block =
  ///   Input::builder().label("foo".plaintext())
  ///                   .element(TextInput::builder().action_id("foo").build())
  ///                   .build();
  /// ```
  #[derive(Debug)]
  pub struct InputBuilder<'a, Element, Label> {
    label: Option<text::Text>,
    element: Option<SupportedElement<'a>>,
    hint: Option<text::Text>,
    block_id: Option<Cow<'a, str>>,
    optional: Option<bool>,
    dispatch_action: Option<bool>,
    state: PhantomData<(Element, Label)>,
  }

  impl<'a, E, L> InputBuilder<'a, E, L> {
    /// Create a new InputBuilder
    pub fn new() -> Self {
      Self { label: None,
             element: None,
             hint: None,
             block_id: None,
             optional: None,
             dispatch_action: None,
             state: PhantomData::<_> }
    }

    /// Set `label` (**Required**)
    ///
    /// A label that appears above an input element in the form of
    /// a [text object 🔗] that must have type of `plain_text`.
    ///
    /// Maximum length for the text in this field is 2000 characters.
    ///
    /// [text object 🔗]: https://api.slack.com/reference/messaging/composition-objects#text
    pub fn label<T>(self, label: T) -> InputBuilder<'a, E, Set<method::label>>
      where T: Into<text::Plain>
    {
      InputBuilder { label: Some(label.into().into()),
                     element: self.element,
                     hint: self.hint,
                     block_id: self.block_id,
                     optional: self.optional,
                     dispatch_action: self.dispatch_action,
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

    /// Set `dispatch_action` (Optional)
    ///
    /// Will allow the elements in this block to
    /// dispatch block_actions payloads.
    ///
    /// Defaults to false.
    pub fn dispatch_actions(mut self, should: bool) -> Self {
      self.dispatch_action = Some(should);
      self
    }

    /// Sets `optional` (**Required**)
    ///
    /// A boolean that indicates whether the input
    /// element may be empty when a user submits the modal.
    ///
    /// Defaults to false.
    pub fn optional(mut self, optional: bool) -> Self {
      self.optional = Some(optional);
      self
    }

    /// Set `hint` (Optional)
    ///
    /// An optional hint that appears below an input element
    /// in a lighter grey.
    ///
    /// Maximum length for the text in this field is 2000 characters.
    pub fn hint<T>(mut self, hint: T) -> Self
      where T: Into<text::Plain>
    {
      self.hint = Some(hint.into().into());
      self
    }
  }

  impl<'a, L> InputBuilder<'a, RequiredMethodNotCalled<method::element>, L> {
    /// Set `element` (**Required**)
    ///
    /// An interactive `block_element` that will be used to gather
    /// the input for this block.
    ///
    /// For the kinds of Elements supported by
    /// Input blocks, see the `SupportedElement` enum.
    pub fn element<El>(self,
                       element: El)
                       -> InputBuilder<'a, Set<method::element>, L>
      where El: Into<SupportedElement<'a>>
    {
      InputBuilder { label: self.label,
                     element: Some(element.into()),
                     hint: self.hint,
                     block_id: self.block_id,
                     optional: self.optional,
                     dispatch_action: self.dispatch_action,
                     state: PhantomData::<_> }
    }

    /// XML child alias for `element`
    #[cfg(feature = "blox")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blox")))]
    pub fn child<El>(self,
                     element: El)
                     -> InputBuilder<'a, Set<method::element>, L>
      where El: Into<SupportedElement<'a>>
    {
      self.element(element)
    }
  }

  impl<'a> InputBuilder<'a, Set<method::element>, Set<method::label>> {
    /// All done building, now give me a darn actions block!
    ///
    /// > `no method name 'build' found for struct 'InputBuilder<...>'`?
    /// Make sure all required setter methods have been called. See docs for `InputBuilder`.
    ///
    /// ```compile_fail
    /// use slack_blocks::blocks::Input;
    ///
    /// let foo = Input::builder().build(); // Won't compile!
    /// ```
    ///
    /// ```
    /// use slack_blocks::{blocks::Input,
    ///                    compose::text::ToSlackPlaintext,
    ///                    elems::TextInput};
    ///
    /// let block =
    ///   Input::builder().label("foo".plaintext())
    ///                   .element(TextInput::builder().action_id("foo").build())
    ///                   .build();
    /// ```
    pub fn build(self) -> Input<'a> {
      Input { element: self.element.unwrap(),
              label: self.label.unwrap(),
              hint: self.hint,
              dispatch_action: self.dispatch_action,
              optional: self.optional,
              block_id: self.block_id }
    }
  }
}

/// The Block Elements supported in an Input Block.
///
/// Supports:
/// - Radio Buttons
/// - Text Input
/// - Checkboxes
/// - Date Picker
/// - All Select Menus
/// - All Multi-Select Menus
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct SupportedElement<'a>(BlockElement<'a>);

convert!(impl<'a> From<elems::Radio<'a>> for SupportedElement<'a> => |r| SupportedElement(BlockElement::from(r)));
convert!(impl<'a> From<elems::TextInput<'a>> for SupportedElement<'a> => |r| SupportedElement(BlockElement::from(r)));
convert!(impl<'a> From<elems::Checkboxes<'a>> for SupportedElement<'a> => |r| SupportedElement(BlockElement::from(r)));
convert!(impl<'a> From<elems::DatePicker<'a>> for SupportedElement<'a> => |r| SupportedElement(BlockElement::from(r)));

convert!(impl<'a> From<select::Static<'a>> for SupportedElement<'a> => |r| SupportedElement(BlockElement::from(r)));
convert!(impl<'a> From<select::External<'a>> for SupportedElement<'a> => |r| SupportedElement(BlockElement::from(r)));
convert!(impl<'a> From<select::User<'a>> for SupportedElement<'a> => |r| SupportedElement(BlockElement::from(r)));
convert!(impl<'a> From<select::Conversation<'a>> for SupportedElement<'a> => |r| SupportedElement(BlockElement::from(r)));
convert!(impl<'a> From<select::PublicChannel<'a>> for SupportedElement<'a> => |r| SupportedElement(BlockElement::from(r)));

convert!(impl<'a> From<select::multi::Static<'a>> for SupportedElement<'a> => |r| SupportedElement(BlockElement::from(r)));
convert!(impl<'a> From<select::multi::External<'a>> for SupportedElement<'a> => |r| SupportedElement(BlockElement::from(r)));
convert!(impl<'a> From<select::multi::User<'a>> for SupportedElement<'a> => |r| SupportedElement(BlockElement::from(r)));
convert!(impl<'a> From<select::multi::Conversation<'a>> for SupportedElement<'a> => |r| SupportedElement(BlockElement::from(r)));
convert!(impl<'a> From<select::multi::PublicChannel<'a>> for SupportedElement<'a> => |r| SupportedElement(BlockElement::from(r)));

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
