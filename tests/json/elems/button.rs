use pretty_assertions::assert_eq;
use serde_json::json;
use slack_blocks::{blox::*, elems};

#[test]
pub fn docs_ex_1() {
  let block: elems::BlockElement =
    blox! {
      <button action_id="button" value="click_me_123">
        "Click Me"
      </button>
    }.into();
  let actual = serde_json::to_value(block).unwrap();
  let expected = json!({
    "type": "button",
    "text": {
      "type": "plain_text",
      "text": "Click Me"
    },
    "value": "click_me_123",
    "action_id": "button"
  });

  assert_eq!(actual, expected);
}

#[test]
pub fn docs_ex_2() {
  let block: elems::BlockElement =
    blox! {
      <button action_id="button" value="click_me_123" style=btn_primary>
        "Save"
      </button>
    }.into();
  let actual = serde_json::to_value(block).unwrap();
  let expected = json!({
    "type": "button",
    "text": {
      "type": "plain_text",
      "text": "Save"
    },
    "style": "primary",
    "value": "click_me_123",
    "action_id": "button"
  });

  assert_eq!(actual, expected);
}

#[test]
pub fn docs_ex_3() {
  let block: elems::BlockElement =
    blox! {
      <button action_id="button" url="https://api.slack.com/block-kit">
        "Link Button"
      </button>
    }.into();
  let actual = serde_json::to_value(block).unwrap();
  let expected = json!({
    "type": "button",
    "action_id": "button",
    "text": {
      "type": "plain_text",
      "text": "Link Button"
    },
    "url": "https://api.slack.com/block-kit"
  });

  assert_eq!(actual, expected);
}
