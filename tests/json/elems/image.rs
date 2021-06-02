use pretty_assertions::assert_eq;
use serde_json::json;
use slack_blocks::{elems, mox::*};

#[test]
pub fn docs_ex_1() {
  let block: elems::BlockElement =
    blox! {
        <img src="http://placekitten.com/700/500" alt="Multiple cute kittens" />
    }.into();
  let actual = serde_json::to_value(block).unwrap();
  let expected = json!({
    "type": "image",
    "image_url": "http://placekitten.com/700/500",
    "alt_text": "Multiple cute kittens"
  });

  assert_eq!(actual, expected);
}
