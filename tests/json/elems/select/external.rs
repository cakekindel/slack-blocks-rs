use pretty_assertions::assert_eq;
use serde_json::json;
use slack_blocks::{elems::{select, BlockElement},
                   mox::*};

#[test]
pub fn docs_ex_1() {
  let sel: BlockElement = blox! {
                            <select choose_from=external
                                    action_id="text1234"
                                    placeholder="Select an item"
                                    min_query_length=3
                            />
                          }.into();

  let actual = serde_json::to_value(sel).unwrap();
  let expected = json!({
    "action_id": "text1234",
    "type": "external_select",
    "placeholder": {
      "type": "plain_text",
      "text": "Select an item"
    },
    "min_query_length": 3
  });

  assert_eq!(actual, expected);
}

#[test]
pub fn multi_docs_ex_1() {
  let sel: BlockElement = blox! {
                            <select kind=multi
                                    choose_from=external
                                    action_id="text1234"
                                    placeholder="Select items"
                                    min_query_length=3
                            />
                          }.into();

  let actual = serde_json::to_value(sel).unwrap();
  let expected = json!({
    "action_id": "text1234",
    "type": "multi_external_select",
    "placeholder": {
      "type": "plain_text",
      "text": "Select items"
    },
    "min_query_length": 3
  });

  assert_eq!(actual, expected);
}
