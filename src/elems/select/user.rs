//! # Select menu with user list
//! [slack api docs 🔗](https://api.slack.com/reference/block-kit/block-elements#users_select)
//!
//! This select menu will populate its options with a list of
//! Slack users visible to the current user in the active workspace.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{compose::Confirm, text, val_helpr::ValidationResult};

/// # Select menu with user list
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
  /// Build a new user select element
  ///
  /// # Examples
  /// ```
  /// use std::convert::TryFrom;
  ///
  /// use slack_blocks::{blocks::{Actions, Block},
  ///                    compose::Opt,
  ///                    elems::{select, BlockElement},
  ///                    text};
  ///
  /// let select: BlockElement =
  ///   select::User::builder().placeholder("Choose your favorite coworker!")
  ///                          .action_id("fave_fren")
  ///                          .build()
  ///                          .into();
  ///
  /// let block: Block = Actions::try_from(select).unwrap().into();
  /// ```
  pub fn builder() -> build::UserBuilderInit<'a> {
    build::UserBuilderInit::new()
  }

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
  /// use slack_blocks::elems::{BlockElement, select};
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
  #[deprecated(since = "0.16.8", note = "use select::User::builder instead.")]
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
  #[deprecated(since = "0.16.8", note = "use select::User::builder instead.")]
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
  ///   elems::{BlockElement, select::Select},
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
  #[deprecated(since = "0.16.8", note = "use select::User::builder instead.")]
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
  /// use slack_blocks::elems::select;
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

pub mod build {
  use std::marker::PhantomData;

  use super::*;
  use crate::{build::*,
              elems::select::{multi, select_kind}};

  #[allow(non_camel_case_types)]
  pub mod method {
    #[derive(Copy, Clone, Debug)]
    pub struct placeholder;
    #[derive(Copy, Clone, Debug)]
    pub struct action_id;
  }

