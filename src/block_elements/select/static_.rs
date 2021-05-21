use std::borrow::Cow;

use compose::{opt::NoUrl, Confirm};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{compose, text, val_helpr::ValidationResult};

type OptGroup<'a> = compose::OptGroup<'a, text::Plain, NoUrl>;
type Opt<'a> = compose::Opt<'a, text::Plain, NoUrl>;
type OptOrOptGroup<'a> = compose::OptOrOptGroup<'a, text::Plain, NoUrl>;

/// # Select menu with static options
///
/// [slack api docs 🔗](https://api.slack.com/reference/block-kit/block-elements#static_select)
///
/// This is the simplest form of select menu,
/// with a static list of options passed in when defining the element.
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize, Validate)]
pub struct Static<'a> {
  #[validate(custom = "super::validate::placeholder")]
  placeholder: text::Text,

  #[validate(length(max = 255))]
  action_id: Cow<'a, str>,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[validate(length(max = 100))]
  options: Option<Vec<Opt<'a>>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[validate(length(max = 100))]
  option_groups: Option<Vec<OptGroup<'a>>>,

  initial_option: Option<OptOrOptGroup<'a>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[validate]
  confirm: Option<Confirm>,
}

impl<'a> Static<'a> {
  /// Build a new static select element
  ///
  /// # Examples
  /// ```
  /// use std::convert::TryFrom;
  ///
  /// use slack_blocks::{block_elements::{select::Static, BlockElement},
  ///                    blocks::Actions,
  ///                    compose::Opt,
  ///                    text};
  ///
  /// struct City {
  ///   name: String,
  ///   short_code: String,
  /// }
  ///
  /// impl City {
  ///   pub fn new(name: impl ToString, short_code: impl ToString) -> Self {
  ///     Self { name: name.to_string(),
  ///            short_code: short_code.to_string() }
  ///   }
  /// }
  ///
  /// let cities = vec![City::new("Seattle", "SEA"),
  ///                   City::new("Portland", "PDX"),
  ///                   City::new("Phoenix", "PHX")];
  ///
  /// let options =
  ///   cities.iter().map(|City { name, short_code }| {
  ///                  Opt::builder().text_plain(name).value(short_code).build()
  ///                });
  ///
  /// let select: BlockElement =
  ///   Static::builder().placeholder("Choose your favorite city!")
  ///                    .action_id("fave_city")
  ///                    .options(options)
  ///                    .build()
  ///                    .into();
  ///
  /// let block = Actions::try_from(select);
  /// ```
  pub fn builder() -> build::StaticBuilderInit<'a> {
    build::StaticBuilderInit::new()
  }

  /// Validate that this select element agrees with Slack's model requirements
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
  /// let placeholder = r#"Hey I really would appreciate it if you chose
  /// a channel relatively soon, so that we can figure out
  /// where we need to send this poll, ok? it's kind of
  /// important that you specify where this poll should be
  /// sent, in case we haven't made that super clear.
  /// If you understand, could you pick a channel, already??"#;
  ///
  /// let select = select::Static::builder().placeholder(placeholder)
  ///                                       .action_id("abc123")
  ///                                       .options(std::iter::empty())
  ///                                       .build();
  ///
  /// assert!(matches!(select.validate(), Err(_)))
  /// ```
  pub fn validate(&self) -> ValidationResult {
    Validate::validate(self)
  }
}

pub mod build {
  use std::marker::PhantomData;

  use super::*;
  use crate::build::*;

  /// Type indicating whether `StaticBuilder::placeholder` has been called
  pub struct Placeholder;

  /// Type indicating whether `StaticBuilder::option_groups` has been called
  pub struct OptionGroups;

  /// Type indicating whether `StaticBuilder::options` has been called
  pub struct Options;

  /// Type indicating whether `StaticBuilder::action_id` has been called
  pub struct ActionId;

  #[allow(non_camel_case_types)]
  pub mod method {
    pub struct placeholder;
    pub struct options;
    pub struct action_id;
  }

