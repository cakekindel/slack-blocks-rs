use std::borrow::Cow;

use serde::{Serialize as Ser, Deserialize as De};
use validator::Validate;

use crate::text;

#[derive(Clone, Debug, Hash, PartialEq, Ser, De)]
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
  placeholder: text::Text,
  initial_value: Cow<'a, str>,
  multiline: Option<bool>,
  #[validate(range(max = 3000))]
  min_length: Option<i32>,
  max_length: Option<i32>,
  dispatch_action_config: Option<DispatchActionConfig>,
}
