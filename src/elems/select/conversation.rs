//! # Select Conversation List
//!
//! [slack api docs 🔗](https://api.slack.com/reference/block-kit/block-elements#conversation_select)
//!
//! This select menu will populate its options with a list of public and private channels,
//! DMs, and MPIMs visible to the current user in the active workspace.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{compose::{Confirm, ConversationFilter},
            text,
            val_helpr::ValidationResult};

/// # Select Conversation List
///
/// [slack api docs 🔗](https://api.slack.com/reference/block-kit/block-elements#conversation_select)
///
/// This select menu will populate its options with a list of public and private channels,
/// DMs, and MPIMs visible to the current user in the active workspace.
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize, Validate)]
pub struct Conversation<'a> {
  #[validate(custom = "super::validate::placeholder")]
  placeholder: text::Text,

  #[validate(length(max = 255))]
  action_id: Cow<'a, str>,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[validate]
  confirm: Option<Confirm>,

  #[serde(skip_serializing_if = "Option::is_none")]
  initial_channel: Option<Cow<'a, str>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  default_to_current_conversation: Option<bool>,

  #[validate]
  #[serde(skip_serializing_if = "Option::is_none")]
  filter: Option<ConversationFilter>,
}

impl<'a> Conversation<'a> {
  /// Build a new conversation multi-select element
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
  ///   select::Conversation::builder().placeholder("Choose your favorite channel!")
  ///                                  .action_id("fave_channel")
  ///                                  .build()
  ///                                  .into();
  ///
  /// let block: Block = Actions::try_from(select).unwrap().into();
  /// ```
  pub fn builder() -> build::ConversationBuilderInit<'a> {
    build::ConversationBuilderInit::new()
  }

  /// Construct a Select element, letting users choose a DM / Group DM / Public channel from their workspace.
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
  /// let select: BlockElement = select::Conversation
  ///                                  ::from_placeholder_and_action_id("Channel", "ABC123")
  ///                                   .into();
  ///
  /// let title = "Pick a channel to send your poll to...".plaintext();
  ///
  /// let blocks: Vec<blocks::Block> = vec![
  ///     blocks::Section::from_text(title).into(),
  ///     blocks::Actions::try_from(vec![select]).unwrap().into(),
  /// ];
  ///
  /// // <send `blocks` to slack's API>
  /// ```
  #[deprecated(since = "0.16.9",
               note = "use elems::select::Conversation::builder instead.")]
  pub fn from_placeholder_and_action_id(placeholder: impl Into<text::Plain>,
                                        action_id: impl Into<Cow<'a, str>>)
                                        -> Self {
    Self { placeholder: placeholder.into().into(),
           action_id: action_id.into(),
           confirm: None,
           initial_channel: None,
           default_to_current_conversation: None,
           filter: None }
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
  /// let select: BlockElement = Select::from_placeholder_and_action_id("Pick a channel to delete!", "delete_chan_select")
  ///                                   .with_confirm(confirm)
  ///                                   .choose_from_all_channels()
  ///                                   .into();
  ///
  /// let block: Block = Actions::try_from(select).unwrap().into();
  ///
  /// // < send `block` to slack API >
  /// # Ok(())
  /// # }
  /// ```
  #[deprecated(since = "0.16.9",
               note = "use elems::select::Conversation::builder instead.")]
  pub fn with_confirm(mut self, confirm: Confirm) -> Self {
    self.confirm = Some(confirm);
    self
  }

  /// Optional method that allows you to filter the conversations
  /// available in the menu using `ConversationFilter`.
  ///
  /// # Arguments
  /// - `filter` - A [filter object 🔗] that defines an
  ///     optional confirmation dialog that appears after
  ///     a menu item is selected.
  ///
  /// [filter object 🔗]: https://api.slack.com/reference/block-kit/composition-objects#filter_conversations
  ///
  /// # Example
  /// ```
  /// use std::iter;
  /// use std::convert::TryFrom;
  ///
  /// use slack_blocks::{
  ///   blocks::{Block, Actions},
  ///   elems::{BlockElement, select::Select},
  ///   compose::{
  ///     text,
  ///     Confirm,
  ///     ConversationFilter,
  ///     conversation_filter::ConversationKind::*,
  ///     text::ToSlackPlaintext,
  ///   },
  /// };
  ///
  /// # use std::error::Error;
  /// # pub fn main() -> Result<(), Box<dyn Error>> {
  ///
  /// let filter = ConversationFilter::new().include_conversation_kinds(vec![Dm, PrivateChannel]);
  ///
  /// let select: BlockElement = Select::from_placeholder_and_action_id("Pick a channel to delete!", "delete_chan_select")
  ///                                   .choose_from_all_channels()
  ///                                   .with_filter(filter)
  ///                                   .into();
  ///
  /// let block: Block = Actions::try_from(select).unwrap().into();
  ///
  /// // < send `block` to slack API >
  /// # Ok(())
  /// # }
  /// ```
  #[deprecated(since = "0.16.9",
               note = "use elems::select::Conversation::builder instead.")]
  pub fn with_filter(mut self, filter: ConversationFilter) -> Self {
    self.filter = Some(filter);
    self
  }

  /// Optional method that allows you to set a pre-selected
  /// channel in the select menu with the channel's ID.
  ///
  /// If `default_to_current_conversation` is also supplied, `initial_channel` will take precedence.
  ///
  /// # Arguments
  /// - `channel` - The ID of any valid public channel to be
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
  /// let confirm = Confirm::from_parts(
  ///     "Confirm Poll Channel",
  ///     "Are you sure this is the channel you want to send this to?".plaintext(),
  ///     "Yep, I'm sure",
  ///     "No way!"
  /// );
  ///
  /// let general = "C12345";
  /// let select: BlockElement = Select::from_placeholder_and_action_id(
  ///                                     "Pick a Channel...",
  ///                                     "ABC123"
  ///                                   )
  ///                                   .with_confirm(confirm)
  ///                                   .choose_from_all_channels()
  ///                                   .with_initial_channel(general)
  ///                                   .into();
  ///
  /// let blocks: Vec<Block> = vec![
  ///     Section::from_text("Pick a channel to send your poll to...".plaintext()).into(),
  ///     Actions::try_from(select).unwrap().into(),
  /// ];
  ///
  /// // <send to slack's API>
  /// ```
  #[deprecated(since = "0.16.9",
               note = "use elems::select::Conversation::builder instead.")]
  pub fn with_initial_channel(mut self,
                              channel_id: impl Into<Cow<'a, str>>)
                              -> Self {
    self.initial_channel = Some(channel_id.into());
    self
  }

  /// Pre-populates the select menu with the conversation that the user was viewing when they opened the modal,
  /// if available.
  ///
  /// Default is false.
  #[deprecated(since = "0.16.9",
               note = "use elems::select::Conversation::builder instead.")]
  pub fn default_to_current_conversation(mut self) -> Self {
    self.default_to_current_conversation = Some(true);
    self
  }

  /// Undo `default_to_current_conversation`
  #[deprecated(since = "0.16.9",
               note = "use elems::select::Conversation::builder instead.")]
  pub fn no_default(mut self) -> Self {
    self.default_to_current_conversation = None; // aka false
    self
  }

  /// Validate that this conversation select agrees with Slack's model requirements
  ///
  /// # Errors
  /// - If `from_placeholder_and_action_id` was called with
  ///     `placeholder` longer than 150 chars
  /// - If `from_placeholder_and_action_id` was called with
  ///     `action_id` longer than 255 chars
  /// - If `with_confirm` was called with an invalid `Confirm` structure
  ///
  /// # Example
  /// ```
  /// use slack_blocks::elems::select;
  ///
  /// let select = select::Conversation::from_placeholder_and_action_id(
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

