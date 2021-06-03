//! # Option Group
//! [slack api docs 🔗]
//!
//! Provides a way to group options in a [select menu 🔗] or [multi-select menu 🔗].
//!
//! [select menu 🔗]: https://api.slack.com/reference/block-kit/block-elements#select
//! [multi-select menu 🔗]: https://api.slack.com/reference/block-kit/block-elements#multi_select
//! [slack api docs 🔗]: https://api.slack.com/reference/block-kit/composition-objects#option_group
//! [`plain_text` only text object 🔗]: https://api.slack.com/reference/block-kit/composition-objects#text

use serde::{Deserialize, Serialize};

#[cfg(feature = "validation")]
use validator::Validate;

use super::{opt::{AnyText, NoUrl},
            text,
            Opt};

#[cfg(feature = "validation")]
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
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
#[cfg_attr(feature = "validation", derive(Validate))]
pub struct OptGroup<'a, T = AnyText, U = NoUrl> {
  #[cfg_attr(feature = "validation", validate(custom = "validate::label"))]
  label: text::Text,

  #[cfg_attr(feature = "validation", validate(length(max = 100)))]
  options: Vec<Opt<'a, T, U>>,
}

impl<'a> OptGroup<'a> {
  /// Build a new option group composition object
  ///
  /// # Examples
  /// see example for `OptGroupBuilder`
  pub fn builder() -> build::OptGroupBuilderInit<'a> {
    build::OptGroupBuilderInit::new()
  }
}

impl<'a, T, U> OptGroup<'a, T, U> {
  /// Validate that this Option Group object
  /// agrees with Slack's model requirements
  ///
  /// # Errors
  /// - If `label` longer than 75 chars
  /// - If `opts` contains more than 100 options
  ///
  /// # Example
  /// ```
  /// use std::iter::repeat;
  ///
  /// use slack_blocks::compose::{Opt, OptGroup};
  ///
  /// let long_string: String = repeat(' ').take(76).collect();
  ///
  /// let opt = Opt::builder().text_plain("San Diego")
  ///                         .value("ca_sd")
  ///                         .build();
  /// let grp = OptGroup::builder().label(long_string).option(opt).build();
  ///
  /// assert_eq!(true, matches!(grp.validate(), Err(_)));
  /// ```
  #[cfg(feature = "validation")]
  #[cfg_attr(feature = "validation", doc(cfg(feature = "validation")))]
  pub fn validate(&self) -> ValidationResult {
    Validate::validate(self)
  }
}

/// OptGroup builder
pub mod build {
  use std::marker::PhantomData;

  use super::*;
  use crate::build::*;