  /// User Select builder
  ///
  /// Allows you to construct a User Select safely, with compile-time checks
  /// on required setter methods.
  ///
  /// # Required Methods
  /// `UserBuilder::build()` is only available if these methods have been called:
  ///  - `placeholder`
  ///  - `action_id`
  ///
  /// NOTE: I'm experimenting with an API that deviates from the `from_foo_and_bar`.
  ///       If you're a user of this library, please give me feedback in the repository
  ///       as to which pattern you like more. This will most likely be the new builder pattern
  ///       for every structure in this crate.
  ///
  /// # Example
  /// ```
  /// use std::convert::TryFrom;
  ///
  /// use slack_blocks::{blocks::{Actions, Block},
  ///                    compose::Opt,
  ///                    elems::{select::User, BlockElement}};
  ///
  /// let select: BlockElement =
  ///   User::builder().placeholder("Choose your favorite co-worker!")
  ///                  .action_id("favorite_coworker")
  ///                  .build()
  ///                  .into();
  ///
  /// let block: Block =
  ///   Actions::try_from(select).expect("actions supports select elements")
  ///                            .into();
  ///
  /// // <send block to API>
  /// ```
  #[derive(Debug)]
  pub struct UserBuilder<'a, Multi, Placeholder, ActionId> {
    placeholder: Option<text::Text>,
    action_id: Option<Cow<'a, str>>,
    confirm: Option<Confirm>,
    initial_user: Option<Cow<'a, str>>,
    initial_users: Option<Cow<'a, [String]>>,
    max_selected_items: Option<u32>,
    state: PhantomData<(Multi, Placeholder, ActionId)>,
  }

  pub type UserBuilderInit<'a> =
    UserBuilder<'a,
                select_kind::Single,
                RequiredMethodNotCalled<method::placeholder>,
                RequiredMethodNotCalled<method::action_id>>;

  pub type MultiUserBuilderInit<'a> =
    UserBuilder<'a,
                select_kind::Multi,
                RequiredMethodNotCalled<method::placeholder>,
                RequiredMethodNotCalled<method::action_id>>;

  // Methods that are always available
  impl<'a, M, P, A> UserBuilder<'a, M, P, A> {
    /// Construct a new UserBuilder
    pub fn new() -> Self {
      Self { placeholder: None,
             action_id: None,
             initial_user: None,
             initial_users: None,
             max_selected_items: None,
             confirm: None,
             state: PhantomData::<_> }
    }

    /// Change the marker type params to some other arbitrary marker type params
    fn cast_state<P2, A2>(self) -> UserBuilder<'a, M, P2, A2> {
      UserBuilder { placeholder: self.placeholder,
                    action_id: self.action_id,
                    confirm: self.confirm,
                    initial_user: self.initial_user,
                    initial_users: self.initial_users,
                    max_selected_items: self.max_selected_items,
                    state: PhantomData::<_> }
    }

    /// Set `placeholder` (**Required**)
    ///
    /// A [`plain_text` only text object 🔗] that defines
    /// the placeholder text shown on the menu.
    /// Maximum length for the `text` in this field is 150 characters.
    ///
    /// [`plain_text` only text object 🔗]: https://api.slack.comhttps://api.slack.com/reference/block-kit/composition-objects#text
    pub fn placeholder(mut self,
                       text: impl Into<text::Plain>)
                       -> UserBuilder<'a, M, Set<method::placeholder>, A> {
      self.placeholder = Some(text.into().into());
      self.cast_state()
    }

    /// Set `action_id` (**Required**)
    ///
    /// An identifier for the action triggered when a menu option is selected.
    /// You can use this when you receive an interaction payload to [identify the source of the action 🔗].
    /// Should be unique among all other `action_id`s used elsewhere by your app.
    /// Maximum length for this field is 255 characters.
    ///
    /// [identify the source of the action 🔗]: https://api.slack.comhttps://api.slack.com/interactivity/handling#payloads
    pub fn action_id(mut self,
                     text: impl Into<Cow<'a, str>>)
                     -> UserBuilder<'a, M, P, Set<method::action_id>> {
      self.action_id = Some(text.into());
      self.cast_state()
    }

    /// Set `confirm` (Optional)
    ///
    /// A [confirm object 🔗] that defines an
    /// optional confirmation dialog that appears after
    /// a menu item is selected.
    ///
    /// [confirm object 🔗]: https://api.slack.comhttps://api.slack.com/reference/block-kit/composition-objects#confirm
    pub fn confirm(mut self, confirm: Confirm) -> Self {
      self.confirm = Some(confirm);
      self
    }
  }

  impl<'a, P, A> UserBuilder<'a, select_kind::Single, P, A> {
    /// Set `initial_user` (Optional)
    ///
    /// The user ID of any valid user to be pre-selected when the menu loads.
    pub fn initial_user<S>(mut self, user: S) -> Self
      where S: Into<Cow<'a, str>>
    {
      self.initial_user = Some(user.into());
      self.cast_state()
    }
  }

  impl<'a, P, A> UserBuilder<'a, select_kind::Multi, P, A> {
    /// Set `initial_users` (Optional)
    ///
    /// A collection of user IDs of any valid users to be pre-selected when the menu loads.
    pub fn initial_users<S>(mut self, users: S) -> Self
      where S: Into<Cow<'a, [String]>>
    {
      self.initial_users = Some(users.into());
      self.cast_state()
    }

    /// Set `max_selected_items` (Optional)
    ///
    /// Specifies the maximum number of items that can be selected in the menu.
    ///
    /// Minimum number is 1.
    pub fn max_selected_items(mut self, max: u32) -> Self {
      self.max_selected_items = Some(max);
      self
    }
  }

  impl<'a>
    UserBuilder<'a,
                select_kind::Single,
                Set<method::placeholder>,
                Set<method::action_id>>
  {
    /// All done building, now give me a darn select element!
    ///
    /// > `no method name 'build' found for struct 'select::static_::build::UserBuilder<...>'`?
    /// Make sure all required setter methods have been called. See docs for `UserBuilder`.
    ///
    /// ```compile_fail
    /// use slack_blocks::elems::select::User;
    ///
    /// let sel = User::builder().build(); // Won't compile!
    /// ```
    ///
    /// ```
    /// use slack_blocks::elems::select::User;
    ///
    /// let sel = User::builder().placeholder("foo").action_id("bar").build();
    /// ```
    pub fn build(self) -> User<'a> {
      User { placeholder: self.placeholder.unwrap(),
             action_id: self.action_id.unwrap(),
             confirm: self.confirm,
             initial_user: self.initial_user }
    }
  }

  impl<'a>
    UserBuilder<'a,
                select_kind::Multi,
                Set<method::placeholder>,
                Set<method::action_id>>
  {
    /// All done building, now give me a darn select element!
    ///
    /// > `no method name 'build' found for struct 'select::static_::build::UserBuilder<...>'`?
    /// Make sure all required setter methods have been called. See docs for `UserBuilder`.
    ///
    /// ```compile_fail
    /// use slack_blocks::elems::select;
    ///
    /// let sel = select::multi::User::builder().build(); // Won't compile!
    /// ```
    ///
    /// ```
    /// use slack_blocks::elems::select;
    ///
    /// let sel = select::multi::User::builder().placeholder("foo")
    ///                                         .action_id("bar")
    ///                                         .build();
    /// ```
    pub fn build(self) -> multi::User<'a> {
      multi::User { placeholder: self.placeholder.unwrap(),
                    action_id: self.action_id.unwrap(),
                    confirm: self.confirm,
                    initial_users: self.initial_users,
                    max_selected_items: self.max_selected_items }
    }
  }
}
