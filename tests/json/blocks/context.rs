use pretty_assertions::assert_eq;
use serde_json::json;
use slack_blocks::{blocks, blox::*};

#[test]
pub fn docs_ex_1() {
  let block: blocks::Block = blox!{
    <context_block>
      <img src="https://image.freepik.com/free-photo/red-drawing-pin_1156-445.jpg" alt="images"/>
      <text kind=mrkdwn>"Location: **Dogpatch**"</text>
    </context_block>
  }.into();

  let actual = serde_json::to_value(block).unwrap();
  let expected = json!({
    "type": "context",
    "elements": [
      {
        "type": "image",
        "image_url": "https://image.freepik.com/free-photo/red-drawing-pin_1156-445.jpg",
        "alt_text": "images"
      },
      {
        "type": "mrkdwn",
        "text": "Location: **Dogpatch**"
      }
    ]
  });

  assert_eq!(actual, expected);
}

#[test]
pub fn all_attributes() {
  let block: blocks::Block = blox! {
                               <context_block block_id="foo">
                                 <text kind=plain>"Foo"</text>
                                 <text kind=plain>"Bar"</text>
                               </context_block>
                             }.into();

  let actual = serde_json::to_value(block).expect("should serialize");
  let expected = json!({
    "type": "context",
    "block_id": "foo",
    "elements": [
      {"type": "plain_text", "text": "Foo"},
      {"type": "plain_text", "text": "Bar"}
    ]
  });

  assert_eq!(actual, expected);
}
