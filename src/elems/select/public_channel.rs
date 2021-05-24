use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{compose::Confirm, text, val_helpr::ValidationResult};

/// # Public Channel Select
/// [slack api docs 🔗](https://api.slack.com/reference/block-kit/block-elements#channel_select)
///
/// This select menu will populate its options with a list of
/// public channels visible to the current user in the active workspace.
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize, Validate)]
pub struct PublicChannel<'a> {
  #[validate(custom = "super::validate::placeholder")]
  placeholder: text::Text,

  #[validate(length(max = 255))]
  action_id: Cow<'a, str>,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[validate]
  confirm: Option<Confirm>,

  #[serde(skip_serializing_if = "Option::is_none")]
  initial_channel: Option<Cow<'a, str>>,
}

impl<'a> PublicChannel<'a> {
  /// Construct a Select element, with a data
  /// source of the Public Channels in the user's
  /// Workspace.
  ///
  /// # Arguments
  /// - `placeholder` - A [`plain_text` only text object 🔗] that defines
  ///     the placeholder text shown on the menu.
  ///     Maximum length for the `text` in this field is 150 characters.
  ///
  /// - `action_id` - An identifier for the action triggered when a menu option is selected.
  ///     You can use this when you receive an interaction payload to [identify the source of the action 🔗].
  ///     Should be unique among all other `action_id`s used elsewhere by your app.
  ///     Maximum length for this field is 255 characters.
  ///
  /// [`plain_text` only text object 🔗]: https://api.slack.comhttps://api.slack.com/reference/block-kit/composition-objects#text
  /// [identify the source of the action 🔗]: https://api.slack.comhttps://api.slack.com/interactivity/handling#payloads
  ///
  /// # Example
  /// ```
  /// use slack_blocks::elems::select;
  /// use slack_blocks::blocks;
  /// use slack_blocks::text;
  ///
  /// let select = select::PublicChannel::from_placeholder_and_action_id(
  ///     "Pick a Channel...",
  ///     "ABC123"
  /// );
  ///
  /// let blocks: Vec<blocks::Block> = vec![
  ///     blocks::Section::from_text(text::Plain::from(
  ///         "Pick a channel to send your poll to...")
  ///     ).into(),
  ///     blocks::Actions::from_action_elements(vec![select.into()]).into(),
  /// ];
  ///
  /// // <send to slack's API>
  /// ```
  pub fn from_placeholder_and_action_id(placeholder: impl Into<text::Plain>,
                                        action_id: impl Into<Cow<'a, str>>)
                                        -> Self {
    Self { placeholder: placeholder.into().into(),
           action_id: action_id.into(),
           confirm: None,
           initial_channel: None }
  }

  /// Optional method that allows you to add a
  /// confirmation dialog that appears after a
  /// menu item is selected.
  ///
  /// # Arguments
  /// - `confirm` - A [confirm object 🔗] that defines an
  ///     optional confirmation dialog that appears after
  ///     a menu item is selected.
  ///
  /// [confirm object 🔗]: https://api.slack.comhttps://api.slack.com/reference/block-kit/composition-objects#confirm
  ///
  /// # Example
  /// ```
  /// use slack_blocks::elems::select;
  /// use slack_blocks::text;
  /// use slack_blocks::blocks;
  /// use slack_blocks::compose;
  ///
  /// let confirm = compose::Confirm::from_parts(
  ///     "Confirm Poll Channel",
  ///     text::Plain::from(
  ///         "Are you sure this is the channel you want to send this to?"
  ///     ),
  ///     "Yep, I'm sure",
  ///     "No way!"
  /// );
  ///
  /// let select = select::PublicChannel::from_placeholder_and_action_id(
  ///         "Pick a Channel...",
  ///         "ABC123"
  ///     )
  ///     .with_confirm(confirm);
  ///
  /// let blocks: Vec<blocks::Block> = vec![
  ///     blocks::Section::from_text(text::Plain::from(
  ///         "Pick a channel to send your poll to..."
  ///     )).into(),
  ///     blocks::Actions::from_action_elements(vec![select.into()]).into(),
  /// ];
  ///
  /// // <send to slack's API>
  /// ```
  pub fn with_confirm(mut self, confirm: Confirm) -> Self {
    self.confirm = Some(confirm);
    self
  }

  /// Optional method that allows you to set a pre-selected
  /// channel in the select menu with the channel's ID.
  ///
  /// # Arguments
  /// - `channel` - The ID of any valid public channel to be
  ///     pre-selected when the menu loads.
  ///
  /// # Example
  /// ```
  /// use slack_blocks::elems::select;
  /// use slack_blocks::text;
  /// use slack_blocks::blocks;
  /// use slack_blocks::compose;
  ///
  /// let confirm = compose::Confirm::from_parts(
  ///     "Confirm Poll Channel",
  ///     text::Plain::from(
  ///         "Are you sure this is the channel you want to send this to?"
  ///     ),
  ///     "Yep, I'm sure",
  ///     "No way!"
  /// );
  /// let channel_general = ("#general", "C12345");
  /// let select = select::PublicChannel::from_placeholder_and_action_id(
  ///         "Pick a Channel...",
  ///         "ABC123"
  ///     )
  ///     .with_confirm(confirm)
  ///     .with_initial_channel(channel_general.1);
  ///
  /// let blocks: Vec<blocks::Block> = vec![
  ///     blocks::Section::from_text(text::Plain::from(
  ///         "Pick a channel to send your poll to..."
  ///     )).into(),
  ///     blocks::Actions::from_action_elements(vec![select.into()]).into(),
  /// ];
  ///
  /// // <send to slack's API>
  /// ```
  pub fn with_initial_channel(mut self,
                              channel_id: impl Into<Cow<'a, str>>)
                              -> Self {
    self.initial_channel = Some(channel_id.into());
    self
  }

  /// Validate that this Public Channel Select element
  /// agrees with Slack's model requirements
  ///
  /// # Errors
  /// - If `from_placeholder_and_action_id` was called with
  ///     `placeholder` longer than 150 chars
  /// - If `from_placeholder_and_action_id` was called with
  ///     `action_id` longer than 255 chars
  ///
  /// # Example
  /// ```
  /// use slack_blocks::elems::select;
  ///
  /// let select = select::PublicChannel::from_placeholder_and_action_id(
  ///         r#"Hey I really would appreciate it if you chose
  ///         a channel relatively soon, so that we can figure out
  ///         where we need to send this poll, ok? it's kind of
  ///         important that you specify where this poll should be
  ///         sent, in case we haven't made that super clear.
  ///         If you understand, could you pick a channel, already??"#,
  ///         "ABC123"
  ///     );
  ///
  /// assert!(matches!(select.validate(), Err(_)))
  /// ```
  pub fn validate(&self) -> ValidationResult {
    Validate::validate(&self)
  }
}
