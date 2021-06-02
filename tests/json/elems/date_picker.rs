use pretty_assertions::assert_eq;
use serde_json::json;
use slack_blocks::{blox::*, elems, text::ToSlackPlaintext};

#[test]
pub fn docs_ex_1() {
  let block: elems::BlockElement = blox! {
                                       <date_picker action_id="datepicker123"
                                                    initial_date=(28, 04, 1990)
                                                    placeholder="Select a date"
                                       />
                                   }.into();
  let actual = serde_json::to_value(block).unwrap();
  let expected = json!({
    "type": "datepicker",
    "action_id": "datepicker123",
    "initial_date": "1990-04-28",
    "placeholder": {
      "type": "plain_text",
      "text": "Select a date"
    }
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

  let block: elems::BlockElement = blox! {
                                       <date_picker action_id="datepicker123"
                                                    confirm
                                                    initial_date=(28, 04, 1990)
                                                    placeholder="Select a date"
                                       />
                                   }.into();
  let actual = serde_json::to_value(block).unwrap();
  let expected = json!({
    "type": "datepicker",
    "action_id": "datepicker123",
    "initial_date": "1990-04-28",
    "placeholder": {
      "type": "plain_text",
      "text": "Select a date"
    },
    "confirm": {
      "title": { "type":"plain_text", "text": "Foo" },
      "text": { "type":"plain_text", "text": "" },
      "confirm": { "type":"plain_text", "text": "ye" },
      "deny": { "type":"plain_text", "text": "nah" },
    }
  });

  assert_eq!(actual, expected);
}
