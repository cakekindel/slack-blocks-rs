//! # Button
//! [slack api docs 🔗]
//!
//! Works with block types:
//! - [Section 🔗]
//! - [Actions 🔗]
//!
//! An interactive component that inserts a button.
//! The button can be a trigger for anything from opening
//! a simple link to starting a complex workflow.
//!
//! To use interactive components,
//! you will need to make some changes
//! to prepare your app.
//!
//! Read our [guide to enabling interactivity 🔗].
//!
//! [slack api docs 🔗]: https://api.slack.com/reference/block-kit/block-elements#button
//! [Section 🔗]: https://api.slack.com/reference/block-kit/blocks#section
//! [Actions 🔗]: https://api.slack.com/reference/block-kit/blocks#actions
//! [guide to enabling interactivity 🔗]: https://api.slack.com/interactivity/handling

use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{compose::Confirm, text, val_helpr::ValidationResult};

/// # Button
/// [slack api docs 🔗]
///
/// Works with block types:
/// - [Section 🔗]
/// - [Actions 🔗]
///
/// An interactive component that inserts a button.
/// The button can be a trigger for anything from opening
/// a simple link to starting a complex workflow.
///
/// To use interactive components,
/// you will need to make some changes
/// to prepare your app.
///
/// Read our [guide to enabling interactivity 🔗].
///
/// [slack api docs 🔗]: https://api.slack.com/reference/block-kit/block-elements#button
/// [Section 🔗]: https://api.slack.com/reference/block-kit/blocks#section
/// [Actions 🔗]: https://api.slack.com/reference/block-kit/blocks#actions
/// [guide to enabling interactivity 🔗]: https://api.slack.com/interactivity/handling
#[derive(Validate, Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Button<'a> {
  #[validate(custom = "validate::text")]
  text: text::Text,

  #[validate(length(max = 255))]
  action_id: Cow<'a, str>,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[validate(custom = "validate::url")]
  url: Option<Cow<'a, str>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[validate(custom = "validate::value")]
  value: Option<Cow<'a, str>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  style: Option<Style>,

  #[serde(skip_serializing_if = "Option::is_none")]
  confirm: Option<()>, // FIX: doesn't exist yet
}

