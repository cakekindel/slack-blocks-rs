use pretty_assertions::assert_eq;
use serde_json::json;
use slack_blocks::{elems, mox::*};

#[test]
pub fn docs_ex_1() {
  let block: elems::BlockElement =
    blox! {
      <text_input action_id="plain_input"
                  placeholder="Enter some plain text"
      />
    }.into();

  let actual = serde_json::to_value(block).unwrap();
  let expected = json!({
      "type": "plain_text_input",
      "action_id": "plain_input",
      "placeholder": {
        "type": "plain_text",
        "text": "Enter some plain text"
      }
    }
  );

  assert_eq!(actual, expected);
}

#[test]
pub fn action_trigger() {
  use slack_blocks::elems::text_input::ActionTrigger::*;

  let input: elems::BlockElement =
    blox! {
        <text_input action_id="plain_input"
                    placeholder="Enter some plain text"
                    multiline=true
                    action_trigger=OnCharacterEntered
        />
    }.into();

  let actual = serde_json::to_value(input).unwrap();
  let expected = json!({
    "type": "plain_text_input",
    "multiline": true,
    "dispatch_action_config": {
      "trigger_actions_on": ["on_character_entered"]
    }
  });
}
