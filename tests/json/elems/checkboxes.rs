use pretty_assertions::assert_eq;
use serde_json::json;
use slack_blocks::{blox::*, elems, text::ToSlackPlaintext};

#[test]
pub fn docs_ex_1() {
  let opt_1 = blox! {<option value="A1" text_plain="Checkbox 1" />};
  let block: elems::BlockElement =
    blox! {
      <checkboxes action_id="this_is_an_action_id" initial_options=vec![opt_1.clone()]>
        {opt_1}
        <option value="A2" text_plain="Checkbox 2" />
      </checkboxes>
    }.into();
  let actual = serde_json::to_value(block).unwrap();
  let expected = json!({
    "type": "checkboxes",
    "action_id": "this_is_an_action_id",
    "initial_options": [{
      "value": "A1",
      "text": {
        "type": "plain_text",
        "text": "Checkbox 1"
      }
    }],
    "options": [
      {
        "value": "A1",
        "text": {
          "type": "plain_text",
          "text": "Checkbox 1"
        }
      },
      {
        "value": "A2",
        "text": {
          "type": "plain_text",
          "text": "Checkbox 2"
        }
      }
    ]
  });

  assert_eq!(actual, expected);
}

#[test]
pub fn all_attributes() {
  let opt_1 = blox! {<option value="A1" text_plain="Checkbox 1" />};
  let confirm = blox! {
    <confirm title="You sure?"
             text="yes or no, are you sure?".plaintext()
             confirm="Yes"
             deny="No"
    />
  };
  let block: elems::BlockElement =
    blox! {
      <checkboxes confirm action_id="this_is_an_action_id" initial_options=vec![opt_1.clone()]>
        {opt_1}
        <option value="A2" text_plain="Checkbox 2" />
      </checkboxes>
    }.into();
  let actual = serde_json::to_value(block).unwrap();
  let expected = json!({
    "type": "checkboxes",
    "action_id": "this_is_an_action_id",
    "confirm": {
      "title": {"type": "plain_text", "text": "You sure?"},
      "text": {"type": "plain_text", "text": "yes or no, are you sure?"},
      "confirm": {"type": "plain_text", "text": "Yes"},
      "deny": {"type": "plain_text", "text": "No"}
    },
    "initial_options": [{
      "value": "A1",
      "text": {
        "type": "plain_text",
        "text": "Checkbox 1"
      }
    }],
    "options": [
      {
        "value": "A1",
        "text": {
          "type": "plain_text",
          "text": "Checkbox 1"
        }
      },
      {
        "value": "A2",
        "text": {
          "type": "plain_text",
          "text": "Checkbox 2"
        }
      }
    ]
  });

  assert_eq!(actual, expected);
}
