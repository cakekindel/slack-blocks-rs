use pretty_assertions::assert_eq;
use serde_json::json;
use slack_blocks::{blox::*, text::ToSlackMarkdown};

#[test]
pub fn docs_ex_1() {
  let confirm = blox! {
    <confirm title="Are you sure?"
             text="Wouldn't you prefer a good game of _chess_?".markdown()
             confirm="Do it"
             deny="Stop, I've changed my mind!"
    />
  };
  let actual = serde_json::to_value(confirm).unwrap();
  let expected = json!({
    "title": {
        "type": "plain_text",
        "text": "Are you sure?"
    },
    "text": {
        "type": "mrkdwn",
        "text": "Wouldn't you prefer a good game of _chess_?"
    },
    "confirm": {
        "type": "plain_text",
        "text": "Do it"
    },
    "deny": {
        "type": "plain_text",
        "text": "Stop, I've changed my mind!"
    }
  }
  );

  assert_eq!(actual, expected);
}
