use pretty_assertions::assert_eq;
use serde_json::json;
use slack_blocks::compose::ConversationFilter;

#[test]
pub fn docs_ex_1() {
  use slack_blocks::compose::conversation_filter::ConversationKind as Kind;
  let confirm = ConversationFilter::new()
    .include_conversation_kinds(vec![Kind::PublicChannel, Kind::GroupDm])
    .exclude_bot_users();
  let actual = serde_json::to_value(confirm).unwrap();
  let expected = json!({
      "include": [
        "public",
        "mpim"
      ],
      "exclude_bot_users" : true
  });

  assert_eq!(actual, expected);
}
