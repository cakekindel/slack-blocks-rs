use pretty_assertions::assert_eq;
use serde_json::json;
use slack_blocks::{elems::{BlockElement},
                   mox::*};

#[test]
pub fn docs_ex_1() {
  let sel: BlockElement = blox! {
    <select choose_from=static_ action_id="text1234" placeholder="Select an item">
      <option value="value-0" text_plain="*this is plain_text text*" />
      <option value="value-1" text_plain="*this is plain_text text*" />
      <option value="value-2" text_plain="*this is plain_text text*" />
    </select>
  }.into();

  let actual = serde_json::to_value(sel).unwrap();
  let expected = json!({
    "action_id": "text1234",
    "type": "static_select",
    "placeholder": {
      "type": "plain_text",
      "text": "Select an item"
    },
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
      }
    ]
  });

  assert_eq!(actual, expected);
}

#[test]
pub fn multi_docs_ex_1() {
  let sel: BlockElement = blox! {
    <select kind=multi choose_from=static_ action_id="text1234" placeholder="Select items">
      <option value="value-0" text_plain="*this is plain_text text*" />
      <option value="value-1" text_plain="*this is plain_text text*" />
      <option value="value-2" text_plain="*this is plain_text text*" />
    </select>
  }.into();

  let actual = serde_json::to_value(sel).unwrap();
  let expected = json!({
    "action_id": "text1234",
    "type": "multi_static_select",
    "placeholder": {
      "type": "plain_text",
      "text": "Select items"
    },
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
      }
    ]
  });

  assert_eq!(actual, expected);
}
