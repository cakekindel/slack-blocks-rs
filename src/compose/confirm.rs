//! # Confirm Dialog
//! [slack api docs 🔗]
//!
//! An object that defines a dialog that provides a confirmation step to any interactive element.
//! This dialog will ask the user to confirm their action by offering a confirm and deny buttons.
//!
//! [slack api docs 🔗]: https://api.slack.com/reference/block-kit/composition-objects#confirm

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{text, val_helpr::ValidationResult};

/// # Confirm Dialog
/// [slack api docs 🔗]
///
/// An object that defines a dialog that provides a confirmation step to any interactive element.
/// This dialog will ask the user to confirm their action by offering a confirm and deny buttons.
///
/// [slack api docs 🔗]: https://api.slack.com/reference/block-kit/composition-objects#confirm
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize, Validate)]
pub struct Confirm {
  #[validate(custom = "validate::title")]
  title: text::Text,

  #[validate(custom = "validate::text")]
  text: text::Text,

  #[validate(custom = "validate::confirm")]
  confirm: text::Text,

  #[validate(custom = "validate::deny")]
  deny: text::Text,
  style: Option<ConfirmStyle>,
}

impl Confirm {
  // TODO: refactor into builder
  /// Creates a Confirmation Dialog from the required parts.
  ///
  /// # Arguments
  ///
  /// - `title` - A [`plain_text`-only text object 🔗] that defines the dialog's title.
  ///     Maximum length for this field is 100 characters.
  ///
  /// - `text` - A [text object 🔗] that defines the explanatory text that
  ///     appears in the confirm dialog.
  ///     Maximum length for the `text` in this field is 300 characters.
  ///
  /// - `confirm` - A [`plain_text`-only text object 🔗] to define
  ///     the text of the button that confirms the action.
  ///     Maximum length for the `text` in this field is 30 characters.
  ///
  /// - `deny` - A [`plain_text`-only text object 🔗] to define
  ///     the text of the button that cancels the action.
  ///     Maximum length for the `text` in this field is 30 characters.
  ///
  /// [text object 🔗]: https://api.slack.com/reference/block-kit/composition-objects#text
  /// [`plain_text`-only text object 🔗]: https://api.slack.com/reference/block-kit/composition-objects#text
  ///
  /// # Example
  /// ```
  /// use slack_blocks::compose::Confirm;
  /// use slack_blocks::text;
  ///
  /// let dialog = Confirm::from_parts(
  ///     "Are you sure?",
  ///     text::Mrkdwn::from("Are you _sure_ you're sure?\nThis action is permanent."),
  ///     "I'm sure.",
  ///     "I'm not sure!",
  /// );
  ///
  /// // Results in a modal that looks like:
  /// //  _______________________________
  /// // |                               |
  /// // | Are you sure?                 |
  /// // |_______________________________|
  /// // |                               |
  /// // | Are you _sure_ you're sure?   |
  /// // | This action is permanent.     |
  /// // |_______________________________|
  /// // |                               |
  /// // |   |I'm not sure!| |I'm sure.| |
  /// // |_______________________________|
  /// ```
  pub fn from_parts(title: impl Into<text::Plain>,
                    text: impl Into<text::Text>,
                    confirm: impl Into<text::Plain>,
                    deny: impl Into<text::Plain>)
                    -> Self {
    Self { title: title.into().into(),
           text: text.into(),
           confirm: confirm.into().into(),
           deny: deny.into().into(),
           style: None }
  }

  /// Chainable setter method, used to set the **style** of the
  /// confirm button of your modal.
  ///
  /// # Arguments
  /// - `style` - Defines the color scheme applied to the `confirm` button.
  ///     A value of `danger` will display the button with a red background on desktop, or red text on mobile.
  ///     A value of `primary` will display the button with a green background on desktop, or blue text on mobile.
  ///     If this field is not provided, the default value will be `primary`.
  ///
  /// # Example
  /// ```
  /// use slack_blocks::compose::{Confirm, ConfirmStyle};
  /// use slack_blocks::text;
  ///
  /// let dialog = Confirm::from_parts(
  ///         "Are you sure?",
  ///         text::Mrkdwn::from("Are you _sure_ you're sure?\nThis action is permanent."),
  ///         "I'm sure.",
  ///         "I'm not sure!",
  ///     )
  ///     .with_style(ConfirmStyle::Danger);
  /// ```
  pub fn with_style(mut self, style: ConfirmStyle) -> Self {
    self.style = Some(style);
    self
  }

  /// Validate that this Confirm composition object
  /// agrees with Slack's model requirements
  ///
  /// # Errors
  /// - If `from_parts` was called with `title` longer than 100 chars
  /// - If `from_parts` was called with `text` longer than 300 chars
  /// - If `from_parts` was called with `confirm` longer than 30 chars
  /// - If `from_parts` was called with `deny` longer than 30 chars
  ///
  /// # Example
  /// ```
  /// use slack_blocks::compose::{Confirm, ConfirmStyle};
  /// use slack_blocks::text;
  ///
  /// let dialog = Confirm::from_parts(
  ///         "Are you sure?",
  ///         text::Mrkdwn::from("Are you _sure_ you're sure?\nThis action is permanent."),
  ///         "I'm sure.",
  ///         "I'm not sure! Oh, geez, I just don't know! Help me decide, please??? Gosh, this is scary...",
  ///     )
  ///     .with_style(ConfirmStyle::Danger);
  ///
  /// assert_eq!(true, matches!(dialog.validate(), Err(_)));
  /// ```
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
