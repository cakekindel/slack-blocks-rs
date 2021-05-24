use std::borrow::Cow;

use serde::{Deserialize as De, Serialize as Ser};
use validator::Validate;

use crate::{compose::{opt::{AnyText, NoUrl},
                      Confirm,
                      Opt},
            val_helpr::*};

type MyOpt<'a> = Opt<'a, AnyText, NoUrl>;

fn validate_options<'a>(o: &Cow<'a, [MyOpt<'a>]>) -> ValidatorResult {
  below_len("Checkboxes.options", 10, o.as_ref())
}

fn validate_initial_options<'a>(o: &Cow<'a, [MyOpt<'a>]>) -> ValidatorResult {
  below_len("Checkboxes.initial_options", 10, o.as_ref())
}

/// # Checkbox Group
///
/// A checkbox group that allows a user to choose multiple items from a list of possible options.
///
/// [slack api docs 🔗]
///
/// Works in [blocks 🔗]: Section, Actions, Input
/// Works in [app surfaces 🔗]: Home tabs, Modals, Messages
///
/// [slack api docs 🔗]: https://api.slack.com/reference/block-kit/block-elements#checkboxes
/// [blocks 🔗]: https://api.slack.com/reference/block-kit/blocks
/// [app surfaces 🔗]: https://api.slack.com/surfaces
#[derive(Clone, Debug, Hash, PartialEq, Ser, De, Validate)]
pub struct Checkboxes<'a> {
  #[validate(length(max = 255))]
  action_id: Cow<'a, str>,
  #[validate(custom = "validate_options")]
  options: Cow<'a, [MyOpt<'a>]>,
  #[validate(custom = "validate_initial_options")]
  initial_options: Option<Cow<'a, [MyOpt<'a>]>>,
  #[validate]
  confirm: Option<Confirm>,
}

impl<'a> Checkboxes<'a> {
  /// Build a new checkboxes element.
  ///
  /// # Example
  /// see example for `build::CheckboxesBuilder`.
  pub fn builder() -> build::CheckboxesBuilderInit<'a> {
    build::CheckboxesBuilderInit::new()
  }

  /// Validate that this element agrees with Slack's model requirements.
  ///
  /// # Errors
  /// - length of `action_id` greater than 255
  /// - length of `options` greater than 10
  /// - length of `initial_options` greater than 10
  /// - one or more of `options` is invalid // TODO
  /// - one or more of `initial_options` is invalid // TODO
  /// - `initial_option` is set and an invalid `Opt`
  /// - `confirm` is set and an invalid `Confirm`
  ///
  /// # Example
  /// ```
  /// use slack_blocks::{compose::Opt, elems::Checkboxes};
  ///
  /// fn repeat<T: Copy>(el: T, n: usize) -> impl Iterator<Item = T> {
  ///   std::iter::repeat(el).take(n)
  /// }
  ///
  /// let long_string: String = repeat('a', 256).collect();
  /// let opt = Opt::builder().text_md("foo").value("bar").build();
  ///
  /// let opts = repeat(&opt, 11).map(|o| o.clone()).collect::<Vec<_>>();
  ///
  /// let input = Checkboxes::builder().action_id(long_string)
  ///                                  .options(&opts)
  ///                                  .initial_options(&opts)
  ///                                  .build();
  ///
  /// assert!(matches!(input.validate(), Err(_)))
  /// ```
  pub fn validate(&self) -> ValidationResult {
    Validate::validate(self)
  }
}

pub mod build {
  use std::marker::PhantomData;

  use super::*;
  use crate::build::*;

  #[allow(non_camel_case_types)]
  pub mod method {
    pub struct action_id;
    pub struct options;
  }

  /// Initial state for Checkbox builder
  pub type CheckboxesBuilderInit<'a> =
    CheckboxesBuilder<'a,
                      RequiredMethodNotCalled<method::action_id>,
                      RequiredMethodNotCalled<method::options>>;

