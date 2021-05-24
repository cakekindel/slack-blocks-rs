use std::borrow::Cow;

use serde::{Deserialize as De, Serialize as Ser};
use validator::Validate;

use crate::{compose::{opt::{AnyText, NoUrl},
                      Confirm,
                      Opt},
            text,
            val_helpr::ValidationResult};

pub type MyOpt<'a> = Opt<'a, AnyText, NoUrl>;

/// # Radio Buttons
///
/// A radio button group that allows a user to choose one item from a list of possible options.
///
/// [slack api docs 🔗]
///
/// Works in [blocks 🔗]: Section, Actions, Input
/// Works in [app surfaces 🔗]: Home tabs, Modals, Messages
///
/// [slack api docs 🔗]: https://api.slack.com/reference/block-kit/block-elements#radio
/// [blocks 🔗]: https://api.slack.com/reference/block-kit/blocks
/// [app surfaces 🔗]: https://api.slack.com/surfaces
#[derive(Clone, Debug, Hash, PartialEq, Ser, De, Validate)]
pub struct Radio<'a> {
  #[validate(length(max = 255))]
  action_id: Cow<'a, str>, // max 255

  #[validate(length(max = 10))]
  #[validate]
  options: Vec<MyOpt<'a>>, // max 10, plain or md

  #[serde(skip_serializing_if = "Option::is_none")]
  #[validate]
  initial_option: Option<MyOpt<'a>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[validate]
  confirm: Option<Confirm>,
}

