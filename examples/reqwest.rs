use reqwest::Client;
use slack_blocks::blox::*;

#[tokio::main]
async fn main() {
  let client = Client::new();
  let block: slack_blocks::Block = blox! {
                                     <section_block>
                                       <text kind=mrkdwn>"Hello!"</text>
                                     </section_block>
                                   }.into();

  let api_token = env!(
                       "SLACK_API_TOKEN",
                       r#"environment variable SLACK_API_TOKEN not defined
Head over to https://api.slack.com/authentication/basics for a guide on how to create a bot user and authenticate it."#
  );

  let channel_id = env!(
      "SLACK_CHANNEL_ID",
      "environment variable SLACK_CHANNEL_ID not defined\nThis example requires a channel ID to send a message to."
    );

  let req = client.post("https://slack.com/api/chat.postMessage")
                  .header("Content-Type", "application/json")
                  .bearer_auth(api_token)
                  .body(serde_json::json!({
                    "channel": channel_id,
                    "blocks": [block]
                  }).to_string())
                  .build()
                  .unwrap();

  println!("{:#?}",
           String::from_utf8_lossy(req.body().unwrap().as_bytes().unwrap()));

  let res = client.execute(req).await.unwrap();
  println!("{:#?}", res.text().await);
}
