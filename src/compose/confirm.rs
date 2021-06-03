//! # Confirm Dialog
//! [slack api docs 🔗]
//!
//! An object that defines a dialog that provides a confirmation step to any interactive element.
//! This dialog will ask the user to confirm their action by offering a confirm and deny buttons.
//!
//! [slack api docs 🔗]: https://api.slack.com/reference/block-kit/composition-objects#confirm

use serde::{Deserialize, Serialize};
#[cfg(feature = "validation")]
use validator::Validate;

use crate::{text, };
#[cfg(feature = "validation")]
use crate::val_helpr::ValidationResult;

/// # Confirm Dialog
/// [slack api docs 🔗]
///
/// An object that defines a dialog that provides a confirmation step to any interactive element.
/// This dialog will ask the user to confirm their action by offering a confirm and deny buttons.
///
/// [slack api docs 🔗]: https://api.slack.com/reference/block-kit/composition-objects#confirm
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
#[cfg_attr(feature = "validation", derive(Validate))]
pub struct Confirm {
  #[cfg_attr(feature = "validation", validate(custom = "validate::title"))]
  title: text::Text,

  #[cfg_attr(feature = "validation", validate(custom = "validate::text"))]
  text: text::Text,

  #[cfg_attr(feature = "validation", validate(custom = "validate::confirm"))]
  confirm: text::Text,

  #[cfg_attr(feature = "validation", validate(custom = "validate::deny"))]
  deny: text::Text,

  #[serde(skip_serializing_if = "Option::is_none")]
  style: Option<ConfirmStyle>,
}

impl Confirm {
  /// Build a new Confirm object
  ///
  /// See ConfirmBuilder for example
  pub fn builder() -> build::ConfirmBuilderInit {
    build::ConfirmBuilderInit::new()
  }

  /// Validate that this Confirm composition object
  /// agrees with Slack's model requirements
  ///
  /// # Errors
  /// - If `title` longer than 100 chars
  /// - If `text` longer than 300 chars
  /// - If `confirm` longer than 30 chars
  /// - If `deny` longer than 30 chars
  ///
  /// # Example
  /// ```
  /// use slack_blocks::compose::{Confirm, ConfirmStyle};
  /// use slack_blocks::text;
  ///
  /// let dialog = Confirm::builder().title(
  ///         "Are you sure?",).text(
  ///         text::Mrkdwn::from("Are you _sure_ you're sure?\nThis action is permanent."),).confirm(
  ///         "I'm sure.",).deny(
  ///         "I'm not sure! Oh, geez, I just don't know! Help me decide, please??? Gosh, this is scary...",)
  ///     .style(ConfirmStyle::Danger)
  ///     .build();
  ///
  /// assert_eq!(true, matches!(dialog.validate(), Err(_)));
  /// ```
  #[cfg(feature = "validation")]
  #[cfg_attr(feature = "validation", doc(cfg(feature = "validation")))]
  pub fn validate(&self) -> ValidationResult {
    Validate::validate(self)
  }
}

/// The possible styles of the confirm button on your dialog.
#[derive(Copy, Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmStyle {
  /// Display the button with a red background on desktop,
  /// or red text on mobile.
  Danger,
  /// Display the button with a green background on desktop,
  /// or blue text on mobile.
  Primary,
}

/// Build a Confirm object
pub mod build {
  use std::marker::PhantomData;

  use super::*;
  use crate::build::*;

  /// Builder methods
  #[allow(non_camel_case_types)]
  pub mod method {
    /// ConfirmBuilder.title
    #[derive(Debug, Copy, Clone)]
    pub struct title;

    /// ConfirmBuilder.text
    #[derive(Debug, Copy, Clone)]
    pub struct text;

    /// ConfirmBuilder.confirm
    #[derive(Debug, Copy, Clone)]
    pub struct confirm;

    /// ConfirmBuilder.deny
    #[derive(Debug, Copy, Clone)]
    pub struct deny;
  }

