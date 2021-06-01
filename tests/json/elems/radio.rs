use pretty_assertions::assert_eq;
use serde_json::json;
use slack_blocks::{elems, mox::*, text::ToSlackPlaintext};

#[test]
pub fn docs_ex_1() {
  let opt_1 = blox! {<option value="A1" text_plain="Radio 1" />};
  let block: elems::BlockElement =
    blox! {
      <radio_buttons action_id="this_is_an_action_id" initial_option={opt_1.clone()}>
        {opt_1}
        <option value="A2" text_plain="Radio 2" />
      </radio_buttons>
    }.into();

  let actual = serde_json::to_value(block).unwrap();
  let expected = json!({
    "type": "radio_buttons",
    "action_id": "this_is_an_action_id",
    "initial_option": {
      "value": "A1",
      "text": {
        "type": "plain_text",
        "text": "Radio 1"
      }
    },
    "options": [
      {
        "value": "A1",
        "text": {
          "type": "plain_text",
          "text": "Radio 1"
        }
      },
      {
        "value": "A2",
        "text": {
          "type": "plain_text",
          "text": "Radio 2"
        }
      }
    ]
  });

  assert_eq!(actual, expected);
}

#[test]
pub fn all_attributes() {
  let confirm = blox! {
    <confirm title="Foo"
             text="".plaintext()
             confirm="ye"
             deny="nah"
    />
  };

  let opt_1 = blox! {<option value="A1" text_plain="Radio 1" />};
  let block: elems::BlockElement =
    blox! {
      <radio_buttons confirm action_id="this_is_an_action_id" initial_option={opt_1.clone()}>
        {opt_1}
        <option value="A2" text_plain="Radio 2" />
      </radio_buttons>
    }.into();

  let actual = serde_json::to_value(block).unwrap();
  let expected = json!({
    "type": "radio_buttons",
    "action_id": "this_is_an_action_id",
    "confirm": {
      "title": { "type":"plain_text", "text": "Foo" },
      "text": { "type":"plain_text", "text": "" },
      "confirm": { "type":"plain_text", "text": "ye" },
      "deny": { "type":"plain_text", "text": "nah" },
    },
    "initial_option": {
      "value": "A1",
      "text": {
        "type": "plain_text",
        "text": "Radio 1"
      }
    },
    "options": [
      {
        "value": "A1",
        "text": {
          "type": "plain_text",
          "text": "Radio 1"
        }
      },
      {
        "value": "A2",
        "text": {
          "type": "plain_text",
          "text": "Radio 2"
        }
      }
    ]
  });

  assert_eq!(actual, expected);
}
