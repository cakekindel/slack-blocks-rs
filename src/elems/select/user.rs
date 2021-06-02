//! # Select menu with user list
//! [slack api docs 🔗](https://api.slack.com/reference/block-kit/block-elements#users_select)
//!
//! This select menu will populate its options with a list of
//! Slack users visible to the current user in the active workspace.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};
#[cfg(feature = "validate")]
use validator::Validate;

use crate::{compose::Confirm, text};
#[cfg(feature = "validate")]
use crate::{val_helpr::ValidationResult};

/// # Select menu with user list
/// [slack api docs 🔗](https://api.slack.com/reference/block-kit/block-elements#users_select)
///
/// This select menu will populate its options with a list of
/// Slack users visible to the current user in the active workspace.
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
  #[cfg_attr(feature = "validation", derive(Validate))]
pub struct User<'a> {
  #[cfg_attr(feature = "validation", validate(custom = "super::validate::placeholder"))]
  placeholder: text::Text,

  #[cfg_attr(feature = "validation", validate(length(max = 255)))]
  action_id: Cow<'a, str>,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[cfg_attr(feature = "validation", validate)]
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
  /// let select =
  ///   select::User::builder().placeholder("Choose your favorite coworker!")
  ///                          .action_id("fave_fren")
  ///                          .build();
  ///
  /// let block: Block = Actions::builder().element(select).build().into();
  /// ```
  pub fn builder() -> build::UserBuilderInit<'a> {
    build::UserBuilderInit::new()
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
  /// let select = select::User::builder().placeholder(
  ///                           r#"Hey I really would appreciate it if you chose
  ///         a channel relatively soon, so that we can figure out
  ///         where we need to send this poll, ok? it's kind of
  ///         important that you specify where this poll should be
  ///         sent, in case we haven't made that super clear.
  ///         If you understand, could you pick a channel, already??"#,
  /// )
  ///              .action_id("ABC123")
  ///              .build();
  ///
  /// assert!(matches!(select.validate(), Err(_)))
  /// ```
  #[cfg(feature = "validation")]
  #[cfg_attr(docsrs, doc(cfg(feature = "validation")))]
  pub fn validate(&self) -> ValidationResult {
    Validate::validate(&self)
  }
}

/// User Select Builder
pub mod build {
  use std::marker::PhantomData;

  use super::*;
  use crate::{build::*,
              elems::select::{multi, select_kind}};

  /// Required builder methods
  #[allow(non_camel_case_types)]
  pub mod method {
    /// UserBuilder.placeholder
    #[derive(Copy, Clone, Debug)]
    pub struct placeholder;

    /// UserBuilder.action_id
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
  /// let select = User::builder().placeholder("Choose your favorite co-worker!")
  ///                             .action_id("favorite_coworker")
  ///                             .build();
  ///
  /// let block: Block = Actions::builder().element(select).build().into();
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

  /// Initial state for UserBuilder.
  ///
  /// Users will be able to choose one user from their workspace.
  ///
  /// To allow choosing many, use `slack_blocks::elems::select::multi::User::builder`.
  pub type UserBuilderInit<'a> =
    UserBuilder<'a,
                select_kind::Single,
                RequiredMethodNotCalled<method::placeholder>,
                RequiredMethodNotCalled<method::action_id>>;

  /// Initial state for UserBuilder.
  ///
  /// Users will be able to choose many users from their workspace.
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
