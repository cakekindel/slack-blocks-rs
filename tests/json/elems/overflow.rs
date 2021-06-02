use pretty_assertions::assert_eq;
use serde_json::json;
use slack_blocks::{elems, blox::*, text::ToSlackPlaintext};

#[test]
pub fn docs_ex_1() {
  let block: elems::BlockElement =
    blox! {
      <overflow action_id="overflow">
        <option value="value-0" text_plain="*this is plain_text text*" />
        <option value="value-1" text_plain="*this is plain_text text*" />
        <option value="value-2" text_plain="*this is plain_text text*" />
        <option value="value-3" text_plain="*this is plain_text text*" />
        <option value="value-4" text_plain="*this is plain_text text*" />
      </overflow>
    }.into();
  let actual = serde_json::to_value(block).unwrap();
  let expected = json!({
    "type": "overflow",
    "options": [
      {
        "text": {
          "type": "plain_text",
          "text": "*this is plain_text text*"
        },
        "value": "value-0"
      },
      {
        "text": {
          "type": "plain_text",
          "text": "*this is plain_text text*"
        },
        "value": "value-1"
      },
      {
        "text": {
          "type": "plain_text",
          "text": "*this is plain_text text*"
        },
        "value": "value-2"
      },
      {
        "text": {
          "type": "plain_text",
          "text": "*this is plain_text text*"
        },
        "value": "value-3"
      },
      {
        "text": {
          "type": "plain_text",
          "text": "*this is plain_text text*"
        },
        "value": "value-4"
      }
    ],
    "action_id": "overflow"
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

  let block: elems::BlockElement =
    blox! {
      <overflow confirm action_id="overflow">
        <option value="value-0" text_plain="*this is plain_text text*" />
      </overflow>
    }.into();

  let actual = serde_json::to_value(block).unwrap();
  let expected = json!({
    "type": "overflow",
    "options": [
      {
        "text": {
          "type": "plain_text",
          "text": "*this is plain_text text*"
        },
        "value": "value-0"
      }
    ],
    "action_id": "overflow",
    "confirm": {
      "title": { "type":"plain_text", "text": "Foo" },
      "text": { "type":"plain_text", "text": "" },
      "confirm": { "type":"plain_text", "text": "ye" },
      "deny": { "type":"plain_text", "text": "nah" },
    }
  });

  assert_eq!(actual, expected);
}
