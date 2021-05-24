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
  /// Build a new public channel select element
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
  ///   select::PublicChannel::builder().placeholder("Choose your favorite channel!")
  ///                                  .action_id("fave_channel")
  ///                                  .build()
  ///                                  .into();
  ///
  /// let block: Block = Actions::try_from(select).unwrap().into();
  /// ```
  pub fn builder() -> build::PublicChannelBuilderInit<'a> {
    build::PublicChannelBuilderInit::new()
  }

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
  #[deprecated(since = "0.16.10",
               note = "use elems::select::PublicChannel::builder instead.")]
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
  #[deprecated(since = "0.16.10",
               note = "use elems::select::PublicChannel::builder instead.")]
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
  #[deprecated(since = "0.16.10",
               note = "use elems::select::PublicChannel::builder instead.")]
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

  /// PublicChannel Select builder
  ///
  /// Allows you to construct a PublicChannel Select safely, with compile-time checks
  /// on required setter methods.
  ///
  /// # Required Methods
  /// `PublicChannelBuilder::build()` is only available if these methods have been called:
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
  ///                    elems::{select::PublicChannel, BlockElement}};
  ///
  /// let select: BlockElement =
  ///   PublicChannel::builder().placeholder("Choose your favorite channel!")
  ///                           .action_id("favorite_channel")
  ///                           .build()
  ///                           .into();
  ///
  /// let block: Block =
  ///   Actions::try_from(select).expect("actions supports select elements")
  ///                            .into();
  ///
  /// // <send block to API>
  /// ```
  #[derive(Debug)]
  pub struct PublicChannelBuilder<'a, Multi, Placeholder, ActionId> {
    placeholder: Option<text::Text>,
    action_id: Option<Cow<'a, str>>,
    confirm: Option<Confirm>,
    initial_channel: Option<Cow<'a, str>>,
    initial_channels: Option<Cow<'a, [String]>>,
    max_selected_items: Option<u32>,
    state: PhantomData<(Multi, Placeholder, ActionId)>,
  }

  pub type PublicChannelBuilderInit<'a> =
    PublicChannelBuilder<'a,
                         select_kind::Single,
                         RequiredMethodNotCalled<method::placeholder>,
                         RequiredMethodNotCalled<method::action_id>>;

  pub type MultiPublicChannelBuilderInit<'a> =
    PublicChannelBuilder<'a,
                         select_kind::Multi,
                         RequiredMethodNotCalled<method::placeholder>,
                         RequiredMethodNotCalled<method::action_id>>;

  // Methods that are always available
  impl<'a, M, P, A> PublicChannelBuilder<'a, M, P, A> {
    /// Construct a new PublicChannelBuilder
    pub fn new() -> Self {
      Self { placeholder: None,
             action_id: None,
             initial_channel: None,
             initial_channels: None,
             max_selected_items: None,
             confirm: None,
             state: PhantomData::<_> }
    }

    /// Change the marker type params to some other arbitrary marker type params
    fn cast_state<P2, A2>(self) -> PublicChannelBuilder<'a, M, P2, A2> {
      PublicChannelBuilder { placeholder: self.placeholder,
                             action_id: self.action_id,
                             confirm: self.confirm,
                             initial_channel: self.initial_channel,
                             initial_channels: self.initial_channels,
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
    pub fn placeholder(
      mut self,
      text: impl Into<text::Plain>)
      -> PublicChannelBuilder<'a, M, Set<method::placeholder>, A> {
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
    pub fn action_id(
      mut self,
      text: impl Into<Cow<'a, str>>)
      -> PublicChannelBuilder<'a, M, P, Set<method::action_id>> {
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

  impl<'a, M, P, A> PublicChannelBuilder<'a, M, P, A> {
    /// Set `initial_channel` (Optional)
    ///
    /// The ID of any valid conversation to be pre-selected when the menu loads.
    ///
    /// If `default_to_current_conversation` is called, this will take precedence.
    pub fn initial_channel<S>(mut self, channel: S) -> Self
      where S: Into<Cow<'a, str>>
    {
      self.initial_channel = Some(channel.into());
      self
    }

    /// Set `initial_channel` (Optional, exclusive with `initial_channel_current`)
    ///
    /// A collection of IDs of any valid conversations to be pre-selected when the menu loads.
    pub fn initial_channels<S>(mut self, channels: S) -> Self
      where S: Into<Cow<'a, [String]>>
    {
      self.initial_channels = Some(channels.into());
      self.cast_state()
    }
  }

  impl<'a, P, A> PublicChannelBuilder<'a, select_kind::Multi, P, A> {
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
    PublicChannelBuilder<'a,
                         select_kind::Single,
                         Set<method::placeholder>,
                         Set<method::action_id>>
  {
    /// All done building, now give me a darn select element!
    ///
    /// > `no method name 'build' found for struct 'select::static_::build::PublicChannelBuilder<...>'`?
    /// Make sure all required setter methods have been called. See docs for `PublicChannelBuilder`.
    ///
    /// ```compile_fail
    /// use slack_blocks::elems::select::PublicChannel;
    ///
    /// let sel = PublicChannel::builder().build(); // Won't compile!
    /// ```
    ///
    /// ```
    /// use slack_blocks::elems::select::PublicChannel;
    ///
    /// let sel = PublicChannel::builder().placeholder("foo")
    ///                                   .action_id("bar")
    ///                                   .build();
    /// ```
    pub fn build(self) -> PublicChannel<'a> {
      PublicChannel { placeholder: self.placeholder.unwrap(),
                      action_id: self.action_id.unwrap(),
                      confirm: self.confirm,
                      initial_channel: self.initial_channel }
    }
  }

  impl<'a>
    PublicChannelBuilder<'a,
                         select_kind::Multi,
                         Set<method::placeholder>,
                         Set<method::action_id>>
  {
    /// All done building, now give me a darn select element!
    ///
    /// > `no method name 'build' found for struct 'select::static_::build::PublicChannelBuilder<...>'`?
    /// Make sure all required setter methods have been called. See docs for `PublicChannelBuilder`.
    ///
    /// ```compile_fail
    /// use slack_blocks::elems::select;
    ///
    /// let sel = select::multi::PublicChannel::builder().build(); // Won't compile!
    /// ```
    ///
    /// ```
    /// use slack_blocks::elems::select;
    ///
    /// let sel = select::multi::PublicChannel::builder().placeholder("foo")
    ///                                                  .action_id("bar")
    ///                                                  .build();
    /// ```
    pub fn build(self) -> multi::PublicChannel<'a> {
      multi::PublicChannel { placeholder: self.placeholder.unwrap(),
                             action_id: self.action_id.unwrap(),
                             confirm: self.confirm,
                             initial_channels: self.initial_channels,
                             max_selected_items: self.max_selected_items }
    }
  }
}