impl<'a> Button<'a> {
  /// Create a `button::Contents` from a text label and ID for your app
  /// to be able to identify what was pressed.
  ///
  /// # Arguments
  /// - `text` - A [text object 🔗] that defines the button's text.
  ///     Can only be of type: `plain_text`.
  ///     Maximum length for the text in this field is 75 characters.
  /// - `action_id` - An identifier for this action.
  ///     You can use this when you receive an interaction payload to [identify the source of the action 🔗].
  ///     Should be unique among all other `action_id`s used elsewhere by your app.
  ///     Maximum length for this field is 255 characters.
  ///
  /// [identify the source of the action 🔗]: https://api.slack.com/interactivity/handling#payloads
  ///
  /// [text object 🔗]: https://api.slack.com/reference/block-kit/composition-objects#text
  ///
  /// # Example
  /// ```
  /// use slack_blocks::blocks::{Block, actions};
  /// use slack_blocks::elems;
  ///
  /// let btn = elems::Button::from_text_and_action_id("Button", "123");
  /// let actions_block: Block = actions::Contents::from_action_elements(vec![btn.into()]).into();
  /// // < send block to slack's API >
  /// ```
  #[deprecated(since = "0.17.3", note = "use Button::builder instead")]
  pub fn from_text_and_action_id(text: impl Into<text::Plain>,
                                 action_id: impl Into<Cow<'a, str>>)
                                 -> Self {
    Self { text: text.into().into(),
           action_id: action_id.into(),
           url: None,
           value: None,
           style: None,
           confirm: None }
  }

  /// Configure a button to be a link to an external URL to load
  /// in the user's browser on click.
  ///
  /// # Arguments
  /// - `url` - A URL to load in the user's browser when the button is clicked.
  ///     Maximum length for this field is 3000 characters.
  ///     If you're using url, you'll still receive an [interaction payload 🔗]
  ///     and will need to send an [acknowledgement response 🔗].
  ///
  /// [interaction payload 🔗]: https://api.slack.com/interactivity/handling#payloads
  /// [acknowledgement response 🔗]: https://api.slack.com/interactivity/handling#acknowledgment_response
  ///
  /// # Example
  /// ```
  /// use slack_blocks::blocks::{Block, actions};
  /// use slack_blocks::elems;
  ///
  /// let btn = elems::Button::from_text_and_action_id("Go to cheese!", "123").with_url("https://www.cheese.com/");
  /// let actions_block: Block = actions::Contents::from_action_elements(vec![btn.into()]).into();
  /// // < send block to slack's API >
  /// ```
  #[deprecated(since = "0.17.3", note = "use Button::builder instead")]
  pub fn with_url(mut self, url: impl Into<Cow<'a, str>>) -> Self {
    self.url = Some(url.into());
    self
  }

  /// Add a meaningful value to send back to your app when this button is clicked.
  ///
  /// # Arguments
  /// - `value` - The value to send along with the interaction payload.
  ///     Maximum length for this field is 2000 characters.
  ///
  /// # Example
  /// ```
  /// use slack_blocks::blocks::{Block, actions};
  /// use slack_blocks::elems;
  ///
  /// let btn = elems::Button::from_text_and_action_id("Click me!", "123")
  ///     .with_value("<something that will help your system better act on the interaction>");
  /// let actions_block: Block = actions::Contents::from_action_elements(vec![btn.into()]).into();
  /// // < send block to slack's API >
  /// ```
  #[deprecated(since = "0.17.3", note = "use Button::builder instead")]
  pub fn with_value(mut self, value: impl Into<Cow<'a, str>>) -> Self {
    self.value = Some(value.into());
    self
  }

  /// Decorates buttons with alternative visual color schemes.
  /// Use this option with restraint.
  ///
  /// If this method is not called,
  /// the default button style will be used.
  ///
  /// # Arguments
  /// - `style` - The style to decorate your button with.
  ///
  /// # Example
  /// ```
  /// use slack_blocks::blocks::{Block, actions};
  /// use slack_blocks::elems::{Button, button::Style};
  ///
  /// let confirm_btn = Button::from_text_and_action_id("Confirm!", "123")
  ///     .with_style(Style::Primary);
  ///
  /// let deny_btn = Button::from_text_and_action_id("Deny!", "123")
  ///     .with_style(Style::Danger);
  ///
  /// let actions_block: Block = actions::Contents::from_action_elements(
  ///     vec![confirm_btn.into(), deny_btn.into()]
  /// ).into();
  /// // < send block to slack's API >
  /// ```
  #[deprecated(since = "0.17.3", note = "use Button::builder instead")]
  pub fn with_style(mut self, style: Style) -> Self {
    self.style = Some(style);
    self
  }

  #[allow(dead_code)]
  fn with_confirm(_confirm: ()) -> Self {
    todo!()
  } // FIX: private until usable

  /// Validate that this Button element agrees with Slack's model requirements
  ///
  /// # Errors
  /// - If `from_text_and_action_id` was called with an action_id longer
  ///     than 255 chars
  /// - If `from_text_and_action_id` was called with text longer
  ///     than 75 chars
  /// - If `with_url` was called with url longer than 3000 chars
  /// - If `with_value` was called with url longer than 2000 chars
  ///
  /// # Example
  /// ```
  /// use slack_blocks::elems::Button;
  ///
  /// let long_string = std::iter::repeat(' ').take(256).collect::<String>();
  ///
  /// let btn = Button::from_text_and_action_id("Button", long_string);
  ///
  /// assert_eq!(true, matches!(btn.validate(), Err(_)));
  /// ```
  pub fn validate(&self) -> ValidationResult {
    Validate::validate(self)
  }
}

/// Style to optionally decorate buttons with
#[derive(Copy, Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Style {
  /// Gives buttons a green outline and text, ideal for affirmation or confirmation actions.
  /// This should only be used for one button within a set.
  Primary,
  /// Gives buttons a red outline and text, and should be used when the action is destructive.
  /// Use this even more sparingly than Primary.
  Danger,
}

/// Button builder
pub mod build {
  use std::marker::PhantomData;

  use super::*;
  use crate::build::*;

  /// Required builder methods
  #[allow(non_camel_case_types)]
  pub mod method {
    /// ButtonBuilder.text
    #[derive(Copy, Clone, Debug)]
    pub struct text;

    /// ButtonBuilder.action_id
    #[derive(Copy, Clone, Debug)]
    pub struct action_id;
  }

  /// Initial state for ButtonBuilder
  pub type ButtonBuilderInit<'a> =
    ButtonBuilder<'a,
                  RequiredMethodNotCalled<method::text>,
                  RequiredMethodNotCalled<method::action_id>>;