  /// Checkbox group builder
  ///
  /// Allows you to construct safely, with compile-time checks
  /// on required setter methods.
  ///
  /// # Required Methods
  /// `CheckboxesBuilder::build()` is only available if these methods have been called:
  ///  - `action_id`
  ///  - `options`
  ///
  /// # Example
  /// ```
  /// use std::convert::TryFrom;
  ///
  /// use slack_blocks::{blocks::{Actions, Block},
  ///                    compose::Opt,
  ///                    elems::{BlockElement, Checkboxes}};
  ///
  /// mod usa {
  ///   pub struct State {
  ///     pub name: String,
  ///     pub abbrev: String,
  ///   }
  ///
  ///   pub fn arizona() -> State {
  ///     State { name: String::from("Arizona"),
  ///             abbrev: String::from("AZ") }
  ///   }
  ///
  ///   pub fn get_states() -> Vec<State> {
  ///     // ...
  ///     # vec![]
  ///   }
  /// }
  ///
  /// let state_opt = |state: usa::State| {
  ///   Opt::builder().text_plain(state.name)
  ///                 .value(state.abbrev)
  ///                 .build()
  /// };
  ///
  /// let states: Vec<Opt<_, _>> =
  ///   usa::get_states().into_iter().map(state_opt).collect();
  ///
  /// let boxes: BlockElement =
  ///   Checkboxes::builder().action_id("state_picker")
  ///                        .options(&states)
  ///                        .initial_options(vec![state_opt(usa::arizona())])
  ///                        .build()
  ///                        .into();
  ///
  /// let block: Block = Actions::try_from(boxes).unwrap().into();
  ///
  /// // <send block to slack API>
  /// ```
  pub struct CheckboxesBuilder<'a, A, O> {
    action_id: Option<Cow<'a, str>>,
    options: Option<Cow<'a, [MyOpt<'a>]>>,
    initial_options: Option<Cow<'a, [MyOpt<'a>]>>,
    confirm: Option<Confirm>,
    state: PhantomData<(A, O)>,
  }

  impl<'a, A, O> CheckboxesBuilder<'a, A, O> {
    /// Create a new builder
    pub fn new() -> Self {
      Self { action_id: None,
             options: None,
             initial_options: None,
             confirm: None,
             state: PhantomData::<_> }
    }

    fn convert_options<I, Op>(options: I) -> Cow<'a, [MyOpt<'a>]>
      where I: Into<Cow<'a, [Op]>>,
            Op: 'a + Into<MyOpt<'a>> + Clone
    {
      // TODO: this type hell I've painted myself into requires that Opts must be
      //       owned and explicitly converted from `Opt<text::Plain>` -> `Opt<AnyText>`.
      //       If there was a better solution at the type level this wouldn't have to do
      //       this potential clone.
      options.into() // Cow<[Op]>
             .into_owned()
             .into_iter()
             .map(|o| -> MyOpt<'a> { o.into() })
             .collect()
    }

    /// Set `action_id` (Optional)
    ///
    /// An identifier for the action triggered when the checkbox group is changed.
    ///
    /// You can use this when you receive an interaction payload to [identify the source of the action 🔗].
    ///
    /// Should be unique among all other `action_id`s in the containing block.
    ///
    /// Maximum length for this field is 255 characters.
    ///
    /// [identify the source of the action 🔗]: https://api.slack.com/interactivity/handling#payloads
    pub fn action_id<S>(self,
                        action_id: S)
                        -> CheckboxesBuilder<'a, Set<method::action_id>, O>
      where S: Into<Cow<'a, str>>
    {
      CheckboxesBuilder { action_id: Some(action_id.into()),
                          options: self.options,
                          initial_options: self.initial_options,
                          confirm: self.confirm,
                          state: PhantomData::<_> }
    }

    /// Set `options` (**Required**)
    ///
    /// An array of [option objects 🔗].
    ///
    /// A maximum of 10 options are allowed.
    ///
    /// [option objects 🔗]: https://api.slack.com/reference/block-kit/composition-objects#option
    pub fn options<I, Op>(self,
                          options: I)
                          -> CheckboxesBuilder<'a, A, Set<method::options>>
      where I: Into<Cow<'a, [Op]>>,
            Op: 'a + Into<MyOpt<'a>> + Clone
    {
      CheckboxesBuilder { action_id: self.action_id,
                          options: Some(Self::convert_options(options)),
                          initial_options: self.initial_options,
                          confirm: self.confirm,
                          state: PhantomData::<_> }
    }

    /// Set `initial_options` (Optional)
    ///
    /// An array of [option objects 🔗] that exactly matches one or more
    /// of the options within `options`.
    ///
    /// These options will be selected when the checkbox group initially loads.
    ///
    /// [option objects 🔗]: https://api.slack.com/reference/messaging/composition-objects#option
    pub fn initial_options<I, Op>(mut self, options: I) -> Self
      where I: Into<Cow<'a, [Op]>>,
            Op: 'a + Into<MyOpt<'a>> + Clone
    {
      self.initial_options = Some(Self::convert_options(options));
      self
    }

    /// Set `confirm` (Optional)
    ///
    /// A [confirm object 🔗] that defines an optional confirmation dialog
    /// that appears after clicking one of the checkboxes in this element.
    ///
    /// [confirm object 🔗]: https://api.slack.com/reference/block-kit/composition-objects#confirm
    pub fn confirm(mut self, confirm: Confirm) -> Self {
      self.confirm = Some(confirm);
      self
    }
  }

  impl<'a> CheckboxesBuilder<'a, Set<method::action_id>, Set<method::options>> {
    /// All done building, now give me a darn checkbox group!
    ///
    /// > `no method name 'build' found for struct 'CheckboxesBuilder<...>'`?
    /// Make sure all required setter methods have been called. See docs for `CheckboxesBuilder`.
    ///
    /// ```compile_fail
    /// use slack_blocks::elems::Checkboxes;
    ///
    /// let foo = Checkboxes::builder().build(); // Won't compile!
    /// ```
    ///
    /// ```
    /// use slack_blocks::{compose::Opt, elems::Checkboxes};
    ///
    /// let foo = Checkboxes::builder().action_id("foo")
    ///                                .options(vec![Opt::builder().text_plain("foo")
    ///                                                            .value("bar")
    ///                                                            .build()])
    ///                                .build();
    /// ```
    pub fn build(self) -> Checkboxes<'a> {
      Checkboxes { action_id: self.action_id.unwrap(),
                   options: self.options.unwrap(),
                   initial_options: self.initial_options,
                   confirm: self.confirm }
    }
  }
}
