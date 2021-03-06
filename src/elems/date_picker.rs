//! # Date Picker Element
//!
//! An element which lets users easily select a date from a calendar style UI.
//!
//! [slack api docs 🔗]
//!
//! Works in [blocks 🔗]: Section, Actions, Input
//!
//! [slack api docs 🔗]: https://api.slack.com/reference/block-kit/block-elements#datepicker
//! [blocks 🔗]: https://api.slack.com/reference/block-kit/blocks

use std::borrow::Cow;

use serde::{Deserialize as De, Serialize as Ser};
#[cfg(feature = "validation")]
use validator::Validate;

#[cfg(feature = "validation")]
use crate::val_helpr::*;
use crate::{compose::Confirm, text};

/// # Date Picker Element
///
/// An element which lets users easily select a date from a calendar style UI.
///
/// [slack api docs 🔗]
///
/// Works in [blocks 🔗]: Section, Actions, Input
///
/// [slack api docs 🔗]: https://api.slack.com/reference/block-kit/block-elements#datepicker
/// [blocks 🔗]: https://api.slack.com/reference/block-kit/blocks
#[derive(Clone, Debug, Hash, PartialEq, Ser, De)]
#[cfg_attr(feature = "validation", derive(Validate))]
pub struct DatePicker<'a> {
  #[cfg_attr(feature = "validation", validate(length(max = 255)))]
  action_id: Cow<'a, str>,

  #[cfg_attr(feature = "validation",
             validate(custom = "validate_placeholder"))]
  #[serde(skip_serializing_if = "Option::is_none")]
  placeholder: Option<text::Text>,

  #[serde(skip_serializing_if = "Option::is_none")]
  initial_date: Option<String>,

  #[cfg_attr(feature = "validation", validate)]
  #[serde(skip_serializing_if = "Option::is_none")]
  confirm: Option<Confirm>,
}

#[cfg(feature = "validation")]
fn validate_placeholder(p: &text::Text) -> ValidatorResult {
  below_len("DatePicker.placeholder", 150, p)
}

impl<'a> DatePicker<'a> {
  /// Build a new Date picker element.
  ///
  /// # Example
  /// see example for `build::DatePickerBuilder`.
  pub fn builder() -> build::DatePickerBuilderInit<'a> {
    build::DatePickerBuilderInit::new()
  }

  /// Validate that this image element agrees with Slack's model requirements.
  ///
  /// No rules are specified in the Slack docs at the time of writing so this will always succeed.
  #[cfg(feature = "validation")]
  #[cfg_attr(docsrs, doc(cfg(feature = "validation")))]
  pub fn validate(&self) -> ValidationResult {
    Validate::validate(self)
  }
}

/// Date picker builder
pub mod build {
  use std::marker::PhantomData;

  use super::*;
  use crate::build::*;

  /// Required builder methods
  #[allow(non_camel_case_types)]
  pub mod method {
    /// DatePickerBuilder.action_id
    #[derive(Copy, Clone, Debug)]
    pub struct action_id;
  }

  /// Initial state for Date picker
  pub type DatePickerBuilderInit<'a> =
    DatePickerBuilder<'a, RequiredMethodNotCalled<method::action_id>>;

  /// Date Picker builder
  ///
  /// Allows you to construct safely, with compile-time checks
  /// on required setter methods.
  ///
  /// # Required Methods
  /// `DatePickerBuilder::build()` is only available if these methods have been called:
  ///  - `action_id`
  ///
  /// # Example
  /// ```
  /// use std::convert::TryFrom;
  ///
  /// use slack_blocks::{blocks::{Actions, Block},
  ///                    elems::{BlockElement, DatePicker}};
  ///
  /// let picker = DatePicker::builder().action_id("foo").build();
  ///
  /// let block: Block = Actions::builder().element(picker).build().into();
  ///
  /// // <send block to slack API>
  /// ```
  #[derive(Debug)]
  pub struct DatePickerBuilder<'a, A> {
    action_id: Option<Cow<'a, str>>,
    placeholder: Option<text::Text>,
    initial_date: Option<String>,
    confirm: Option<Confirm>,
    state: PhantomData<A>,
  }

  impl<'a, A> DatePickerBuilder<'a, A> {
    /// Create a new builder
    pub fn new() -> Self {
      Self { action_id: None,
             placeholder: None,
             initial_date: None,
             confirm: None,
             state: PhantomData::<_> }
    }

    /// Set `action_id` (Optional)
    ///
    /// An identifier for the action triggered when a menu option is selected.
    ///
    /// You can use this when you receive an interaction payload to [identify the source of the action 🔗].
    ///
    /// Should be unique among all other `action_id`s in the containing block.
    ///
    /// Maximum length for this field is 255 characters.
    ///
    /// [identify the source of the action 🔗]: https://api.slack.com/interactivity/handling#payloads
    pub fn action_id<S>(self,
                        action_id: S)
                        -> DatePickerBuilder<'a, Set<method::action_id>>
      where S: Into<Cow<'a, str>>
    {
      DatePickerBuilder { action_id: Some(action_id.into()),
                          placeholder: self.placeholder,
                          initial_date: self.initial_date,
                          confirm: self.confirm,
                          state: PhantomData::<_> }
    }

    /// Set `placeholder` (Optional)
    ///
    /// A [`plain_text` only text object 🔗] that defines the placeholder text shown on the datepicker.
    ///
    /// Maximum length for the `text` in this field is 150 characters.
    ///
    /// [`plain_text` only text object 🔗]: https://api.slack.com/reference/block-kit/composition-objects#text
    pub fn placeholder<S>(mut self, placeholder: S) -> Self
      where S: Into<text::Plain>
    {
      self.placeholder = Some(placeholder.into().into());
      self
    }

    /// Set `initial_date` (Optional)
    ///
    /// The initial date that is selected when the element is loaded.
    ///
    /// ```
    /// use slack_blocks::elems::DatePicker;
    ///
    /// DatePicker::builder().action_id("foo")
    ///                      .initial_date((01, 05, 2021))
    ///                      .build();
    /// ```
    pub fn initial_date(mut self, (day, month, year): (u8, u8, u16)) -> Self {
      self.initial_date = Some(format!("{:02}-{:02}-{}", year, month, day));
      self
    }

    /// Set `confirm` (Optional)
    ///
    /// A [confirm object 🔗] that defines an optional confirmation dialog
    /// that appears after a date is selected.
    ///
    /// [confirm object 🔗]: https://api.slack.com/reference/block-kit/composition-objects#confirm
    pub fn confirm(mut self, confirm: Confirm) -> Self {
      self.confirm = Some(confirm);
      self
    }
  }

  impl<'a> DatePickerBuilder<'a, Set<method::action_id>> {
    /// All done building, now give me a darn date picker!
    ///
    /// > `no method name 'build' found for struct 'DatePickerBuilder<...>'`?
    /// Make sure all required setter methods have been called. See docs for `DatePickerBuilder`.
    ///
    /// ```compile_fail
    /// use slack_blocks::elems::DatePicker;
    ///
    /// let foo = DatePicker::builder().build(); // Won't compile!
    /// ```
    ///
    /// ```
    /// use slack_blocks::{compose::Opt, elems::DatePicker};
    ///
    /// let foo = DatePicker::builder().action_id("foo").build();
    /// ```
    pub fn build(self) -> DatePicker<'a> {
      DatePicker { action_id: self.action_id.unwrap(),
                   placeholder: self.placeholder,
                   initial_date: self.initial_date,
                   confirm: self.confirm }
    }
  }
}
