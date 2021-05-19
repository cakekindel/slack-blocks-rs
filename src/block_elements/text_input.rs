use std::borrow::Cow;

use serde::{Serialize as Ser, Deserialize as De};
use validator::Validate;

use crate::{text, val_helpr::ValidationResult};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Ser, De)]
#[serde(untagged, rename_all = "snake_case")]
pub enum ActionTrigger {
  OnEnterPressed,
  OnCharacterEntered,
}

#[derive(Clone, Debug, Hash, PartialEq, Ser, De)]
struct DispatchActionConfig {
  trigger_actions_on: Vec<ActionTrigger>,
}

#[derive(Clone, Debug, Hash, PartialEq, Ser, De, Validate)]
pub struct TextInput<'a> {
  #[validate(length(max = 255))]
  action_id: Cow<'a, str>,
  placeholder: Option<text::Text>,
  initial_value: Option<Cow<'a, str>>,
  multiline: Option<bool>,
  #[validate(range(max = 3000))]
  min_length: Option<u32>,
  max_length: Option<u32>,
  dispatch_action_config: Option<DispatchActionConfig>,
}

impl<'a> TextInput<'a> {
  pub fn builder() -> build::TextInputBuilderInit<'a> {
    build::TextInputBuilderInit::new()
  }

  pub fn validate(&self) -> ValidationResult {
    Validate::validate(self)
  }
}

pub mod build {
  use super::*;
  use crate::build::*;
  use std::marker::PhantomData;

  #[allow(non_camel_case_types)]
  pub mod method {
    pub struct action_id;
  }

  pub type TextInputBuilderInit<'a> = TextInputBuilder<'a, RequiredMethodNotCalled<method::action_id>>;

  pub struct TextInputBuilder<'a, A> {
    action_id: Option<Cow<'a, str>>,
    placeholder: Option<text::Text>,
    initial_value: Option<Cow<'a, str>>,
    multiline: Option<bool>,
    min_length: Option<u32>,
    max_length: Option<u32>,
    dispatch_action_config: Option<DispatchActionConfig>,
    state: PhantomData<A>
  }

  impl<'a, A> TextInputBuilder<'a, A> {
    pub fn new() -> Self {
      Self {
        action_id: None,
        placeholder: None,
        initial_value: None,
        multiline: None,
        min_length: None,
        max_length: None,
        dispatch_action_config: None,
        state: PhantomData::<_>
      }
    }

    pub fn action_id(mut self, action_id: impl Into<Cow<'a, str>>) ->  TextInputBuilder<'a, Set<method::action_id>> {
      TextInputBuilder {
        action_id: Some(action_id.into()),
        placeholder: self.placeholder,
        initial_value: self.initial_value,
        multiline: self.multiline,
        min_length: self.min_length,
        max_length: self.max_length,
        dispatch_action_config: self.dispatch_action_config,
        state: PhantomData::<_>
      }
    }

    pub fn trigger_action_on(mut self, trigger: ActionTrigger) -> Self {
      let config = self.dispatch_action_config
                                        .map(|mut c| {
                                          if !c.trigger_actions_on.contains(&trigger) {
                                            c.trigger_actions_on.push(trigger)
                                          }

                                          c
                                        })
                                        .unwrap_or_else(|| {
                                          DispatchActionConfig {
                                            trigger_actions_on: vec![trigger],
                                          }
                                        });

      self.dispatch_action_config = Some(config);
      self
    }

    pub fn placeholder(mut self, placeholder: impl Into<text::Plain>) -> Self {
      self.placeholder = Some(placeholder.into().into());
      self
    }

    pub fn initial_value(mut self, init: impl Into<Cow<'a, str>>) -> Self {
      self.initial_value = Some(init.into());
      self
    }

    pub fn multiline(mut self) -> Self {
      self.multiline = Some(true);
      self
    }

    pub fn min_length(mut self, min: impl Into<u32>) -> Self {
      self.min_length = Some(min.into());
      self
    }

    pub fn max_length(mut self, max: impl Into<u32>) -> Self {
      self.max_length = Some(max.into());
      self
    }

    pub fn length(mut self, rng: impl std::ops::RangeBounds<u32>) -> Self {
      use std::ops::Bound;

      self.min_length = match rng.start_bound() {
        Bound::Included(min) => Some(*min),
        Bound::Excluded(min) => Some(min + 1),
        Bound::Unbounded => None,
      };

      self.max_length = match rng.end_bound() {
        Bound::Included(max) => Some(*max),
        Bound::Excluded(max) => Some(max - 1),
        Bound::Unbounded => None,
      };

      self
    }

  }

  impl<'a> TextInputBuilder<'a, Set<method::action_id>> {
    pub fn build(self) -> TextInput<'a> {
      TextInput {
        action_id: self.action_id.unwrap(),
        placeholder: self.placeholder,
        initial_value: self.initial_value,
        multiline: self.multiline,
        min_length: self.min_length,
        max_length: self.max_length,
        dispatch_action_config: self.dispatch_action_config,
      }
    }
  }
}
