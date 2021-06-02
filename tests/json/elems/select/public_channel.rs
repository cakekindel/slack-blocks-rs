use pretty_assertions::assert_eq;
use serde_json::json;
use slack_blocks::{blox::*, elems::BlockElement};

#[test]
pub fn docs_ex_1() {
  let sel: BlockElement = blox! {
                            <select choose_from=public_channels
                                    action_id="text1234"
                                    placeholder="Select an item"
                            />
                          }.into();

  let actual = serde_json::to_value(sel).unwrap();
  let expected = json!({
    "action_id": "text1234",
    "type": "channels_select",
    "placeholder": {
      "type": "plain_text",
      "text": "Select an item"
    }
  });

  assert_eq!(actual, expected);
}

#[test]
pub fn multi_docs_ex_1() {
  let sel: BlockElement = blox! {
                            <select kind=multi
                                    choose_from=public_channels
                                    action_id="text1234"
                                    placeholder="Select channels"
                            />
                          }.into();

  let actual = serde_json::to_value(sel).unwrap();
  let expected = json!({
    "action_id": "text1234",
    "type": "multi_channels_select",
    "placeholder": {
      "type": "plain_text",
      "text": "Select channels"
    }
  });

  assert_eq!(actual, expected);
}