  /// Initial state for Confirm Builder
  pub type ConfirmBuilderInit =
    ConfirmBuilder<RequiredMethodNotCalled<method::title>,
                   RequiredMethodNotCalled<method::text>,
                   RequiredMethodNotCalled<method::confirm>,
                   RequiredMethodNotCalled<method::deny>>;

  /// # Confirm Builder
  ///
  /// Allows you to construct safely, with compile-time checks
  /// on required setter methods.
  ///
  /// # Required Methods
  /// `ConfirmBuilder::build()` is only available if these methods have been called:
  ///  - `text`
  ///  - `plain` or `mrkdwn`
  ///
  /// ```
  /// use slack_blocks::compose::Confirm;
  ///
  /// let foo = Confirm::builder().title("do stuff?")
  ///                             .text_plain("stuff")
  ///                             .confirm("do the stuff")
  ///                             .deny("wait no")
  ///                             .build();
  /// ```
  #[derive(Debug)]
  pub struct ConfirmBuilder<Title, Text, Confirm, Deny> {
    title: Option<text::Text>,
    text: Option<text::Text>,
    confirm: Option<text::Text>,
    deny: Option<text::Text>,
    style: Option<ConfirmStyle>,
    state: PhantomData<(Title, Text, Confirm, Deny)>,
  }

  impl<Title, Text, Confirm, Deny> ConfirmBuilder<Title, Text, Confirm, Deny> {
    /// Construct a new confirm builder
    pub fn new() -> Self {
      Self { title: None,
             text: None,
             confirm: None,
             deny: None,
             style: None,
             state: PhantomData::<_> }
    }

    /// Set `style` (**Required**)
    ///
    /// Defines the color scheme applied to the `confirm` button.
    ///
    /// A value of `danger` will display the button with a red background on desktop, or red text on mobile.
    ///
    /// A value of `primary` will display the button with a green background on desktop, or blue text on mobile.
    ///
    /// If this field is not provided, the default value will be `primary`.
    pub fn style(mut self, style: ConfirmStyle) -> Self {
      self.style = Some(style);
      self
    }

    /// Set `title` (**Required**)
    ///
    /// A [`plain_text`-only text object 🔗] that defines the dialog's title.
    ///
    /// Maximum length for this field is 100 characters.
    ///
    /// [`plain_text`-only text object 🔗]: https://api.slack.com/reference/block-kit/composition-objects#text
    pub fn title(self,
                 t: impl Into<text::Plain>)
                 -> ConfirmBuilder<Set<method::title>, Text, Confirm, Deny>
    {
      ConfirmBuilder { text: self.text,
                       title: Some(t.into().into()),
                       confirm: self.confirm,
                       deny: self.deny,
                       style: self.style,
                       state: PhantomData::<_> }
    }

    /// Set `confirm` (**Required**)
    ///
    /// A [`plain_text`-only text object 🔗] to define
    /// the text of the button that confirms the action.
    ///
    /// Maximum length for the `text` in this field is 30 characters.
    ///
    /// [`plain_text`-only text object 🔗]: https://api.slack.com/reference/block-kit/composition-objects#text
    pub fn confirm(
      self,
      t: impl Into<text::Plain>)
      -> ConfirmBuilder<Title, Text, Set<method::confirm>, Deny> {
      ConfirmBuilder { text: self.text,
                       title: self.title,
                       confirm: Some(t.into().into()),
                       deny: self.deny,
                       style: self.style,
                       state: PhantomData::<_> }
    }

    /// Set `deny` (**Required**)
    ///
    /// A [`plain_text`-only text object 🔗] to define
    /// the text of the button that cancels the action.
    ///
    /// Maximum length for the `text` in this field is 30 characters.
    ///
    /// [`plain_text`-only text object 🔗]: https://api.slack.com/reference/block-kit/composition-objects#text
    pub fn deny(self,
                t: impl Into<text::Plain>)
                -> ConfirmBuilder<Title, Text, Confirm, Set<method::deny>> {
      ConfirmBuilder { text: self.text,
                       title: self.title,
                       confirm: self.confirm,
                       deny: Some(t.into().into()),
                       style: self.style,
                       state: PhantomData::<_> }
    }

