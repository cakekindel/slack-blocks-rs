use pretty_assertions::assert_eq;
use serde_json::json;
use slack_blocks::{blocks,
                   mox::*,
                   text,
                   text::{ToSlackMarkdown, ToSlackPlaintext}};

#[test]
pub fn docs_ex_1() {
  let label = blox! {<text kind=plain emoji=true>"Label"</text>};
  let block: blocks::Block = blox! {
                               <input_block label>
                                 <text_input action_id="input" />
                               </input_block>
                             }.into();

  let actual = serde_json::to_value(block).expect("should serialize");
  let expected = json!({
    "type": "input",
    "element": {
      "type": "plain_text_input",
      "action_id": "input"
    },
    "label": {
      "type": "plain_text",
      "text": "Label",
      "emoji": true
    }
  });

  assert_eq!(actual, expected);
}

#[test]
pub fn all_attributes() {
  let label = blox! {<text kind=plain emoji=true>"Label"</text>};
  let block: blocks::Block = blox! {
                               <input_block block_id="blokkk"
                                            label
                                            optional=true
                                            dispatch_actions=true
                                            hint="type some text you fool"
                               >
                                 <text_input action_id="input" />
                               </input_block>
                             }.into();

  let actual = serde_json::to_value(block).expect("should serialize");
  let expected = json!({
    "type": "input",
    "block_id": "blokkk",
    "dispatch_action": true,
    "optional": true,
    "hint": {
      "type": "plain_text",
      "text": "type some text you fool"
    },
    "element": {
      "type": "plain_text_input",
      "action_id": "input"
    },
    "label": {
      "type": "plain_text",
      "text": "Label",
      "emoji": true
    }
  });

  assert_eq!(actual, expected);
}
