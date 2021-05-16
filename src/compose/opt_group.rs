use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{text, Opt};
use crate::val_helpr::ValidationResult;

/// # Option Group
/// [slack api docs 🔗]
///
/// Provides a way to group options in a [select menu 🔗] or [multi-select menu 🔗].
///
/// [select menu 🔗]: https://api.slack.com/reference/block-kit/block-elements#select
/// [multi-select menu 🔗]: https://api.slack.com/reference/block-kit/block-elements#multi_select
/// [slack api docs 🔗]: https://api.slack.com/reference/block-kit/composition-objects#option_group
/// [`plain_text` only text object 🔗]: https://api.slack.com/reference/block-kit/composition-objects#text
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize, Validate)]
pub struct OptGroup<'a, M = ()> {
  #[validate(custom = "validate::label")]
  label: text::Text,

  #[validate(length(max = 100))]
  options: Vec<Opt<'a, M>>,
}

impl<'a> OptGroup<'a> {
  /// Build a new option group composition object
  ///
  /// # Examples
  ///  ```
  ///  use std::convert::TryFrom;
  ///
  ///  use slack_blocks::{block_elements::{select::Static, BlockElement},
  ///                     blocks::{Actions, Block},
  ///                     compose::{Opt, OptGroup}};
  ///
  ///  #[derive(Clone, Copy, PartialEq)]
  ///  enum LangStyle {
  ///    Functional,
  ///    ObjectOriented,
  ///    SomewhereInbetween,
  ///  }
  ///
  ///  use LangStyle::*;
  ///
  ///  #[derive(Clone, Copy)]
  ///  struct Lang {
  ///    name: &'static str,
  ///    code: &'static str,
  ///    style: LangStyle,
  ///  }
  ///
  ///  impl Lang {
  ///    fn new(name: &'static str, code: &'static str, style: LangStyle) -> Self {
  ///      Self {
  ///        name,
  ///        code,
  ///        style,
  ///      }
  ///    }
  ///  }
  ///
  ///  let langs = vec![
  ///    Lang::new("Rust", "rs", SomewhereInbetween),
  ///    Lang::new("C#", "cs", ObjectOriented),
  ///    Lang::new("Haskell", "hs", Functional),
  ///  ];
  ///
  ///  let langs_of_style = |needle: LangStyle| langs.iter()
  ///                                                .filter(|Lang {style, ..}| *style == needle)
  ///                                                .map(|lang| Opt::builder()
  ///                                                                .text_plain(lang.name)
  ///                                                                .value(lang.code)
  ///                                                                .build()
  ///                                                )
  ///                                                .collect::<Vec<_>>();
  ///
  ///  let groups = vec![
  ///    OptGroup::builder()
  ///             .label("Functional")
  ///             .options(langs_of_style(Functional))
  ///             .build(),
  ///
  ///    OptGroup::builder()
  ///             .label("Object-Oriented")
  ///             .options(langs_of_style(ObjectOriented))
  ///             .build(),
  ///
  ///    OptGroup::builder()
  ///             .label("Somewhere Inbetween")
  ///             .options(langs_of_style(SomewhereInbetween))
  ///             .build(),
  ///  ];
  ///
  ///  let select: BlockElement =
  ///    Static::builder().placeholder("Choose your favorite programming language!")
  ///                     .option_groups(groups)
  ///                     .action_id("lang_chosen")
  ///                     .build()
  ///                     .into();
  ///
  ///  let block: Block =
  ///    Actions::try_from(select).expect("actions supports select elements")
  ///                             .into();
  ///
  ///  // <send block to API>
  ///  ```
  pub fn builder() -> build::OptGroupBuilder<'a> {
    build::OptGroupBuilder::new()
  }

  /// Construct an Option Group from a label and
  /// collection of options in the group
  ///
  /// # Arguments
  /// - `label` - A [`plain_text` only text object 🔗] that defines
  ///     the label shown above this group of options.
  ///     Maximum length for the `text` in this field is 75 characters.
  /// - `opts` - An array of [option objects 🔗] that belong to
  ///     this specific group. Maximum of 100 items.
  ///
  /// [option objects 🔗]: https://api.slack.comCURRENT_PAGEoption
  /// [`plain_text` only text object 🔗]: https://api.slack.comCURRENT_PAGEtext
  ///
  /// # Example
  /// ```
  /// use slack_blocks::blocks::Block;
  /// use slack_blocks::blocks::section::Contents as Section;
  /// use slack_blocks::blocks::actions::Contents as Actions;
  /// use slack_blocks::text::{Mrkdwn};
  /// use slack_blocks::compose::{OptGroup, Opt};
  ///
  /// let prompt = "Choose your favorite city from each state!";
  ///
  /// let blocks: Vec<Block> = vec![
  ///     Section::from_text(Mrkdwn::from(prompt)).into(),
  ///     // TODO: insert option group once block elements are in place
  ///     Actions::from_action_elements(vec![]).into(),
  /// ];
  ///
  /// let groups: Vec<OptGroup<_>> = vec![
  ///     OptGroup::from_label_and_opts(
  ///         "Arizona",
  ///         vec![
  ///             Opt::from_mrkdwn_and_value("Phoenix", "az_phx"),
  ///             // etc...
  ///         ]
  ///     ),
  ///     OptGroup::from_label_and_opts(
  ///         "California",
  ///         vec![
  ///             Opt::from_mrkdwn_and_value("San Diego", "ca_sd"),
  ///             // etc...
  ///         ]
  ///     ),
  /// ];
  /// ```
  #[deprecated(since = "0.15.0", note = "Use OptGroup::builder instead")]
  pub fn from_label_and_opts<M>(label: impl Into<text::Plain>,
                                options: impl IntoIterator<Item = Opt<'a, M>>)
                                -> OptGroup<'a, M> {
    OptGroup::<'a, M> { label: label.into().into(),
                        options: options.into_iter().collect() }
  }
}