  /// Required builder methods
  #[allow(non_camel_case_types)]
  mod method {
    /// OptGroupBuilder.label
    #[derive(Copy, Clone, Debug)]
    pub struct label;
    /// OptGroupBuilder.options
    #[derive(Copy, Clone, Debug)]
    pub struct options;
  }

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
  ///  use slack_blocks::{elems::{select::Static, BlockElement},
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
  ///  let select =
  ///    Static::builder().placeholder("Choose your favorite programming language!")
  ///                     .option_groups(groups)
  ///                     .action_id("lang_chosen")
  ///                     .build();
  ///
  ///  let block: Block =
  ///    Actions::builder().element(select).build()
  ///                             .into();
  ///
  ///  // <send block to API>
  ///  ```
  #[derive(Debug)]
  pub struct OptGroupBuilder<'a, T, U, Options, Label> {
    label: Option<text::Text>,
    options: Option<Vec<Opt<'a, T, U>>>,
    state: PhantomData<(Options, Label)>,
  }

  /// Initial state for OptGroupBuilder
  pub type OptGroupBuilderInit<'a> =
    OptGroupBuilder<'a,
                    AnyText,
                    NoUrl,
                    RequiredMethodNotCalled<method::options>,
                    RequiredMethodNotCalled<method::label>>;

  impl<'a, T, U, O, L> OptGroupBuilder<'a, T, U, O, L> {
    /// Construct a new OptGroupBuilder
    pub fn new() -> Self {
      Self { label: None,
             options: None,
             state: PhantomData::<_> }
    }

    fn cast_state<O2, L2>(self) -> OptGroupBuilder<'a, T, U, O2, L2> {
      OptGroupBuilder { label: self.label,
                        options: self.options,
                        state: PhantomData::<_> }
    }

    /// Set the options of this group (**Required**, or `option`)
    ///
    /// An array of [option objects 🔗] that belong to
    /// this specific group.
    ///
    /// Maximum of 100 items.
    ///
    /// [option objects 🔗]: https://api.slack.comCURRENT_PAGEoption
    pub fn options<T2, U2, I>(
      self,
      options: I)
      -> OptGroupBuilder<'a, T2, U2, Set<method::options>, L>
      where I: IntoIterator<Item = Opt<'a, T2, U2>>
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
    pub fn label<S>(mut self,
                    label: S)
                    -> OptGroupBuilder<'a, T, U, O, Set<method::label>>
      where S: Into<text::Plain>
    {
      self.label = Some(label.into().into());
      self.cast_state()
    }
  }

  // First call to `option` SETS the builder's "text kind" and "whether url was set"
  // marker type parameters
  impl<'a, T, U, L>
    OptGroupBuilder<'a, T, U, RequiredMethodNotCalled<method::options>, L>
  {
    /// Append an option to this group (**Required**, or `options`)
    ///
    /// Maximum of 100 items.
    pub fn option<T2, U2>(
      self,
      option: Opt<'a, T2, U2>)
      -> OptGroupBuilder<'a, T2, U2, Set<method::options>, L> {
      OptGroupBuilder { label: self.label,
                        options: Some(vec![option.into()]),
                        state: PhantomData::<_> }
    }

    /// XML child alias for `option`.
    #[cfg(feature = "blox")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blox")))]
    pub fn child<T2, U2>(
      self,
      option: Opt<'a, T2, U2>)
      -> OptGroupBuilder<'a, T2, U2, Set<method::options>, L> {
      self.option::<T2, U2>(option)
    }
  }

  // Subsequent calls must be the same type as the first call.
  impl<'a, T, U, L> OptGroupBuilder<'a, T, U, Set<method::options>, L> {
    /// Append an option to this group (**Required**, or `options`)
    ///
    /// Maximum of 100 items.
    pub fn option(mut self, option: Opt<'a, T, U>) -> Self {
      self.options.as_mut().unwrap().push(option);
      self
    }

    /// XML child alias for `option`.
    #[cfg(feature = "blox")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blox")))]
    pub fn child(self, option: Opt<'a, T, U>) -> Self {
      self.option(option)
    }
  }

  impl<'a, T, U>
    OptGroupBuilder<'a, T, U, Set<method::options>, Set<method::label>>
  {
    /// All done building, now give me a darn option group!
    ///
    /// > `no method name 'build' found for struct 'compose::opt_group::build::OptGroupBuilder<...>'`?
    /// Make sure all required setter methods have been called. See docs for `OptGroupBuilder`.
    ///
    /// ```compile_fail
    /// use slack_blocks::compose::OptGroup;
    ///
    /// let sel = OptGroup::builder()
    ///                    .build();
    /// /*                  ^^^^^ method not found in
    ///                    `OptGroupBuilder<'_, RequiredMethodNotCalled<options>, RequiredMethodNotCalled<value>, _>`
    /// */
    /// ```
    ///
    /// ```
    /// use slack_blocks::compose::{Opt, OptGroup};
    ///
    /// let sel = OptGroup::builder().options(vec![Opt::builder().text_plain("foo")
    ///                                                          .value("bar")
    ///                                                          .build()])
    ///                              .label("foo")
    ///                              .build();
    /// ```
    pub fn build(self) -> OptGroup<'a, T, U> {
      OptGroup { label: self.label.unwrap(),
                 options: self.options.unwrap() }
    }
  }
}

#[cfg(feature = "validation")]
mod validate {
  use super::*;
  use crate::val_helpr::{below_len, ValidatorResult};

  pub(super) fn label(text: &text::Text) -> ValidatorResult {
    below_len("Option Group Label", 75, text.as_ref())
  }
}
