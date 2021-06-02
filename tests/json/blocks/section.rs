use pretty_assertions::assert_eq;
use serde_json::json;
use slack_blocks::{blocks,
                   mox::*,
                   text::{ToSlackMarkdown}};

#[test]
pub fn all_attributes() {
  let button = blox! {<button action_id="fart">"Click me!!!!"</button>};
  let block: blocks::Block =
    blox! {
      <section_block block_id="blokkkk" text="Some dang text".markdown() accessory=button>
        <text kind=plain>"Foo"</text>
        <text kind=mrkdwn>"Bar"</text>
      </section_block>
    }.into();

  let actual = serde_json::to_value(block).expect("should serialize");
  let expected = json!({
    "type": "section",
    "block_id": "blokkkk",
    "accessory": {
      "action_id": "fart",
      "type": "button",
      "text": {
        "type": "plain_text",
        "text": "Click me!!!!"
      }
    },
    "fields": [
      {"type": "plain_text", "text": "Foo"},
      {"type": "mrkdwn", "text": "Bar"}
    ],
    "text": {
      "type": "mrkdwn",
      "text": "Some dang text"
    }
  });

  assert_eq!(actual, expected);
}

#[test]
pub fn docs_ex_1() {
  let block: blocks::Block = blox!{
    <section_block text="A message *with some bold text* and _some italicized text_.".markdown() />
  }.into();

  let actual = serde_json::to_value(block).expect("should serialize");
  let expected = json!({
    "type": "section",
    "text": {
      "type": "mrkdwn",
      "text": "A message *with some bold text* and _some italicized text_."
    }
  });

  if actual != expected {
    panic!("Expected {:#?}\n\nGot {:#?}", expected, actual);
  }
}

#[test]
pub fn docs_ex_2() {
  let block: blocks::Block = blox!{
    <section_block text="A message *with some bold text* and _some italicized text_.".markdown()>
      <text kind=mrkdwn>"High"</text>
      <text kind=plain emoji=true>"String"</text>
    </section_block>
  }.into();

  let actual = serde_json::to_value(block).expect("should serialize");
  let expected = json!({
    "type": "section",
    "text": {
      "text": "A message *with some bold text* and _some italicized text_.",
      "type": "mrkdwn"
    },
    "fields": [
      {
        "type": "mrkdwn",
        "text": "High"
      },
      {
        "type": "plain_text",
        "emoji": true,
        "text": "String"
      }
    ]
  });

  assert_eq!(actual, expected);
}

#[test]
pub fn docs_ex_3() {
  let pick_date = blox! {
    <date_picker action_id="datepicker123"
                 placeholder="Select a date"
                 initial_date=(28, 4, 1990) />
  };

  let block: blocks::Block = blox!{
    <section_block accessory=pick_date text="*Sally* has requested you set the deadline for the Nano launch project".markdown() />
  }.into();

  let actual = serde_json::to_value(block).expect("should serialize");
  let expected = json!({
    "type": "section",
    "text": {
      "text": "*Sally* has requested you set the deadline for the Nano launch project",
      "type": "mrkdwn"
    },
    "accessory": {
      "type": "datepicker",
      "action_id": "datepicker123",
      "initial_date": "1990-04-28",
      "placeholder": {
        "type": "plain_text",
        "text": "Select a date"
      }
    }
  });

  assert_eq!(actual, expected);
}