    /// Set `text` (**Required**)
    ///
    /// A [text object 🔗] that defines the explanatory text that
    /// appears in the confirm dialog.
    ///
    /// Maximum length for the `text` in this field is 300 characters.
    ///
    /// [text object 🔗]: https://api.slack.com/reference/block-kit/composition-objects#text
    pub fn text(self,
                t: impl Into<text::Text>)
                -> ConfirmBuilder<Title, Set<method::text>, Confirm, Deny> {
      ConfirmBuilder { text: Some(t.into().into()),
                       title: self.title,
                       confirm: self.confirm,
                       deny: self.deny,
                       style: self.style,
                       state: PhantomData::<_> }
    }

    /// Set `text` to some plain text (**Required**)
    ///
    /// A [text object 🔗] that defines the explanatory text that
    /// appears in the confirm dialog.
    ///
    /// Maximum length for the `text` in this field is 300 characters.
    ///
    /// [text object 🔗]: https://api.slack.com/reference/block-kit/composition-objects#text
    pub fn text_plain(
      self,
      t: impl Into<text::Plain>)
      -> ConfirmBuilder<Title, Set<method::text>, Confirm, Deny> {
      self.text(t.into())
    }

    /// Set `text` to some markdown (**Required**)
    ///
    /// A [text object 🔗] that defines the explanatory text that
    /// appears in the confirm dialog.
    ///
    /// Maximum length for the `text` in this field is 300 characters.
    ///
    /// [text object 🔗]: https://api.slack.com/reference/block-kit/composition-objects#text
    pub fn text_md(
      self,
      t: impl Into<text::Mrkdwn>)
      -> ConfirmBuilder<Title, Set<method::text>, Confirm, Deny> {
      self.text(t.into())
    }
  }

  impl
    ConfirmBuilder<Set<method::title>,
                   Set<method::text>,
                   Set<method::confirm>,
                   Set<method::deny>>
  {
    /// All done building, now give me a darn confirm object!
    ///
    /// > `no method name 'build' found for struct 'ConfirmBuilder<...>'`?
    /// Make sure all required setter methods have been called. See docs for `ConfirmBuilder`.
    ///
    /// ```compile_fail
    /// use slack_blocks::compose::Confirm;
    ///
    /// let foo = Confirm::builder().build(); // Won't compile!
    /// ```
    ///
    /// ```
    /// use slack_blocks::compose::Confirm;
    ///
    /// let foo = Confirm::builder().title("do stuff?")
    ///                             .text_plain("stuff")
    ///                             .confirm("do the stuff")
    ///                             .deny("wait no")
    ///                             .build();
    /// ```
    pub fn build(self) -> Confirm {
      Confirm { text: self.text.unwrap(),
                title: self.title.unwrap(),
                confirm: self.confirm.unwrap(),
                deny: self.deny.unwrap(),
                style: self.style }
    }
  }
}

#[cfg(feature = "validation")]
mod validate {
  use crate::{text, val_helpr::*};

  pub(super) fn text(text: &text::Text) -> ValidatorResult {
    below_len("Confirmation Dialog text", 300, text.as_ref())
  }

  pub(super) fn title(text: &text::Text) -> ValidatorResult {
    below_len("Confirmation Dialog title", 100, text.as_ref())
  }

  pub(super) fn confirm(text: &text::Text) -> ValidatorResult {
    below_len("Confirmation Dialog confirmation text", 30, text.as_ref())
  }

  pub(super) fn deny(text: &text::Text) -> ValidatorResult {
    below_len("Confirmation Dialog deny text", 30, text.as_ref())
  }
}