  /// # Button Builder
  ///
  /// Allows you to construct safely, with compile-time checks
  /// on required setter methods.
  ///
  /// # Required Methods
  /// `ButtonBuilder::build()` is only available if these methods have been called:
  ///  - `action_id`
  ///  - `text`
  ///
  /// ```
  /// use slack_blocks::{blocks, elems};
  ///
  /// let button: elems::BlockElement = elems::Button::builder().text("do stuff!")
  ///                                                           .action_id("stuff")
  ///                                                           .build();
  /// let block: block::Block =
  ///   block::Action::try_from(button).expect("Actions block supports buttons")
  ///                                  .into();
  /// ```
  #[derive(Debug)]
  pub struct ButtonBuilder<'a, Text, ActionId> {
    text: Option<text::Text>,
    action_id: Option<Cow<'a, str>>,
    url: Option<Cow<'a, str>>,
    value: Option<Cow<'a, str>>,
    style: Option<Style>,
    confirm: Option<Confirm>,
    state: PhantomData<(Text, ActionId)>,
  }

  impl<'a, T, A> ButtonBuilder<'a, T, A> {
    /// Construct a new button builder
    pub fn new() -> Self {
      Self { text: None,
             action_id: None,
             url: None,
             value: None,
             style: None,
             confirm: None,
             state: PhantomData::<_> }
    }

    /// Set `style` (Optional)
    ///
    /// Decorates buttons with alternative visual color schemes.
    ///
    /// Use this option with restraint.
    ///
    /// If this method is not called,
    /// the default button style will be used.
    pub fn style(mut self, style: Style) -> Self {
      self.style = Some(style);
      self
    }

    /// Set `confirm` (Optional)
    ///
    /// A [confirm object 🔗] that defines an optional confirmation dialog after the button is clicked.
    ///
    /// [confirm object 🔗]: https://api.slack.com/reference/block-kit/composition-objects#confirm
    pub fn confirm(mut self, confirm: Confirm) -> Self {
      self.confirm = Some(confirm);
      self
    }

    /// Set `value` (Optional)
    ///
    /// Add a meaningful value to send back to your app when this button is clicked.
    ///
    /// Maximum length for this field is 2000 characters.
    pub fn value(mut self, value: impl Into<Cow<'a, str>>) -> Self {
      self.value = Some(value.into());
      self
    }

    /// Set `action_id` (**Required**)
    ///
    /// An identifier for this action.
    ///
    /// You can use this when you receive an interaction payload to [identify the source of the action 🔗].
    ///
    /// Should be unique among all other `action_id`s used elsewhere by your app.
    ///
    /// Maximum length for this field is 255 characters.
    ///
    /// [identify the source of the action 🔗]: https://api.slack.com/interactivity/handling#payloads
    pub fn action_id(self,
                     action_id: impl Into<Cow<'a, str>>)
                     -> ButtonBuilder<'a, T, Set<method::action_id>> {
      ButtonBuilder { text: self.text,
                      action_id: Some(action_id.into()),
                      url: self.url,
                      value: self.value,
                      style: self.style,
                      confirm: self.confirm,
                      state: PhantomData::<_> }
    }

    /// Set `text` (**Required**)
    ///
    /// A plain [text object 🔗] that defines the button's text.
    ///
    /// Maximum length for the text in this field is 75 characters.
    ///
    /// [text object 🔗]: https://api.slack.com/reference/block-kit/composition-objects#text
    pub fn text(self,
                text: impl Into<text::Plain>)
                -> ButtonBuilder<'a, Set<method::text>, A> {
      ButtonBuilder { text: Some(text.into().into()),
                      action_id: self.action_id,
                      url: self.url,
                      value: self.value,
                      style: self.style,
                      confirm: self.confirm,
                      state: PhantomData::<_> }
    }
  }
}

mod validate {
  use super::*;
  use crate::{text,
              val_helpr::{below_len, ValidatorResult}};

  pub(super) fn text(text: &text::Text) -> ValidatorResult {
    below_len("Button Text", 75, text.as_ref())
  }
  pub(super) fn url(url: &Cow<str>) -> ValidatorResult {
    below_len("Button.url", 3000, url)
  }
  pub(super) fn value(value: &Cow<str>) -> ValidatorResult {
    below_len("Button.text", 2000, value)
  }
}
