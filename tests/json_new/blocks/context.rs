use serde_json::json;
use slack_blocks::{blocks,
                   mox::*,
                   text,
                   text::{ToSlackMarkdown, ToSlackPlaintext}};

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

  if actual != expected {
    panic!("Expected {:#?}\n\nGot {:#?}", expected, actual);
  }
}

#[test]
pub fn minimal_attributes() {
  let block: blocks::Block = blox! {
                               <context_block>
                                 <text kind=plain>"Foo"</text>
                               </context_block>
                             }.into();

  let actual = serde_json::to_value(block).expect("should serialize");
  let expected = json!({
    "type": "context",
    "elements": [{"type": "plain_text", "text": "Foo"}]
  });

  if actual != expected {
    panic!("Expected {:#?}\n\nGot {:#?}", expected, actual);
  }
}
