use slack_blocks::{blocks::image, blocks::Block, compose};

#[feature(concat_idents)]

macro_rules! happy_json_test {
    ($test_name:ident: $json:path => $matches:pat) => {
        #[test]
        pub fn $test_name() {
            // arrange

            // act
            let actual: Block = serde_json::from_value($json.clone()).unwrap();

            // assert
            assert_eq!(matches!(actual, $matches), true)
        }
    };
}

happy_json_test!(image_should_deserialize: test_data::IMAGE_JSON => Block::Image { .. });
happy_json_test!(actions_should_deserialize: test_data::ACTIONS_JSON => Block::Actions { .. });
happy_json_test!(context_should_deserialize: test_data::CONTEXT_JSON => Block::Context { .. });
happy_json_test!(section_should_deserialize: test_data::SECTION_JSON => Block::Section { .. });

mod test_data {
    use slack_blocks::{blocks, compose};

    lazy_static::lazy_static! {
        static ref SAMPLE_TEXT_PLAIN: compose::Text = compose::Text::plain("Sample Text");
        static ref SAMPLE_TEXT_MRKDWN: compose::Text = compose::Text::markdown("Sample *_markdown_*");

        pub static ref SECTION_JSON: serde_json::Value = serde_json::json!({
            "type": "section",
            "text": {
                "type": "mrkdwn",
                "text": "fart"
            }
        });

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

        // FIX: add element objects to json here after validation but before merging actions
        pub static ref ACTIONS_JSON: serde_json::Value = serde_json::json!({
            "type": "actions",
            "elements": [],
        });
    }
}
