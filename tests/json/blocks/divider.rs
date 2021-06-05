use pretty_assertions::assert_eq;
use serde_json::json;
use slack_blocks::{blocks, blox::*};

#[test]
pub fn docs_ex_1() {
  let block: blocks::Block = blox! {<hr />}.into();
  let actual = serde_json::to_value(block).unwrap();
  let expected = json!({
    "type": "divider"
  });

  assert_eq!(actual, expected);
}
