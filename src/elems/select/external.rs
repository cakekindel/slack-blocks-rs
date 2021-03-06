//! # Select menu with external data source
//! [slack api docs 🔗]
//!
//! This select menu will load its options from an external data source,
//! allowing for a dynamic list of options.
//!
//! ## Setup
//! [Slack API doc guide for setting up an external data source 🔗](https://api.slack.com/reference/block-kit/block-elements#external_select__setup)
//!
//! [slack api docs 🔗]: https://api.slack.com/reference/block-kit/block-elements#external_select

use std::borrow::Cow;

use serde::{Deserialize, Serialize};
#[cfg(feature = "validation")]
use validator::Validate;

#[cfg(feature = "validation")]
use crate::val_helpr::ValidationResult;
use crate::{compose,
            compose::{opt::NoUrl, Confirm},
            text};

type Opt<'a> = compose::Opt<'a, text::Plain, NoUrl>;
type OptGroup<'a> = compose::OptGroup<'a, text::Plain, NoUrl>;
type OptOrOptGroup<'a> = compose::OptOrOptGroup<'a, text::Plain, NoUrl>;

/// # Select menu with external data source
/// [slack api docs 🔗]
///
/// This select menu will load its options from an external data source,
/// allowing for a dynamic list of options.
///
/// ## Setup
/// [Slack API doc guide for setting up an external data source 🔗](https://api.slack.com/reference/block-kit/block-elements#external_select__setup)
///
/// [slack api docs 🔗]: https://api.slack.com/reference/block-kit/block-elements#external_select
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
#[cfg_attr(feature = "validation", derive(Validate))]
pub struct External<'a> {
  #[cfg_attr(feature = "validation",
             validate(custom = "super::validate::placeholder"))]
  placeholder: text::Text,

  #[cfg_attr(feature = "validation", validate(length(max = 255)))]
  action_id: Cow<'a, str>,

  #[serde(skip_serializing_if = "Option::is_none")]
  initial_option: Option<OptOrOptGroup<'a>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  min_query_length: Option<u64>,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[cfg_attr(feature = "validation", validate)]
  confirm: Option<Confirm>,
}

impl<'a> External<'a> {
  /// Build a new external select element
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
  ///   select::External::builder().placeholder("Choose your favorite city!")
  ///                              .action_id("fave_city")
  ///                              .build();
  ///
  /// let block: Block = Actions::builder().element(select).build().into();
  /// ```
  pub fn builder() -> build::ExternalBuilderInit<'a> {
    build::ExternalBuilderInit::new()
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
  /// let select = select::External::builder().placeholder(
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
    Validate::validate(self)
  }
}

/// External Select Builder
pub mod build {
  use std::marker::PhantomData;

  use super::*;
  use crate::{build::*,
              elems::select::{multi, select_kind}};

  /// Required builder methods
  #[allow(non_camel_case_types)]
  pub mod method {
    /// PublicChannelBuilder.placeholder
    #[derive(Copy, Clone, Debug)]
    pub struct placeholder;
    /// PublicChannelBuilder.initial_option(s) or initial_option_group(s)
    #[derive(Copy, Clone, Debug)]
    pub struct initial_option;
    /// PublicChannelBuilder.action_id
    #[derive(Copy, Clone, Debug)]
    pub struct action_id;
  }

