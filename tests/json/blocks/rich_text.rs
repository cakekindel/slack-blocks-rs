use pretty_assertions::assert_eq;
use serde_json::json;
use slack_blocks::{blocks::{self, RichText, rich_text::RichTextSection}, blox::*, text::ToSlackPlaintext};

/// <https://github.com/cakekindel/slack-blocks-rs/issues/170>
#[test]
pub fn issue_170() {
  let button = blox! {<button action_id="fart">"Click me!!!!"</button>};
  let block: blocks::Block = slack_blocks::Block::RichText(RichText {
    block_id: Some("ommitted".into()),
    elements: vec![RichTextSection { ty: "rich_text_section".into(), elements: vec!["foobar".plaintext()] }],
});

  let actual = serde_json::to_value(block).expect("should serialize");
  let expected = json!({
        "type": "rich_text",
        "block_id": "<ommitted>",
        "elements": [
          {
            "type": "rich_text_section",
            "elements": [
              {
                "type": "text",
                "text": "foobar"
              }
            ]
          }
        ]
      });

  assert_eq!(actual, expected);
}