/// Conversation Select Builder
pub mod build {
  use std::marker::PhantomData;

  use super::*;
  use crate::{build::*,
              elems::select::{multi, select_kind}};

  /// Required builder methods
  #[allow(non_camel_case_types)]
  pub mod method {
    /// ConversationBuilder.placeholder
    #[derive(Copy, Clone, Debug)]
    pub struct placeholder;
    /// ConversationBuilder.action_id
    #[derive(Copy, Clone, Debug)]
    pub struct action_id;
    /// ConversationBuilder.initial_channel(s)
    #[derive(Copy, Clone, Debug)]
    pub struct initial_channel;
  }

  /// Conversation Select builder
  ///
  /// Allows you to construct a Conversation Select safely, with compile-time checks
  /// on required setter methods.
  ///
  /// # Required Methods
  /// `ConversationBuilder::build()` is only available if these methods have been called:
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
  ///                    elems::{select::Conversation, BlockElement}};
  ///
  /// let select: BlockElement =
  ///   Conversation::builder().placeholder("Choose your favorite channel!")
  ///                          .action_id("favorite_channel")
  ///                          .build()
  ///                          .into();
  ///
  /// let block: Block =
  ///   Actions::try_from(select).expect("actions supports select elements")
  ///                            .into();
  ///
  /// // <send block to API>
  /// ```
  #[derive(Debug)]
  pub struct ConversationBuilder<'a,
   Multi,
   Placeholder,
   ActionId,
   InitialChannel> {
    placeholder: Option<text::Text>,
    action_id: Option<Cow<'a, str>>,
    confirm: Option<Confirm>,
    filter: Option<ConversationFilter>,
    default_to_current_conversation: Option<bool>,
    initial_channel: Option<Cow<'a, str>>,
    initial_channels: Option<Cow<'a, [String]>>,
    max_selected_items: Option<u32>,
    state: PhantomData<(Multi, Placeholder, ActionId, InitialChannel)>,
  }

