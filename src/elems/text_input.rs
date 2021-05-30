//! # Plain Text Input
//!
//! [slack api docs 🔗]
//!
//! A plain-text input, similar to the HTML `<input>` tag, creates a field where a user can enter freeform data.
//! It can appear as a single-line field or a larger textarea using the `multiline` flag.
//!
//! Works in [blocks 🔗]: Input
//! Works in [app surfaces 🔗]: Home tabs, Modals, Messages
//!
//! [slack api docs 🔗]: https://api.slack.com/reference/block-kit/block-elements#radio

use std::borrow::Cow;

use serde::{Deserialize as De, Serialize as Ser};
use validator::Validate;

use crate::{text, val_helpr::*};

/// Interaction types that you would like to receive a [`block_actions` payload 🔗] for.
///
/// [`block_actions` payload 🔗]: https://api.slack.com/reference/interaction-payloads/block-actions
#[derive(Clone, Copy, Debug, Hash, PartialEq, Ser, De)]
#[serde(untagged, rename_all = "snake_case")]
pub enum ActionTrigger {
  /// Payload is dispatched when user presses the enter key while the input is in focus.
  ///
  /// Hint text will appear underneath the input explaining to the user to press enter to submit.
  OnEnterPressed,

  /// Payload is dispatched when a character is entered (or removed) in the input.
  OnCharacterEntered,
}

/// [api docs](https://api.slack.com/reference/block-kit/composition-objects#dispatch_action_config)
#[derive(Clone, Debug, Hash, PartialEq, Ser, De)]
struct DispatchActionConfig {
  trigger_actions_on: Vec<ActionTrigger>,
}

/// # Plain Text Input
///
/// [slack api docs 🔗]
///
/// A plain-text input, similar to the HTML `<input>` tag, creates a field where a user can enter freeform data.
/// It can appear as a single-line field or a larger textarea using the `multiline` flag.
///
/// Works in [blocks 🔗]: Input
/// Works in [app surfaces 🔗]: Home tabs, Modals, Messages
///
/// [slack api docs 🔗]: https://api.slack.com/reference/block-kit/block-elements#radio
#[derive(Clone, Debug, Hash, PartialEq, Ser, De, Validate)]
pub struct TextInput<'a> {
  #[validate(length(max = 255))]
  action_id: Cow<'a, str>,

  #[validate(custom = "validate_placeholder")]
  #[serde(skip_serializing_if = "Option::is_none")]
  placeholder: Option<text::Text>,

  #[serde(skip_serializing_if = "Option::is_none")]
  initial_value: Option<Cow<'a, str>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  multiline: Option<bool>,

  #[validate(range(max = 3000))]
  #[serde(skip_serializing_if = "Option::is_none")]
  min_length: Option<u32>,

  #[serde(skip_serializing_if = "Option::is_none")]
  max_length: Option<u32>,

  #[serde(skip_serializing_if = "Option::is_none")]
  dispatch_action_config: Option<DispatchActionConfig>,
}

impl<'a> TextInput<'a> {
  /// Build a new text input block element
  ///
  /// # Examples
  /// See example for `build::TextInputBuilder`.
  pub fn builder() -> build::TextInputBuilderInit<'a> {
    build::TextInputBuilderInit::new()
  }

  /// Validate that this select element agrees with Slack's model requirements
  ///
  /// # Errors
  /// - length of `placeholder` greater than 150
  /// - length of `action_id` greater than 255
  /// - value of `min_length` greater than 3000
  ///
  /// # Example
  /// ```
  /// use slack_blocks::elems::TextInput;
  ///
  /// let long_string = || std::iter::repeat('a').take(256).collect::<String>();
  ///
  /// let input = TextInput::builder().action_id(long_string())
  ///                                 .placeholder(long_string())
  ///                                 .min_length(3001)
  ///                                 .build();
  ///
  /// assert!(matches!(input.validate(), Err(_)))
  /// ```
  pub fn validate(&self) -> ValidationResult {
    Validate::validate(self)
  }
}

fn validate_placeholder<'a>(p: &text::Text) -> ValidatorResult {
  below_len("TextInput.placeholder", 150, p)
}

/// Text Input Builder
pub mod build {
  use std::marker::PhantomData;

  use super::*;
  use crate::build::*;

