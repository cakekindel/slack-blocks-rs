use pretty_assertions::assert_eq;
use serde_json::json;
use slack_blocks::{blocks, blox::*};

#[test]
pub fn docs_ex_1() {
  let block: blocks::Block = blox! {<file_block external_id="ABCD1" />}.into();
  let actual = serde_json::to_value(block).unwrap();
  let expected = json!({
    "type": "file",
    "external_id": "ABCD1",
    "source": "remote",
  });

  assert_eq!(actual, expected);
}