impl<'a, M> OptGroup<'a, M> {
  /// Validate that this Option Group object
  /// agrees with Slack's model requirements
  ///
  /// # Errors
  /// - If `from_label_and_opts` was called with `label`
  ///     longer than 75 chars
  /// - If `from_label_and_opts` was called with
  ///     more than 100 options
  ///
  /// # Example
  /// ```
  /// use std::iter::repeat;
  ///
  /// use slack_blocks::compose::{Opt, OptGroup};
  ///
  /// let long_string: String = repeat(' ').take(76).collect();
  ///
  /// let opt = Opt::from_mrkdwn_and_value("San Diego", "ca_sd");
  /// let grp = OptGroup::from_label_and_opts(long_string, vec![opt]);
  ///
  /// assert_eq!(true, matches!(grp.validate(), Err(_)));
  /// ```
  pub fn validate(&self) -> ValidationResult {
    Validate::validate(self)
  }
}

pub mod build {
  use std::marker::PhantomData;

  use super::*;
  use crate::build::*;

  pub struct Label;

  ///  Option Group builder
  ///
  ///  Allows you to construct a Option Group object safely, with compile-time checks
  ///  on required setter methods.
  ///
  ///  # Required Methods
  ///  `OptGroup::build()` is only available if these methods have been called:
  ///   - `options`
  ///   - `label`
  ///
  ///  # Example
  ///  ```
  ///  use std::convert::TryFrom;
  ///
  ///  use slack_blocks::{block_elements::{select::Static, BlockElement},
  ///                     blocks::{Actions, Block},
  ///                     compose::{Opt, OptGroup}};
  ///
  ///  #[derive(Clone, Copy, PartialEq)]
  ///  enum LangStyle {
  ///    Functional,
  ///    ObjectOriented,
  ///    SomewhereInbetween,
  ///  }
  ///
  ///  use LangStyle::*;
  ///
  ///  #[derive(Clone, Copy)]
  ///  struct Lang {
  ///    name: &'static str,
  ///    code: &'static str,
  ///    style: LangStyle,
  ///  }
  ///
  ///  impl Lang {
  ///    fn new(name: &'static str, code: &'static str, style: LangStyle) -> Self {
  ///      Self {
  ///        name,
  ///        code,
  ///        style,
  ///      }
  ///    }
  ///  }
  ///
  ///  let langs = vec![
  ///    Lang::new("Rust", "rs", SomewhereInbetween),
  ///    Lang::new("C#", "cs", ObjectOriented),
  ///    Lang::new("Haskell", "hs", Functional),
  ///  ];
  ///
  ///  let langs_of_style = |needle: LangStyle| langs.iter()
  ///                                                .filter(|Lang {style, ..}| *style == needle)
  ///                                                .map(|lang| Opt::builder()
  ///                                                                .text_plain(lang.name)
  ///                                                                .value(lang.code)
  ///                                                                .build()
  ///                                                )
  ///                                                .collect::<Vec<_>>();
  ///
  ///  let groups = vec![
  ///    OptGroup::builder()
  ///             .label("Functional")
  ///             .options(langs_of_style(Functional))
  ///             .build(),
  ///
  ///    OptGroup::builder()
  ///             .label("Object-Oriented")
  ///             .options(langs_of_style(ObjectOriented))
  ///             .build(),
  ///
  ///    OptGroup::builder()
  ///             .label("Somewhere Inbetween")
  ///             .options(langs_of_style(SomewhereInbetween))
  ///             .build(),
  ///  ];
  ///
  ///  let select: BlockElement =
  ///    Static::builder().placeholder("Choose your favorite programming language!")
  ///                     .option_groups(groups)
  ///                     .action_id("lang_chosen")
  ///                     .build()
  ///                     .into();
  ///
  ///  let block: Block =
  ///    Actions::try_from(select).expect("actions supports select elements")
  ///                             .into();
  ///
  ///  // <send block to API>
  ///  ```
  pub struct OptGroupBuilder<'a,
   M = (),
   O = Unset<Opt<'a, M>>,
   L = Unset<Label>> {
    label: Option<text::Text>,
    options: Option<Vec<Opt<'a, M>>>,
    state: PhantomData<(O, L)>,
  }

