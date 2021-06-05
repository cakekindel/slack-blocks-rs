use pretty_assertions::assert_eq;
use serde_json::json;
use slack_blocks::{blocks, blox::*};

#[test]
pub fn it_works() {
  blox! {
    <h1>"Foo"</h1>
  };
}

#[test]
pub fn docs_ex_1() {
  let block: blocks::Block = blox! {<h1>"Budget Performance"</h1>}.into();
  let actual = serde_json::to_value(block).unwrap();
  let expected = json!({
    "type": "header",
    "text": {
      "type": "plain_text",
      "text": "Budget Performance"
    }
  });

  assert_eq!(actual, expected);
}

#[test]
pub fn all_attrs() {
  let block: blocks::Block =
    blox! {<h1 block_id="foo">"Budget Performance"</h1>}.into();
  let actual = serde_json::to_value(block).unwrap();
  let expected = json!({
    "type": "header",
    "block_id": "foo",
    "text": {
      "type": "plain_text",
      "text": "Budget Performance"
    }
  });

  assert_eq!(actual, expected);
}