  /// Required Builder Method markers
  #[allow(non_camel_case_types)]
  pub mod method {
    /// TextInputBuilder.action_id
    #[derive(Copy, Clone, Debug)]
    pub struct action_id;
  }

  /// Initial state for TextInputBuilder
  pub type TextInputBuilderInit<'a> =
    TextInputBuilder<'a, RequiredMethodNotCalled<method::action_id>>;

  /// Build a Text Input element
  ///
  /// Allows you to construct a text input safely, with compile-time checks
  /// on required setter methods.
  ///
  /// # Required Methods
  /// `TextInputBuilder::build()` is only available if these methods have been called:
  ///  - `action_id`
  ///
  /// # Examples
  ///
  /// ```
  /// use slack_blocks::{blocks::{Block, Input}, elems::TextInput};
  ///
  /// let text_input = TextInput::builder()
  ///                            .action_id("plate_num")
  ///                            .placeholder("ABC1234")
  ///                            .length(1..=7)
  ///                            .build();
  ///
  /// let block: Block = Input::from_label_and_element("enter custom license plate", text_input)
  ///                          .dispatch_block_actions()
  ///                          .into();
  /// ```
  #[derive(Debug)]
  pub struct TextInputBuilder<'a, A> {
    action_id: Option<Cow<'a, str>>,
    placeholder: Option<text::Text>,
    initial_value: Option<Cow<'a, str>>,
    multiline: Option<bool>,
    min_length: Option<u32>,
    max_length: Option<u32>,
    dispatch_action_config: Option<DispatchActionConfig>,
    state: PhantomData<A>,
  }

  impl<'a, A> TextInputBuilder<'a, A> {
    /// Construct a new text input builder of empty state
    pub fn new() -> Self {
      Self { action_id: None,
             placeholder: None,
             initial_value: None,
             multiline: None,
             min_length: None,
             max_length: None,
             dispatch_action_config: None,
             state: PhantomData::<_> }
    }

    /// Set `action_id` (**Required**)
    ///
    /// An identifier for the input value when the parent modal is submitted.
    ///
    /// You can use this when you receive a `view_submission` payload [to identify the value of the input element 🔗].
    ///
    /// Should be unique among all other `action_id`s in the containing block.
    ///
    /// Maximum length for this field is 255 characters.
    ///
    /// [to identify the value of the input element 🔗]: https://api.slack.com/surfaces/modals/using#handling-submissions
    pub fn action_id(self,
                     action_id: impl Into<Cow<'a, str>>)
                     -> TextInputBuilder<'a, Set<method::action_id>> {
      TextInputBuilder { action_id: Some(action_id.into()),
                         placeholder: self.placeholder,
                         initial_value: self.initial_value,
                         multiline: self.multiline,
                         min_length: self.min_length,
                         max_length: self.max_length,
                         dispatch_action_config: self.dispatch_action_config,
                         state: PhantomData::<_> }
    }

    /// Add a new event trigger (Optional)
    ///
    /// In messages, in order to receive events you must invoke this method and set `dispatch_action` to `true` on the containing Input block.
    ///
    /// In modals and other contexts, the value of this element will be included with the submission of the form.
    ///
    /// By invoking this with `ActionTrigger::OnCharacterEntered`, `ActionTrigger::OnEnterPressed`, or both,
    /// you can configure the input element to send additional events when these triggers are fired by the client.
    ///
    /// For more info on these events, see [`block_actions` interaction payload 🔗].
    ///
    /// [`block_actions` interaction payload 🔗]: https://api.slack.com/reference/interaction-payloads/block-actions
    ///
    /// # Examples
    ///
    /// ```
    /// use slack_blocks::{blocks::{Block, Input}, elems::TextInput};
    ///
    /// let text_input = TextInput::builder()
    ///                            .action_id("plate_num")
    ///                            .placeholder("ABC1234")
    ///                            .length(1..=7)
    ///                            .build();
    ///
    /// let block: Block = Input::from_label_and_element("enter custom license plate", text_input)
    ///                          .dispatch_block_actions()
    ///                          .into();
    /// ```
    pub fn trigger_action_on(mut self, trigger: ActionTrigger) -> Self {
      let config =
        self.dispatch_action_config
            .map(|mut c| {
              if !c.trigger_actions_on.contains(&trigger) {
                c.trigger_actions_on.push(trigger)
              }

              c
            })
            .unwrap_or_else(|| DispatchActionConfig { trigger_actions_on:
                                                        vec![trigger] });

      self.dispatch_action_config = Some(config);
      self
    }

    /// Set `placeholder` (**Optional**)
    ///
    /// A [`plain_text` only text object 🔗] that defines the placeholder text shown in the plain-text input.
    ///
    /// Maximum length for the `text` in this field is 150 characters.
    ///
    /// [`plain_text` only text object 🔗]: https://api.slack.com/reference/block-kit/composition-objects#text
    pub fn placeholder(mut self, placeholder: impl Into<text::Plain>) -> Self {
      self.placeholder = Some(placeholder.into().into());
      self
    }

    /// Set `initial value` (**Optional**)
    ///
    /// The initial value in the plain-text input when it is loaded.
    pub fn initial_value(mut self, init: impl Into<Cow<'a, str>>) -> Self {
      self.initial_value = Some(init.into());
      self
    }

    /// Set `multiline` (**Optional**)
    ///
    /// Indicates that the input will be a larger textarea,
    /// rather than a single line.
    ///
    /// Default is `false`.
    pub fn multiline(mut self, multiline: bool) -> Self {
      self.multiline = Some(multiline);
      self
    }

    /// Set `min_length` (**Optional**)
    ///
    /// The minimum length of input that the user must provide.
    ///
    /// If the user provides less, they will receive an error.
    ///
    /// Maximum value is 3000.
    pub fn min_length(mut self, min: u32) -> Self {
      self.min_length = Some(min.into());
      self
    }

    /// Set `max_length` (**Optional**)
    ///
    /// The maximum length of input that the user can provide.
    ///
    /// If the user provides more, they will receive an error.
    pub fn max_length(mut self, max: u32) -> Self {
      self.max_length = Some(max.into());
      self
    }

    /// Set `min_length` and/or `max_length` with a rust range literal (**Optional**)
    ///
    /// ```
    /// use slack_blocks::elems::TextInput;
    ///
    /// TextInput::builder().action_id("vanity_plate")
    ///                     .placeholder("enter your desired custom license plate")
    ///                     .length(1..=7);
    /// ```
    ///
    /// ```
    /// use slack_blocks::elems::TextInput;
    ///
    /// TextInput::builder().action_id("first_name")
    ///                     .placeholder("enter your first name")
    ///                     .length(2..);
    /// ```
    ///
    /// ```
    /// use slack_blocks::elems::TextInput;
    ///
    /// TextInput::builder()
    ///           .action_id("does nothing")
    ///           .placeholder("This is the same as not calling length at all!")
    ///           .length(..);
    /// ```
    pub fn length(mut self, rng: impl std::ops::RangeBounds<u32>) -> Self {
      use std::ops::Bound;

      self.min_length = match rng.start_bound() {
        | Bound::Included(min) => Some(*min),
        | Bound::Excluded(min) => Some(min + 1),
        | Bound::Unbounded => None,
      };

      self.max_length = match rng.end_bound() {
        | Bound::Included(max) => Some(*max),
        | Bound::Excluded(max) => Some(max - 1),
        | Bound::Unbounded => None,
      };

      self
    }
  }

  impl<'a> TextInputBuilder<'a, Set<method::action_id>> {
    /// All done building, now give me a darn text input!
    ///
    /// > `no method name 'build' found for struct 'text_input::build::TextInputBuilder<...>'`?
    /// Make sure all required setter methods have been called. See docs for `TextInputBuilder`.
    ///
    /// ```compile_fail
    /// use slack_blocks::elems::TextInput;
    ///
    /// let sel = TextInput::builder().build(); // Won't compile!
    /// ```
    ///
    /// ```
    /// use slack_blocks::elems::TextInput;
    ///
    /// let sel = TextInput::builder().action_id("bar").build();
    /// ```
    pub fn build(self) -> TextInput<'a> {
      TextInput { action_id: self.action_id.unwrap(),
                  placeholder: self.placeholder,
                  initial_value: self.initial_value,
                  multiline: self.multiline,
                  min_length: self.min_length,
                  max_length: self.max_length,
                  dispatch_action_config: self.dispatch_action_config }
    }
  }
}