  impl<'a> OptGroupBuilder<'a> {
    /// Construct a new OptGroupBuilder
    pub fn new() -> Self {
      Self { label: None,
             options: None,
             state: PhantomData::<_> }
    }
  }

  impl<'a, M, O, L> OptGroupBuilder<'a, M, O, L> {
    fn cast_state<O2, L2>(self) -> OptGroupBuilder<'a, M, O2, L2> {
      OptGroupBuilder { label: self.label,
                        options: self.options,
                        state: PhantomData::<_> }
    }

    /// Set the options of this group (**Required**)
    ///
    /// An array of [option objects 🔗] that belong to
    /// this specific group.
    ///
    /// Maximum of 100 items.
    ///
    /// [option objects 🔗]: https://api.slack.comCURRENT_PAGEoption
    pub fn options<M2, I>(self,
                          options: I)
                          -> OptGroupBuilder<'a, M2, Set<Opt<'a, M2>>, L>
      where I: IntoIterator<Item = Opt<'a, M2>>
    {
      OptGroupBuilder { label: self.label,
                        options: Some(options.into_iter().collect()),
                        state: PhantomData::<_> }
    }

    /// A [`plain_text` only text object 🔗] that defines
    /// the label shown above this group of options.
    ///
    /// Maximum length for the `text` in this field is 75 characters.
    ///
    /// [`plain_text` only text object 🔗]: https://api.slack.comCURRENT_PAGEtext
    pub fn label<S>(mut self, label: S) -> OptGroupBuilder<'a, M, O, Set<Label>>
      where S: Into<text::Plain>
    {
      self.label = Some(label.into().into());
      self.cast_state()
    }
  }

  impl<'a, M> OptGroupBuilder<'a, M, Set<Opt<'a, M>>, Set<Label>> {
    /// All done building, now give me a darn option group!
    ///
    /// > `no method name 'build' found for struct 'compose::opt_group::build::OptGroupBuilder<...>'`?
    /// Make sure all required setter methods have been called. See docs for `OptGroupBuilder`.
    ///
    /// ```compile_fail
    /// use slack_blocks::compose::OptGroup;
    ///
    /// let sel = OptGroup::builder().build(); // Won't compile!
    /// ```
    ///
    /// ```
    /// use slack_blocks::compose::OptGroup;
    ///
    /// let sel = OptGroup::builder().options::<(), _>(vec![])
    ///                              .label("foo")
    ///                              .build();
    /// ```
    pub fn build(self) -> OptGroup<'a, M> {
      OptGroup { label: self.label.unwrap(),
                 options: self.options.unwrap() }
    }
  }
}

mod validate {
  use super::*;
  use crate::val_helpr::{below_len, ValidatorResult};

  pub fn label(text: &text::Text) -> ValidatorResult {
    below_len("Option Group Label", 75, text.as_ref())
  }
}
