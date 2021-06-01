use pretty_assertions::assert_eq;
use serde_json::json;
use slack_blocks::{blocks, mox::*};

#[test]
pub fn docs_ex_1() {
  let block: blocks::Block =
    blox! {
      <img_block block_id="image4"
                 src="http://placekitten.com/500/500"
                 title="Please enjoy this photo of a kitten"
                 alt="An incredibly cute kitten."
      />
    }.into();
  let actual = serde_json::to_value(block).unwrap();
  let expected = json!({
    "type": "image",
    "title": {
      "type": "plain_text",
      "text": "Please enjoy this photo of a kitten"
    },
    "block_id": "image4",
    "image_url": "http://placekitten.com/500/500",
    "alt_text": "An incredibly cute kitten."
  });

  assert_eq!(actual, expected);
}