  /// External Select builder
  ///
  /// Allows you to construct a External Select safely, with compile-time checks
  /// on required setter methods.
  ///
  /// # Required Methods
  /// `ExternalBuilder::build()` is only available if these methods have been called:
  ///  - `placeholder`
  ///  - `action_id`
  ///  - `options` or `option_groups`
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
  /// use slack_blocks::{elems::{select::External, BlockElement},
  ///                    blocks::{Actions, Block},
  ///                    compose::Opt};
  ///
  /// let select =
  ///   External::builder().placeholder("Choose your favorite programming language!")
  ///                    .action_id("lang_chosen")
  ///                    .build();
  ///
  /// let block: Block =
  ///   Actions::builder().element(select).build().into();
  ///
  /// // <send block to API>
  /// ```
  #[derive(Debug)]
  pub struct ExternalBuilder<'a, Multi, Placeholder, ActionId, Options> {
    placeholder: Option<text::Text>,
    action_id: Option<Cow<'a, str>>,
    confirm: Option<Confirm>,
    initial_option: Option<OptOrOptGroup<'a>>,
    initial_options: Option<Cow<'a, [OptOrOptGroup<'a>]>>,
    max_selected_items: Option<u32>,
    min_query_length: Option<u64>,
    state: PhantomData<(Multi, Placeholder, ActionId, Options)>,
  }

  /// Initial state for ExternalBuilder.
  ///
  /// Users will be able to choose one of the options.
  ///
  /// To allow choosing many, use `slack_blocks::elems::select::multi::External::builder`.
  pub type ExternalBuilderInit<'a> =
    ExternalBuilder<'a,
                    select_kind::Single,
                    RequiredMethodNotCalled<method::placeholder>,
                    RequiredMethodNotCalled<method::action_id>,
                    OptionalMethodNotCalled<method::initial_option>>;

  /// Initial state for ExternalBuilder.
  ///
  /// Users will be able to choose many options.
  pub type MultiExternalBuilderInit<'a> =
    ExternalBuilder<'a,
                    select_kind::Multi,
                    RequiredMethodNotCalled<method::placeholder>,
                    RequiredMethodNotCalled<method::action_id>,
                    OptionalMethodNotCalled<method::initial_option>>;

  // Methods that are always available
  impl<'a, M, P, A, O> ExternalBuilder<'a, M, P, A, O> {
    /// Construct a new ExternalBuilder
    pub fn new() -> Self {
      Self { placeholder: None,
             action_id: None,
             initial_option: None,
             initial_options: None,
             max_selected_items: None,
             confirm: None,
             min_query_length: None,
             state: PhantomData::<_> }
    }

    /// Change the marker type params to some other arbitrary marker type params
    fn cast_state<P2, A2, O2>(self) -> ExternalBuilder<'a, M, P2, A2, O2> {
      ExternalBuilder { placeholder: self.placeholder,
                        action_id: self.action_id,
                        confirm: self.confirm,
                        initial_option: self.initial_option,
                        initial_options: self.initial_options,
                        max_selected_items: self.max_selected_items,
                        min_query_length: self.min_query_length,
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
      -> ExternalBuilder<'a, M, Set<method::placeholder>, A, O> {
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
      -> ExternalBuilder<'a, M, P, Set<method::action_id>, O> {
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

    /// Set `min_query_length` (Optional)
    ///
    /// When the typeahead field is used, a request will be sent on every character change.
    ///
    /// If you prefer fewer requests or more fully ideated queries,
    /// use the `min_query_length` attribute to tell Slack the fewest number
    /// of typed characters required before dispatch.
    ///
    /// The default value is `3`.
    pub fn min_query_length(mut self, min: u64) -> Self {
      self.min_query_length = Some(min);
      self
    }
  }

  impl<'a, P, A, O> ExternalBuilder<'a, select_kind::Multi, P, A, O> {
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
  impl<'a, P, A>
    ExternalBuilder<'a,
                    select_kind::Multi,
                    P,
                    A,
                    OptionalMethodNotCalled<method::initial_option>>
  {
    /// Set `initial_options` (Optional)
    ///
    /// An array of [option objects 🔗] that exactly match one or more of the options loaded from the external data source.
    ///
    /// These options will be selected when the menu initially loads.
    ///
    /// [option objects 🔗]: https://api.slack.com/reference/messaging/composition-objects#option
    pub fn initial_options<I>(
      mut self,
      options: I)
      -> ExternalBuilder<'a,
                         select_kind::Multi,
                         P,
                         A,
                         Set<method::initial_option>>
      where I: IntoIterator<Item = Opt<'a>>
    {
      self.initial_options = Some(options.into_iter()
                                         .map(|o| OptOrOptGroup::<'a>::Opt(o))
                                         .collect());
      self.cast_state()
    }

    /// Set `initial_options` (Optional)
    ///
    /// An array of [option objects 🔗] that exactly match one or more of the options loaded from the external data source.
    ///
    /// These options will be selected when the menu initially loads.
    ///
    /// [option objects 🔗]: https://api.slack.com/reference/messaging/composition-objects#option
    pub fn initial_option_groups<I>(
      mut self,
      option_groups: I)
      -> ExternalBuilder<'a,
                         select_kind::Multi,
                         P,
                         A,
                         Set<method::initial_option>>
      where I: IntoIterator<Item = OptGroup<'a>>
    {
      self.initial_options =
        Some(option_groups.into_iter()
                          .map(|o| OptOrOptGroup::<'a>::OptGroup(o))
                          .collect());
      self.cast_state()
    }
  }

  impl<'a, P, A>
    ExternalBuilder<'a,
                    select_kind::Single,
                    P,
                    A,
                    OptionalMethodNotCalled<method::initial_option>>
  {
    /// Set `initial_option` (Optional)
    ///
    /// A single option that exactly matches one of the options
    /// loaded from the external data source.
    ///
    /// This option will be selected when the menu initially loads.
    pub fn initial_option(
      mut self,
      option: Opt<'a>)
      -> ExternalBuilder<'a,
                         select_kind::Single,
                         P,
                         A,
                         Set<method::initial_option>> {
      self.initial_option = Some(OptOrOptGroup::<'a>::Opt(option));
      self.cast_state()
    }

    /// Set `initial_option` (Optional)
    ///
    /// A single option group that exactly matches one of the option groups
    /// loaded from the external data source.
    ///
    /// This option will be selected when the menu initially loads.
    pub fn initial_option_group(
      mut self,
      option_group: OptGroup<'a>)
      -> ExternalBuilder<'a,
                         select_kind::Single,
                         P,
                         A,
                         Set<method::initial_option>> {
      self.initial_option = Some(OptOrOptGroup::<'a>::OptGroup(option_group));
      self.cast_state()
    }
  }

  impl<'a, O>
    ExternalBuilder<'a,
                    select_kind::Single,
                    Set<method::placeholder>,
                    Set<method::action_id>,
                    O>
  {
    /// All done building, now give me a darn select element!
    ///
    /// > `no method name 'build' found for struct 'select::static_::build::ExternalBuilder<...>'`?
    /// Make sure all required setter methods have been called. See docs for `ExternalBuilder`.
    ///
    /// ```compile_fail
    /// use slack_blocks::elems::select::External;
    ///
    /// let sel = External::builder().build(); // Won't compile!
    /// ```
    ///
    /// ```
    /// use slack_blocks::elems::select::External;
    ///
    /// let sel = External::builder().placeholder("foo")
    ///                              .action_id("bar")
    ///                              .build();
    /// ```
    pub fn build(self) -> External<'a> {
      External { placeholder: self.placeholder.unwrap(),
                 action_id: self.action_id.unwrap(),
                 confirm: self.confirm,
                 initial_option: self.initial_option,
                 min_query_length: self.min_query_length }
    }
  }

  impl<'a, O>
    ExternalBuilder<'a,
                    select_kind::Multi,
                    Set<method::placeholder>,
                    Set<method::action_id>,
                    O>
  {
    /// All done building, now give me a darn select element!
    ///
    /// > `no method name 'build' found for struct 'select::static_::build::ExternalBuilder<...>'`?
    /// Make sure all required setter methods have been called. See docs for `ExternalBuilder`.
    ///
    /// ```compile_fail
    /// use slack_blocks::elems::select::External;
    ///
    /// let sel = External::builder().build(); // Won't compile!
    /// ```
    ///
    /// ```
    /// use slack_blocks::elems::select::External;
    ///
    /// let sel = External::builder().placeholder("foo")
    ///                              .action_id("bar")
    ///                              .build();
    /// ```
    pub fn build(self) -> multi::External<'a> {
      multi::External { placeholder: self.placeholder.unwrap(),
                        action_id: self.action_id.unwrap(),
                        confirm: self.confirm,
                        initial_options: self.initial_options,
                        min_query_length: self.min_query_length,
                        max_selected_items: self.max_selected_items }
    }
  }
}
