use std::borrow::Cow;

use super::{Conversation, External, PublicChannel, User};
use crate::{compose::Confirm, text};

/// # Select Element Builder
/// Use to construct a Select element
/// and easily choose a data source
#[derive(Debug)]
pub struct SelectBuilder<'a> {
  pub placeholder: text::Plain,
  pub action_id: Cow<'a, str>,
  pub confirm: Option<Confirm>,
}

impl<'a> SelectBuilder<'a> {
  /// Construct a Select block element from required parts
  ///
  /// # Arguments
  /// - `placeholder`: A [text object 🔗]
  ///
  /// [text objects 🔗]: https://api.slack.com/reference/messaging/composition-objects#text
  ///
  /// # Example
  /// See example for `Select::from_placeholder_and_action_id`
  pub fn from_placeholder_and_action_id(placeholder: impl Into<text::Plain>,
                                        action_id: impl Into<Cow<'a, str>>)
                                        -> Self {
    Self { placeholder: placeholder.into(),
           action_id: action_id.into(),
           confirm: None }
  }

  /// Construct a Select block element from required parts
  ///
  /// # Arguments
  /// - `confirm`: A [confirm object 🔗] that defines an optional confirmation dialog that appears after a menu item is selected.
  ///
  /// [confirm object 🔗]: https://api.slack.com/reference/block-kit/composition-objects#confirm
  ///
  /// # Example
  /// ```
  /// use std::iter;
  /// use std::convert::TryFrom;
  ///
  /// use slack_blocks::{
  ///   blocks::{Block, Actions},
  ///   elems::{BlockElement, select::Select},
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
  /// let select: BlockElement = Select::from_placeholder_and_action_id("Pick a channel to delete!", "delete_chan_select")
  ///                                   .with_confirm(confirm)
  ///                                   .choose_from_public_channels()
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

  /// Turn the builder into a Public Channel select element
  ///
  /// # Example
  /// ```
  /// use std::iter;
  /// use std::convert::TryFrom;
  ///
  /// use slack_blocks::{
  ///   blocks::{Block, Section, Actions},
  ///   elems::{BlockElement, select::Select},
  ///   compose::{text, Confirm},
  /// };
  ///
  /// # use std::error::Error;
  /// # pub fn main() -> Result<(), Box<dyn Error>> {
  ///
  /// let confirm = Confirm::from_parts("Are you sure?", text::Mrkdwn::from("Think hard about this."), "Yes", "No");
  ///
  /// let select: BlockElement = Select::from_placeholder_and_action_id("Pick a channel to delete!", "1234")
  ///                                   .with_confirm(confirm)
  ///                                   .choose_from_public_channels()
  ///                                   .into();
  ///
  /// let block: Block = Actions::try_from(select).unwrap().into();
  ///
  /// // < send `block` to slack API >
  /// # Ok(())
  /// # }
  /// ```
  pub fn choose_from_public_channels(self) -> PublicChannel<'a> {
    let sel = PublicChannel::from_placeholder_and_action_id(self.placeholder,
                                                            self.action_id);

    match self.confirm {
      | Some(confirm) => sel.with_confirm(confirm),
      | None => sel,
    }
  }

  /// Set the data source to "All conversations". See docs for `select::Conversation` for more info.
  ///
  /// # Example
  /// ```
  /// use std::iter;
  /// use std::convert::TryFrom;
  ///
  /// use slack_blocks::{
  ///   blocks::{Block, Section, Actions},
  ///   elems::{BlockElement, select::Select},
  ///   compose::{text, Confirm},
  /// };
  ///
  /// use text::ToSlackPlaintext;
  ///
  /// # use std::error::Error;
  /// # pub fn main() -> Result<(), Box<dyn Error>> {
  ///
  /// let select: BlockElement = Select::from_placeholder_and_action_id("Channel", "get_member_channel_select")
  ///                                   .choose_from_all_channels()
  ///                                   .into();
  ///
  /// let blocks: Vec<Block> = vec![
  ///   Section::from_text("Get the members of a channel".plaintext()).into(),
  ///   Actions::try_from(select).unwrap().into(),
  /// ];
  ///
  /// // <send `blocks` to slack's API>
  /// # Ok(())
  /// # }
  /// ```
  pub fn choose_from_all_channels(self) -> Conversation<'a> {
    let sel = Conversation::from_placeholder_and_action_id(self.placeholder,
                                                           self.action_id);

    match self.confirm {
      | Some(confirm) => sel.with_confirm(confirm),
      | None => sel,
    }
  }

  /// Set the data source to "users". See docs for `select::User` for more info.
  ///
  /// # Example
  /// ```
  /// use std::{convert::TryFrom, iter};
  ///
  /// use slack_blocks::{blocks::{Actions, Block, Section},
  ///                    elems::{select::Select, BlockElement},
  ///                    text,
  ///                    text::ToSlackPlaintext};
  ///
  /// let select: BlockElement =
  ///   Select::from_placeholder_and_action_id("Choose a user to ban!!",
  ///                                          "ban_chosen").choose_from_users()
  ///                                                       .into();
  ///
  /// let blocks: Block = Actions::try_from(select).unwrap().into();
  ///
  /// // <send `blocks` to slack's API>
  /// ```
  pub fn choose_from_users(self) -> User<'a> {
    let sel =
      User::from_placeholder_and_action_id(self.placeholder, self.action_id);

    match self.confirm {
      | Some(confirm) => sel.with_confirm(confirm),
      | None => sel,
    }
  }

  /// Users will choose from an external data source. See docs for `select::External` for more info.
  ///
  /// # Example
  /// ```
  /// use std::iter;
  /// use std::convert::TryFrom;
  ///
  /// use slack_blocks::{
  ///   text,
  ///   text::ToSlackPlaintext,
  ///   blocks::{Block, Section, Actions},
  ///   elems::{BlockElement, select::Select},
  /// };
  ///
  /// let select: BlockElement = Select::from_placeholder_and_action_id("Pick your favorite cheese", "cheese_chosen")
  ///                                   .choose_from_external()
  ///                                   .into();
  ///
  /// let blocks: Block = Actions::try_from(select).unwrap().into();
  ///
  /// // <send `blocks` to slack's API>
  /// ```
  pub fn choose_from_external(self) -> External<'a> {
    let sel = External::from_placeholder_and_action_id(self.placeholder,
                                                       self.action_id);

    match self.confirm {
      | Some(confirm) => sel.with_confirm(confirm),
      | None => sel,
    }
  }
}