  /// Static Select builder
  ///
  /// Allows you to construct a Static Select safely, with compile-time checks
  /// on required setter methods.
  ///
  /// # Required Methods
  /// `StaticBuilder::build()` is only available if these methods have been called:
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
  /// use slack_blocks::{block_elements::{select::Static, BlockElement},
  ///                    blocks::{Actions, Block},
  ///                    compose::Opt};
  ///
  /// let rust = Opt::from_plain_text_and_value("Rust", "rs");
  ///
  /// let select: BlockElement =
  ///   Static::builder().placeholder("Choose your favorite programming language!")
  ///                    .options(vec![rust])
  ///                    .action_id("lang_chosen")
  ///                    .build()
  ///                    .into();
  ///
  /// let block: Block =
  ///   Actions::try_from(select).expect("actions supports select elements")
  ///                            .into();
  ///
  /// // <send block to API>
  /// ```
  #[derive(Default)]
  pub struct StaticBuilder<'a, Placeholder, ActionId, Options> {
    placeholder: Option<text::Text>,
    action_id: Option<Cow<'a, str>>,
    options: Option<Vec<Opt<'a>>>,
    option_groups: Option<Vec<OptGroup<'a>>>,
    confirm: Option<Confirm>,
    initial_option: Option<OptOrOptGroup<'a>>,
    state: PhantomData<(Placeholder, ActionId, Options)>,
  }

  pub type StaticBuilderInit<'a> =
    StaticBuilder<'a,
                  RequiredMethodNotCalled<method::placeholder>,
                  RequiredMethodNotCalled<method::action_id>,
                  RequiredMethodNotCalled<method::options>>;

  // Methods that are always available
  impl<'a, P, A, O> StaticBuilder<'a, P, A, O> {
    /// Construct a new StaticBuilder
    pub fn new() -> StaticBuilderInit<'a> {
      StaticBuilderInit { placeholder: None,
                          action_id: None,
                          options: None,
                          confirm: None,
                          initial_option: None,
                          option_groups: None,
                          state: PhantomData::<_> }
    }

    /// Change the marker type params to some other arbitrary marker type params
    fn cast_state<P2, A2, O2>(self) -> StaticBuilder<'a, P2, A2, O2> {
      StaticBuilder::<'a, P2, A2, O2> { placeholder: self.placeholder,
                                        action_id: self.action_id,
                                        options: self.options,
                                        option_groups: self.option_groups,
                                        confirm: self.confirm,
                                        initial_option: self.initial_option,
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
                       -> StaticBuilder<'a, Set<Placeholder>, A, O> {
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
                     -> StaticBuilder<'a, P, Set<ActionId>, O> {
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

  impl<'a, P, A> StaticBuilder<'a, P, A, Set<Options>> {
    /// Set `initial_option` (Optional)
    ///
    /// A single option that exactly matches one of the options
    /// that `Self::options` was called with.
    ///
    /// This option will be selected when the menu initially loads.
    pub fn initial_option(mut self, option: Opt<'a>) -> Self {
      self.initial_option = Some(OptOrOptGroup::<'a>::Opt(option));
      self
    }
  }

  impl<'a, P, A> StaticBuilder<'a, P, A, Set<OptionGroups>> {
    /// Set `initial_option` (Optional)
    ///
    /// A single option group that exactly matches one of the option groups
    /// that `Self::options` was called with.
    ///
    /// This option will be selected when the menu initially loads.
    pub fn initial_option_group(mut self, option_group: OptGroup<'a>) -> Self {
      self.initial_option = Some(OptOrOptGroup::<'a>::OptGroup(option_group));
      self
    }
  }

  impl<'a, P, A, O> StaticBuilder<'a, P, A, RequiredMethodNotCalled<O>> {
    /// Set `options` (this or `Self::option_groups` is **Required**)
    ///
    /// An array of [option objects 🔗].
    /// Maximum number of options is 100.
    ///
    /// [option objects 🔗]: https://api.slack.comhttps://api.slack.com/reference/block-kit/composition-objects#option
    pub fn options<Iter>(mut self,
                         options: Iter)
                         -> StaticBuilder<'a, P, A, Set<Options>>
      where Iter: IntoIterator<Item = Opt<'a>>
    {
      self.options = Some(options.into_iter().collect::<Vec<_>>());
      self.cast_state()
    }

    /// Set `option_groups` (this or `Self::options` is **Required**)
    ///
    /// An array of [option group objects 🔗].
    /// Maximum number of option groups is 100.
    ///
    /// [option group objects 🔗]: https://api.slack.com/reference/block-kit/composition-objects#option_group
    pub fn option_groups<Iter>(mut self,
                               groups: Iter)
                               -> StaticBuilder<'a, P, A, Set<OptionGroups>>
      where Iter: IntoIterator<Item = OptGroup<'a>>
    {
      self.option_groups = Some(groups.into_iter().collect::<Vec<_>>());
      self.cast_state()
    }
  }

  impl<'a, O> StaticBuilder<'a, Set<Placeholder>, Set<ActionId>, Set<O>> {
    /// All done building, now give me a darn select element!
    ///
    /// > `no method name 'build' found for struct 'select::static_::build::StaticBuilder<...>'`?
    /// Make sure all required setter methods have been called. See docs for `StaticBuilder`.
    ///
    /// ```compile_fail
    /// use slack_blocks::block_elements::select::Static;
    ///
    /// let sel = Static::builder().build(); // Won't compile!
    /// ```
    ///
    /// ```
    /// use slack_blocks::block_elements::select::Static;
    ///
    /// let sel = Static::builder().placeholder("foo")
    ///                            .action_id("bar")
    ///                            .options(vec![])
    ///                            .build();
    /// ```
    pub fn build(self) -> Static<'a> {
      Static::<'a> { placeholder: self.placeholder.unwrap(),
                     action_id: self.action_id.unwrap(),
                     options: self.options,
                     option_groups: self.option_groups,
                     confirm: self.confirm,
                     initial_option: self.initial_option }
    }
  }
}
