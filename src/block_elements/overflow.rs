use std::borrow::Cow;

use serde::{Deserialize as De, Serialize as Ser};
use validator::Validate;

use crate::{compose::{opt::AllowUrl, Confirm, Opt},
            text,
            val_helpr::*};

type MyOpt<'a> = Opt<'a, text::Plain, AllowUrl>;

/// # Overflow Menu
///
/// This is like a cross between a button and a select menu -
/// when a user clicks on this overflow button,
/// they will be presented with a list of options to choose from.
///
/// Unlike the select menu, there is no typeahead field,
/// and the button always appears with an ellipsis ("…"),
/// rather than customisable text.
///
/// [slack api docs 🔗]
///
/// Works in [blocks 🔗]: Section, Actions
///
/// [slack api docs 🔗]: https://api.slack.com/reference/block-kit/block-elements#overflow
/// [blocks 🔗]: https://api.slack.com/reference/block-kit/blocks
#[derive(Clone, Debug, Hash, PartialEq, Ser, De, Validate)]
pub struct Overflow<'a> {
  #[validate(length(max = 255))]
  action_id: Cow<'a, str>,

  #[validate(custom = "validate_options")]
  // TODO: validate opts contained in cow
  options: Cow<'a, [MyOpt<'a>]>,

  #[validate]
  confirm: Option<Confirm>,
}

impl<'a> Overflow<'a> {
  /// Construct a new Overflow Menu.
  ///
  /// # Example
  /// See example of `build::OverflowBuilder`
  pub fn builder() -> build::OverflowBuilderInit<'a> {
    build::OverflowBuilderInit::new()
  }

  /// Validate that this select element agrees with Slack's model requirements
  ///
  /// # Errors
  /// - length of `action_id` greater than 255
  /// - length of `options` less than 2 or greater than 5
  /// - one or more of `options` is invalid (**TODO**)
  /// - `confirm` is set and an invalid `Confirm`
  ///
  /// # Example
  /// ```
  /// use slack_blocks::{block_elements::Overflow, compose::Opt};
  ///
  /// fn repeat<T: Copy>(el: T, n: usize) -> impl Iterator<Item = T> {
  ///   std::iter::repeat(el).take(n)
  /// }
  ///
  /// let long_string: String = repeat('a', 256).collect();
  ///
  /// let opt = Opt::builder().text_plain("foo")
  ///                         .value("bar")
  ///                         .no_url()
  ///                         .build();
  ///
  /// let opts: Vec<Opt<_, _>> = repeat(&opt, 6).map(|o| o.clone()).collect();
  ///
  /// let input = Overflow::builder().action_id(long_string) // invalid
  ///                                .options(opts) // also invalid
  ///                                .build();
  ///
  /// assert!(matches!(input.validate(), Err(_)))
  /// ```
  pub fn validate(&self) -> ValidationResult {
    Validate::validate(self)
  }
}

fn validate_options<'a>(options: &Cow<'a, [MyOpt<'a>]>) -> ValidatorResult {
  len("Overflow.options", 2..=5, options.as_ref())
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

  /// Initial state for overflow builder
  pub type OverflowBuilderInit<'a> =
    OverflowBuilder<'a,
                    RequiredMethodNotCalled<method::action_id>,
                    RequiredMethodNotCalled<method::options>>;

  /// Overflow Menu Builder
  ///
  /// Allows you to construct safely, with compile-time checks
  /// on required setter methods.
  ///
  /// # Required Methods
  /// `OverflowBuilder::build()` is only available if these methods have been called:
  ///  - `action_id`
  ///  - `options`
  ///
  /// # Example
  /// ```
  /// use slack_blocks::{block_elements::Overflow, compose::Opt};
  ///
  /// Overflow::builder()
  ///          .action_id("foo")
  ///          .options(vec![
  ///            Opt::builder()
  ///                .text_plain("Open in browser")
  ///                .value("open_ext")
  ///                .url("https://foo.org")
  ///                .build(),
  ///            Opt::builder()
  ///                .text_plain("Do stuff")
  ///                .value("do_stuff")
  ///                .no_url()
  ///                .build(),
  ///          ]);
  /// ```
  pub struct OverflowBuilder<'a, A, O> {
    action_id: Option<Cow<'a, str>>,
    options: Option<Cow<'a, [MyOpt<'a>]>>,
    confirm: Option<Confirm>,
    state: PhantomData<(A, O)>,
  }

  impl<'a, A, O> OverflowBuilder<'a, A, O> {
    /// Create a new empty builder
    pub fn new() -> Self {
      Self { action_id: None,
             options: None,
             confirm: None,
             state: PhantomData::<_> }
    }

    /// Cast the internal static builder state to some other arbitrary state
    fn cast_state<A2, O2>(self) -> OverflowBuilder<'a, A2, O2> {
      OverflowBuilder { action_id: self.action_id,
                        options: self.options,
                        confirm: self.confirm,
                        state: PhantomData::<_> }
    }

    /// Set `action_id` (**Required**)
    ///
    /// An identifier for the action triggered when a menu option is selected.
    ///
    /// You can use this when you receive an interaction payload to [identify the source of the action 🔗].
    ///
    /// Should be unique among all other `action_id`s in the containing block.
    ///
    /// Maximum length for this field is 255 characters.
    ///
    /// [identify the source of the action 🔗]: https://api.slack.com/interactivity/handling#payloads
    pub fn action_id<T>(mut self,
                        action_id: T)
                        -> OverflowBuilder<'a, Set<method::action_id>, O>
      where T: Into<Cow<'a, str>>
    {
      self.action_id = Some(action_id.into());
      self.cast_state()
    }

    /// Set `options` (**Required**)
    ///
    /// An array of [option objects 🔗] to display in the menu.
    ///
    /// Maximum number of options is 5, minimum is 2.
    ///
    /// [option objects 🔗]: https://api.slack.com/reference/block-kit/composition-objects#option
    pub fn options<T>(mut self,
                      options: T)
                      -> OverflowBuilder<'a, A, Set<method::options>>
      where T: Into<Cow<'a, [MyOpt<'a>]>>
    {
      self.options = Some(options.into());
      self.cast_state()
    }

    /// Set `confirm` (Optional)
    ///
    /// A [confirm object 🔗] that defines an optional confirmation dialog that appears after a menu item is selected.
    ///
    /// [confirm object 🔗]: https://api.slack.com/reference/block-kit/composition-objects#confirm
    pub fn confirm(mut self, confirm: Confirm) -> Self {
      self.confirm = Some(confirm);
      self
    }
  }

  impl<'a> OverflowBuilder<'a, Set<method::action_id>, Set<method::options>> {
    /// All done building, now give me a darn overflow menu!
    ///
    /// > `no method name 'build' found for struct 'OverflowBuilder<...>'`?
    /// Make sure all required setter methods have been called. See docs for `OverflowBuilder`.
    ///
    /// ```compile_fail
    /// use slack_blocks::block_elements::Overflow;
    ///
    /// let foo = Overflow::builder().build(); // Won't compile!
    /// ```
    ///
    /// ```
    /// use slack_blocks::{block_elements::Overflow, compose::Opt};
    ///
    /// let foo = Overflow::builder().action_id("bar")
    ///                              .options(vec![Opt::builder().text_plain("foo")
    ///                                                          .value("bar")
    ///                                                          .no_url()
    ///                                                          .build()])
    ///                              .build();
    /// ```
    pub fn build(self) -> Overflow<'a> {
      Overflow { action_id: self.action_id.unwrap(),
                 options: self.options.unwrap(),
                 confirm: self.confirm }
    }
  }
}
