use pretty_assertions::assert_eq;
use serde_json::json;
use slack_blocks::{mox::*, text};

#[test]
pub fn docs_ex_1() {
  let text: text::Text = blox! {<text kind=mrkdwn>"A message *with some bold text* and _some italicized text_."</text>}.into();
  let actual = serde_json::to_value(text).unwrap();
  let expected = json!({
    "type": "mrkdwn",
    "text": "A message *with some bold text* and _some italicized text_."
  });

  assert_eq!(actual, expected);
}
