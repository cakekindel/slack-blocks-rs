use pretty_assertions::assert_eq;
use serde_json::json;
use slack_blocks::{blox::*};

#[test]
pub fn option_docs_ex_1() {
  let confirm = blox! {
    <option value="maru" text_plain="Maru" />
  };
  let actual = serde_json::to_value(confirm).unwrap();
  let expected = json!({
    "text": {
      "type": "plain_text",
      "text": "Maru"
    },
    "value": "maru"
  });

  assert_eq!(actual, expected);
}

#[test]
pub fn groups_docs_ex_1() {
  let groups = vec![blox! {
                      <option_group label="Group 1">
                        <option value="value-0" text_plain="*this is plain_text text*" />
                        <option value="value-1" text_plain="*this is plain_text text*" />
                        <option value="value-2" text_plain="*this is plain_text text*" />
                      </option_group>
                    },
                    blox! {
                      <option_group label="Group 2">
                        <option value="value-3" text_plain="*this is plain_text text*" />
                      </option_group>
                    }];
  let actual = serde_json::to_value(groups).unwrap();
  let expected = json!(
  [
    {
      "label": {
        "type": "plain_text",
        "text": "Group 1"
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
    },
    {
      "label": {
          "type": "plain_text",
          "text": "Group 2"
      },
      "options": [
        {
          "text": {
              "type": "plain_text",
              "text": "*this is plain_text text*"
          },
          "value": "value-3"
        }
      ]
    }
  ]);

  assert_eq!(actual, expected);
}
