use pretty_assertions::assert_eq;
use serde_json::json;
use slack_blocks::{blocks, blox::*};

#[test]
pub fn docs_ex_1() {
  let block: blocks::Block = blox!{
    <actions_block block_id="actions1">
      <select choose_from=static_ action_id="select_2" placeholder="Which witch is the witchiest witch?">
        <option value="matilda" text_plain="Matilda" />
        <option value="glinda" text_plain="Glinda" />
        <option value="grannyWeatherwax" text_plain="Granny Weatherwax" />
        <option value="hermione" text_plain="Hermione" />
      </select>
      <button action_id="button_1" value="cancel">"Cancel"</button>
    </actions_block>
  }.into();

  let actual = serde_json::to_value(block).unwrap();
  let expected = json!({
    "type": "actions",
    "block_id": "actions1",
    "elements": [
      {
        "type": "static_select",
        "placeholder":{
            "type": "plain_text",
            "text": "Which witch is the witchiest witch?"
        },
        "action_id": "select_2",
        "options": [
          {
            "text": {
                "type": "plain_text",
                "text": "Matilda"
            },
            "value": "matilda"
          },
          {
            "text": {
                "type": "plain_text",
                "text": "Glinda"
            },
            "value": "glinda"
          },
          {
            "text": {
                "type": "plain_text",
                "text": "Granny Weatherwax"
            },
            "value": "grannyWeatherwax"
          },
          {
            "text": {
                "type": "plain_text",
                "text": "Hermione"
            },
            "value": "hermione"
          }
        ]
      },
      {
        "type": "button",
        "text": {
            "type": "plain_text",
            "text": "Cancel"
        },
        "value": "cancel",
        "action_id": "button_1"
      }
    ]
  });

  assert_eq!(actual, expected);
}

#[test]
pub fn docs_ex_2() {
  let block: blocks::Block =
    blox! {
      <actions_block block_id="actionblock789">
        <date_picker action_id="datepicker123"
                     initial_date=(28, 04, 1990)
                     placeholder="Select a date"
        />
        <overflow action_id="overflow">
          <option value="value-0" text_plain="*this is plain_text text*" />
          <option value="value-1" text_plain="*this is plain_text text*" />
          <option value="value-2" text_plain="*this is plain_text text*" />
          <option value="value-3" text_plain="*this is plain_text text*" />
          <option value="value-4" text_plain="*this is plain_text text*" />
        </overflow>
        <button action_id="button" value="click_me_123">"Click Me"</button>
      </actions_block>
    }.into();

  let actual = serde_json::to_value(block).unwrap();
  let expected = json!({
    "type": "actions",
    "block_id": "actionblock789",
    "elements": [
      {
        "type": "datepicker",
        "action_id": "datepicker123",
        "initial_date": "1990-04-28",
        "placeholder": {
          "type": "plain_text",
          "text": "Select a date"
        }
      },
      {
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
      },
      {
        "type": "button",
        "text": {
          "type": "plain_text",
          "text": "Click Me"
        },
        "value": "click_me_123",
        "action_id": "button"
      }
    ]
  });

  assert_eq!(actual, expected);
}
