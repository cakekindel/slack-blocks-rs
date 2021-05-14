use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{text, val_helpr::ValidationResult};

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
pub struct Contents {
  #[validate(custom = "validate::text")]
  text: text::Text,

  #[validate(length(max = 255))]
  action_id: String,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[validate(length(max = 3000))]
  url: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[validate(length(max = 2000))]
  value: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  style: Option<Style>,

  #[serde(skip_serializing_if = "Option::is_none")]
  confirm: Option<()>, // FIX: doesn't exist yet
}

impl Contents {
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
  /// use slack_blocks::block_elements;
  ///
  /// let btn = block_elements::Button::from_text_and_action_id("Button", "123");
  /// let actions_block: Block = actions::Contents::from_action_elements(vec![btn.into()]).into();
  /// // < send block to slack's API >
  /// ```
  pub fn from_text_and_action_id(text: impl Into<text::Plain>,
                                 action_id: impl ToString)
                                 -> Self {
    Self { text: text.into().into(),
           action_id: action_id.to_string(),
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
  /// use slack_blocks::block_elements;
  ///
  /// let btn = block_elements::Button::from_text_and_action_id("Go to cheese!", "123").with_url("https://www.cheese.com/");
  /// let actions_block: Block = actions::Contents::from_action_elements(vec![btn.into()]).into();
  /// // < send block to slack's API >
  /// ```
  pub fn with_url(mut self, url: impl ToString) -> Self {
    self.url = Some(url.to_string());
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
  /// use slack_blocks::block_elements;
  ///
  /// let btn = block_elements::Button::from_text_and_action_id("Click me!", "123")
  ///     .with_value("<something that will help your system better act on the interaction>");
  /// let actions_block: Block = actions::Contents::from_action_elements(vec![btn.into()]).into();
  /// // < send block to slack's API >
  /// ```
  pub fn with_value(mut self, value: impl ToString) -> Self {
    self.value = Some(value.to_string());
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
  /// use slack_blocks::block_elements::{Button, button::Style};
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
  /// use slack_blocks::block_elements::Button;
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
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Style {
  /// Gives buttons a green outline and text, ideal for affirmation or confirmation actions.
  /// This should only be used for one button within a set.
  Primary,
  /// Gives buttons a red outline and text, and should be used when the action is destructive.
  /// Use this even more sparingly than Primary.
  Danger,
}

mod validate {
  use crate::{text,
              val_helpr::{below_len, ValidatorResult}};

  pub fn text(text: &text::Text) -> ValidatorResult {
    below_len("Button Text", 75, text.as_ref())
  }
}
