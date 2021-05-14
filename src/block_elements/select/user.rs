use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{compose::Confirm, text, val_helpr::ValidationResult};

/// ## Select menu with user list
/// [slack api docs 🔗](https://api.slack.com/reference/block-kit/block-elements#users_select)
///
/// This select menu will populate its options with a list of
/// Slack users visible to the current user in the active workspace.
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize, Validate)]
pub struct User<'a> {
  #[validate(custom = "super::validate::placeholder")]
  placeholder: text::Text,

  #[validate(length(max = 255))]
  action_id: Cow<'a, str>,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[validate]
  confirm: Option<Confirm>,

  #[serde(skip_serializing_if = "Option::is_none")]
  initial_user: Option<Cow<'a, str>>,
}

impl<'a> User<'a> {
  /// Construct a Select element, letting users choose a user from their workspace.
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
  /// use std::convert::TryFrom;
  /// use std::iter;
  ///
  /// use slack_blocks::block_elements::{BlockElement, select};
  /// use slack_blocks::blocks;
  /// use slack_blocks::text;
  /// use text::ToSlackPlaintext;
  ///
  /// let select: BlockElement = select::User
  ///                                  ::from_placeholder_and_action_id("Channel", "ABC123")
  ///                                   .into();
  ///
  /// let title = "Pick a user to ban...".plaintext();
  ///
  /// let blocks: Vec<blocks::Block> = vec![
  ///     blocks::Section::from_text(title).into(),
  ///     blocks::Actions::try_from(vec![select]).unwrap().into(),
  /// ];
  ///
  /// // <send `blocks` to slack's API>
  /// ```
  pub fn from_placeholder_and_action_id(placeholder: impl Into<text::Plain>,
                                        action_id: impl Into<Cow<'a, str>>)
                                        -> Self {
    Self { placeholder: placeholder.into().into(),
           action_id: action_id.into(),
           confirm: None,
           initial_user: None }
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
  /// use std::iter;
  /// use std::convert::TryFrom;
  ///
  /// use slack_blocks::{
  ///   blocks::{Block, Actions},
  ///   block_elements::{BlockElement, select::Select},
  ///   compose::{text, Confirm, text::ToSlackPlaintext},
  /// };
  ///
  /// # use std::error::Error;
  /// # pub fn main() -> Result<(), Box<dyn Error>> {
  ///
  /// let confirm = Confirm::from_parts(
  ///   "Are you sure?",
  ///   "Think hard about this.".plaintext(),
  ///   "Yes",
  ///   "No",
  /// );
  ///
  /// let select: BlockElement = Select::from_placeholder_and_action_id("Pick a user to ban!", "ban_hammer")
  ///                                   .with_confirm(confirm)
  ///                                   .choose_from_users()
  ///                                   .into();
  ///
  /// let block: Block = Actions::try_from(select).unwrap().into();
  ///
  /// // < send `block` to slack API >
  /// # Ok(())
  /// # }
  /// ```
  pub fn with_confirm(mut self, confirm: Confirm) -> Self {
    self.confirm = Some(confirm);
    self
  }

  /// Pre-select a user
  ///
  /// # Arguments
  /// - `user_id` - The ID of any valid public channel to be
  ///     pre-selected when the menu loads.
  ///
  /// # Example
  /// ```
  /// use std::convert::TryFrom;
  ///
  /// use slack_blocks::{
  ///   blocks::{Block, Actions, Section},
  ///   block_elements::{BlockElement, select::Select},
  ///   compose::{text, Confirm, text::ToSlackPlaintext},
  /// };
  ///
  /// # let your_mom = "";
  /// let select: BlockElement = Select::from_placeholder_and_action_id(
  ///                                     "Pick a user to send a cat gif to!",
  ///                                     "cat_gif_recipient"
  ///                                   )
  ///                                   .choose_from_users()
  ///                                   .with_initial_user(your_mom)
  ///                                   .into();
  ///
  /// let blocks: Vec<Block> = vec![
  ///     Section::from_text("Pick a channel to send your poll to...".plaintext()).into(),
  ///     Actions::try_from(select).unwrap().into(),
  /// ];
  ///
  /// // <send to slack's API>
  /// ```
  pub fn with_initial_user(mut self, user_id: impl Into<Cow<'a, str>>) -> Self {
    self.initial_user = Some(user_id.into());
    self
  }

  /// Validate that this user select agrees with Slack's model requirements
  ///
  /// # Errors
  /// - If `from_placeholder_and_action_id` was called with
  ///     `placeholder` longer than 150 chars
  /// - If `from_placeholder_and_action_id` was called with
  ///     `action_id` longer than 255 chars
  ///
  /// # Example
  /// ```
  /// use slack_blocks::block_elements::select;
  ///
  /// let select = select::User::from_placeholder_and_action_id(
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