impl<'a> Radio<'a> {
  /// Build a new Radio Button Group
  ///
  /// # Example
  /// See docs for `RadioBuilder`.
  pub fn builder() -> build::RadioBuilderInit<'a> {
    build::RadioBuilderInit::new()
  }

  /// Validate that this select element agrees with Slack's model requirements
  ///
  /// # Errors
  /// - length of `action_id` greater than 255
  /// - length of `options` greater than 10
  /// - one or more of `options` is invalid
  /// - `initial_option` is set and an invalid `Opt`
  /// - `confirm` is set and an invalid `Confirm`
  ///
  /// # Example
  /// ```
  /// use slack_blocks::{compose::Opt, elems::Radio};
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
  /// let input = Radio::builder().action_id(long_string)
  ///                             .options(opts)
  ///                             .build();
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
  mod method {
    pub struct action_id;
    pub struct options;
  }

  pub type RadioBuilderInit<'a> =
    RadioBuilder<'a,
                 AnyText,
                 RequiredMethodNotCalled<method::action_id>,
                 RequiredMethodNotCalled<method::options>>;

  /// Radio Button builder
  ///
  /// Allows you to construct a radio button safely, with compile-time checks
  /// on required setter methods.
  ///
  /// # Required Methods
  /// `RadioButton::build()` is only available if these methods have been called:
  ///  - `options`
  ///  - `action_id`
  ///
  /// # Example
  /// ```
  /// use slack_blocks::{blocks::{Actions, Block},
  ///                    compose::Opt,
  ///                    elems::Radio};
  ///
  /// let options = vec![Opt::builder().text_md(":joy:").value("joy").build(),
  ///                    Opt::builder().text_md(":smirk:").value("smirk").build(),];
  ///
  /// let radio = Radio::builder().options(options)
  ///                             .action_id("emoji_picker")
  ///                             .build();
  ///
  /// let block: Block =
  ///   Actions::from_action_elements(std::iter::once(radio.into())).into();
  ///
  /// // <send block to slack API>
  /// ```
  pub struct RadioBuilder<'a, T, A, O> {
    action_id: Option<Cow<'a, str>>,
    options: Option<Vec<MyOpt<'a>>>,
    initial_option: Option<MyOpt<'a>>,
    confirm: Option<Confirm>,
    state: PhantomData<(T, A, O)>,
  }

  impl<'a, T, A, O> RadioBuilder<'a, T, A, O> {
    /// Construct a new RadioBuilder
    pub fn new() -> Self {
      Self { action_id: None,
             options: None,
             initial_option: None,
             confirm: None,
             state: PhantomData::<_> }
    }

    fn cast_state<A2, O2>(self) -> RadioBuilder<'a, T, A2, O2> {
      RadioBuilder { action_id: self.action_id,
                     options: self.options,
                     initial_option: self.initial_option,
                     confirm: self.confirm,
                     state: PhantomData::<_> }
    }

    /// Sets `action_id` (**Required**)
    ///
    /// An identifier for the action triggered when the radio button group is changed.
    ///
    /// You can use this when you receive an interaction payload to [identify the source of the action 🔗].
    ///
    /// Should be unique among all other `action_id`s in the containing block.
    /// Maximum length for this field is 255 characters.
    /// [identify the source of the action 🔗]: https://api.slack.com/interactivity/handling#payloads
    pub fn action_id<S>(mut self,
                        action_id: S)
                        -> RadioBuilder<'a, T, Set<method::action_id>, O>
      where S: Into<Cow<'a, str>>
    {
      self.action_id = Some(action_id.into());
      self.cast_state()
    }

    /// Sets `options` (**Required**)
    ///
    /// An array of [option objects 🔗].
    ///
    /// A maximum of 10 options are allowed.
    /// [option objects 🔗]: https://api.slack.com/reference/block-kit/composition-objects#option
    pub fn options<I, T2: Into<text::Text>>(
      self,
      options: I)
      -> RadioBuilder<'a, T2, A, Set<method::options>>
      where I: IntoIterator<Item = Opt<'a, T2, NoUrl>>
    {
      let options = options.into_iter().map(|o| o.into()).collect();

      RadioBuilder { action_id: self.action_id,
                     options: Some(options),
                     initial_option: self.initial_option,
                     confirm: self.confirm,
                     state: PhantomData::<_> }
    }

    /// Sets `confirm` (Optional)
    ///
    /// A [confirm object 🔗] that defines an optional confirmation dialog that appears
    /// after clicking one of the radio buttons in this element.
    ///
    /// [confirm object 🔗]: https://api.slack.com/reference/block-kit/composition-objects#confirm
    pub fn confirm(mut self, confirm: Confirm) -> Self {
      self.confirm = Some(confirm);
      self
    }
  }

  impl<'a, A> RadioBuilder<'a, text::Plain, A, Set<method::options>> {
    /// Sets `initial_option` (Optional)
    ///
    /// An [option object 🔗] that exactly matches one of the options within `options`.
    ///
    /// This option will be selected when the radio button group initially loads.
    ///
    /// [option object 🔗]: https://api.slack.com/reference/messaging/composition-objects#option
    pub fn initial_option(mut self,
                          option: Opt<'a, text::Plain, NoUrl>)
                          -> Self {
      self.initial_option = Some(option.into());
      self
    }
  }

  impl<'a, A> RadioBuilder<'a, text::Mrkdwn, A, Set<method::options>> {
    /// Sets `initial_option` (Optional)
    ///
    /// An [option object 🔗] that exactly matches one of the options within `options`.
    ///
    /// This option will be selected when the radio button group initially loads.
    ///
    /// [option object 🔗]: https://api.slack.com/reference/messaging/composition-objects#option
    pub fn initial_option(mut self,
                          option: Opt<'a, text::Mrkdwn, NoUrl>)
                          -> Self {
      self.initial_option = Some(option.into());
      self
    }
  }

  impl<'a, T> RadioBuilder<'a, T, Set<method::action_id>, Set<method::options>> {
    /// All done building, now give me a darn radio button group!
    ///
    /// > `no method name 'build' found for struct 'RadioBuilder<...>'`?
    /// Make sure all required setter methods have been called. See docs for `RadioBuilder`.
    ///
    /// ```compile_fail
    /// use slack_blocks::elems::Radio;
    ///
    /// let foo = Radio::builder().build(); // Won't compile!
    /// ```
    ///
    /// ```
    /// use slack_blocks::{compose::Opt, elems::Radio};
    ///
    /// let foo = Radio::builder().action_id("bar")
    ///                           .options(vec![Opt::builder().text_md("foo")
    ///                                                       .value("bar")
    ///                                                       .build()])
    ///                           .build();
    /// ```
    pub fn build(self) -> Radio<'a> {
      Radio { action_id: self.action_id.unwrap(),
              options: self.options.unwrap(),
              initial_option: self.initial_option,
              confirm: self.confirm }
    }
  }
}