  /// Initial state for ConversationBuilder.
  ///
  /// Users will be able to choose one of the options.
  ///
  /// To allow choosing many, use `slack_blocks::elems::select::multi::Conversation::builder`.
  pub type ConversationBuilderInit<'a> =
    ConversationBuilder<'a,
                        select_kind::Single,
                        RequiredMethodNotCalled<method::placeholder>,
                        RequiredMethodNotCalled<method::action_id>,
                        OptionalMethodNotCalled<method::initial_channel>>;

  /// Initial state for ConversationBuilder.
  ///
  /// Users will be able to choose many options.
  pub type MultiConversationBuilderInit<'a> =
    ConversationBuilder<'a,
                        select_kind::Multi,
                        RequiredMethodNotCalled<method::placeholder>,
                        RequiredMethodNotCalled<method::action_id>,
                        OptionalMethodNotCalled<method::initial_channel>>;

  // Methods that are always available
  impl<'a, M, P, A, I> ConversationBuilder<'a, M, P, A, I> {
    /// Construct a new ConversationBuilder
    pub fn new() -> Self {
      Self { placeholder: None,
             action_id: None,
             filter: None,
             default_to_current_conversation: None,
             initial_channel: None,
             initial_channels: None,
             max_selected_items: None,
             confirm: None,
             state: PhantomData::<_> }
    }

    /// Change the marker type params to some other arbitrary marker type params
    fn cast_state<P2, A2, I2>(self) -> ConversationBuilder<'a, M, P2, A2, I2> {
      ConversationBuilder { placeholder: self.placeholder,
                            action_id: self.action_id,
                            confirm: self.confirm,
                            filter: self.filter,
                            default_to_current_conversation:
                              self.default_to_current_conversation,
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
      -> ConversationBuilder<'a, M, Set<method::placeholder>, A, I> {
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
      -> ConversationBuilder<'a, M, P, Set<method::action_id>, I> {
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

    /// Set `filter` (Optional)
    ///
    /// A [filter object 🔗] that defines an
    /// optional confirmation dialog that appears after
    /// a menu item is selected.
    ///
    /// [filter object 🔗]: https://api.slack.com/reference/block-kit/composition-objects#filter_conversations
    pub fn filter(mut self, filter: ConversationFilter) -> Self {
      self.filter = Some(filter);
      self
    }
  }

  impl<'a, M, P, A>
    ConversationBuilder<'a,
                        M,
                        P,
                        A,
                        OptionalMethodNotCalled<method::initial_channel>>
  {
    /// Corresponds to `default_to_current_conversation = true` (Optional, exclusive with `initial_channel` or `initial_channels`)
    ///
    /// Pre-populates the select menu with the conversation that the user was viewing when they opened the modal,
    /// if available.
    ///
    /// Default is false.
    pub fn initial_channel_current(
      mut self)
      -> ConversationBuilder<'a, M, P, A, Set<method::initial_channel>> {
      self.default_to_current_conversation = Some(true);
      self.cast_state()
    }
  }

