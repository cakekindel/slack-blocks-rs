use slack_blocks::blocks::Block;
use slack_blocks::block_elements::BlockElement;

macro_rules! happy_json_test {
    ($ty:ty, $test_data:ident => $matches:pat) => {
        #[test]
        #[allow(non_snake_case)]
        pub fn $test_data() {
            // arrange

            // act
            let actual: $ty = serde_json::from_value(test_data::$test_data.clone()).unwrap();

            // assert
            assert_eq!(matches!(actual, $matches), true)
        }
    };
}

happy_json_test!(Block, IMAGE_JSON => Block::Image { .. });
happy_json_test!(Block, ACTIONS_JSON => Block::Actions { .. });
happy_json_test!(Block, CONTEXT_JSON => Block::Context { .. });
happy_json_test!(Block, SECTION_JSON => Block::Section { .. });
happy_json_test!(Block, DIVIDER_JSON => Block::Divider { .. });
happy_json_test!(Block, INPUT_JSON => Block::Input { .. });
happy_json_test!(Block, FILE_JSON => Block::File { .. });

happy_json_test!(BlockElement, BUTTON_JSON => BlockElement::Button { .. });

mod test_data {
    use slack_blocks::compose::text;

    lazy_static::lazy_static! {
        static ref SAMPLE_TEXT_PLAIN: text::Text = text::Plain::from("Sample Text").into();
        static ref SAMPLE_TEXT_MRKDWN: text::Text = text::Mrkdwn::from("Sample *_markdown_*").into();

        pub static ref SECTION_JSON: serde_json::Value = serde_json::json!({
            "type": "section",
            "text": {
                "type": "mrkdwn",
                "text": "fart"
            }
        });

        // FIX: add element objects to json here when implemented
        pub static ref CONTEXT_JSON: serde_json::Value = serde_json::json!({
            "type": "context",
            "elements": []
        });

        pub static ref IMAGE_JSON: serde_json::Value = serde_json::json!({
            "type": "image",
            "image_url": "http://cheese.com/favicon.png",
            "alt_text": "a cheese wheel.",
            "title": SAMPLE_TEXT_PLAIN.clone(),
        });

        // FIX: add element objects to json here when implemented
        pub static ref ACTIONS_JSON: serde_json::Value = serde_json::json!({
            "type": "actions",
            "elements": [],
        });

        pub static ref DIVIDER_JSON: serde_json::Value = serde_json::json!({
            "type": "divider",
        });

        pub static ref FILE_JSON: serde_json::Value = serde_json::json!({
            "type": "file",
            "external_id": "abc123",
            "source": "123"
        });

        // FIX: add element objects to json here when implemented
        pub static ref INPUT_JSON: serde_json::Value = serde_json::json!({
            "type": "input",
            "label": SAMPLE_TEXT_PLAIN.clone(),
            "element": { "fixme": "see comment" },
        });

        pub static ref BUTTON_JSON: serde_json::Value = serde_json::json!({
            "type": "button",
            "text": SAMPLE_TEXT_PLAIN.clone(),
            "action_id": "abc123",
            "url": "https://www.cheese.com/",
            "style": "primary",
            "value": "valvalval",
        });
    }
}