  impl<'a, P, A>
    ConversationBuilder<'a,
                        select_kind::Single,
                        P,
                        A,
                        OptionalMethodNotCalled<method::initial_channel>>
  {
    /// Set `initial_channel` (Optional, exclusive with `initial_channel_current`)
    ///
    /// The ID of any valid conversation to be pre-selected when the menu loads.
    ///
    /// If `default_to_current_conversation` is called, this will take precedence.
    pub fn initial_channel<S>(
      mut self,
      channel: S)
      -> ConversationBuilder<'a,
                             select_kind::Single,
                             P,
                             A,
                             Set<method::initial_channel>>
      where S: Into<Cow<'a, str>>
    {
      self.initial_channel = Some(channel.into());
      self.cast_state()
    }
  }

  impl<'a, P, A>
    ConversationBuilder<'a,
                        select_kind::Multi,
                        P,
                        A,
                        OptionalMethodNotCalled<method::initial_channel>>
  {
    /// Set `initial_channel` (Optional, exclusive with `initial_channel_current`)
    ///
    /// A collection of IDs of any valid conversations to be pre-selected when the menu loads.
    pub fn initial_channels<S>(
      mut self,
      channels: S)
      -> ConversationBuilder<'a,
                             select_kind::Multi,
                             P,
                             A,
                             Set<method::initial_channel>>
      where S: Into<Cow<'a, [String]>>
    {
      self.initial_channels = Some(channels.into());
      self.cast_state()
    }
  }

  impl<'a, P, A, I> ConversationBuilder<'a, select_kind::Multi, P, A, I> {
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

  impl<'a, I>
    ConversationBuilder<'a,
                        select_kind::Single,
                        Set<method::placeholder>,
                        Set<method::action_id>,
                        I>
  {
    /// All done building, now give me a darn select element!
    ///
    /// > `no method name 'build' found for struct 'select::static_::build::ConversationBuilder<...>'`?
    /// Make sure all required setter methods have been called. See docs for `ConversationBuilder`.
    ///
    /// ```compile_fail
    /// use slack_blocks::elems::select::Conversation;
    ///
    /// let sel = Conversation::builder().build(); // Won't compile!
    /// ```
    ///
    /// ```
    /// use slack_blocks::elems::select::Conversation;
    ///
    /// let sel = Conversation::builder().placeholder("foo")
    ///                                  .action_id("bar")
    ///                                  .build();
    /// ```
    pub fn build(self) -> Conversation<'a> {
      Conversation { placeholder: self.placeholder.unwrap(),
                     action_id: self.action_id.unwrap(),
                     filter: self.filter,
                     default_to_current_conversation:
                       self.default_to_current_conversation,
                     confirm: self.confirm,
                     initial_channel: self.initial_channel }
    }
  }

  impl<'a, I>
    ConversationBuilder<'a,
                        select_kind::Multi,
                        Set<method::placeholder>,
                        Set<method::action_id>,
                        I>
  {
    /// All done building, now give me a darn select element!
    ///
    /// > `no method name 'build' found for struct 'select::static_::build::ConversationBuilder<...>'`?
    /// Make sure all required setter methods have been called. See docs for `ConversationBuilder`.
    ///
    /// ```compile_fail
    /// use slack_blocks::elems::select;
    ///
    /// let sel = select::multi::Conversation::builder().build(); // Won't compile!
    /// ```
    ///
    /// ```
    /// use slack_blocks::elems::select;
    ///
    /// let sel = select::multi::Conversation::builder().placeholder("foo")
    ///                                                 .action_id("bar")
    ///                                                 .build();
    /// ```
    pub fn build(self) -> multi::Conversation<'a> {
      multi::Conversation { placeholder: self.placeholder.unwrap(),
                            action_id: self.action_id.unwrap(),
                            confirm: self.confirm,
                            filter: self.filter,
                            default_to_current_conversation:
                              self.default_to_current_conversation,
                            initial_channels: self.initial_channels,
                            max_selected_items: self.max_selected_items }
    }
  }
}
