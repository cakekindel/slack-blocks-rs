use slack_blocks::block_elements::BlockElement;
use slack_blocks::blocks::Block;
use slack_blocks::compose::{ConversationFilter, Opt, OptGroup, Text};

macro_rules! happy_json_test {
    ($name:ident, $test_data:expr => $matches:pat) => {
        #[test]
        #[allow(non_snake_case)]
        pub fn $name() {
            // arrange

            // act
            let actual = serde_json::from_value($test_data.clone()).unwrap();

            // assert
            assert_eq!(matches!(actual, $matches), true)
        }
    };
}

happy_json_test!(image,   test_data::IMAGE_JSON => Block::Image { .. });
happy_json_test!(actions, test_data::ACTIONS_JSON => Block::Actions { .. } );
happy_json_test!(context, test_data::CONTEXT_JSON => Block::Context { .. } );
happy_json_test!(section, test_data::SECTION_JSON => Block::Section { .. } );
happy_json_test!(divider, test_data::DIVIDER_JSON => Block::Divider { .. } );
happy_json_test!(input,   test_data::INPUT_JSON => Block::Input { .. });
happy_json_test!(file,    test_data::FILE_JSON => Block::File { .. });

happy_json_test!(option,       test_data::OPT_JSON => Opt::<()> { .. });
happy_json_test!(option_group, test_data::OPT_GROUP_JSON => OptGroup::<()> { .. });
happy_json_test!(text,         test_data::MRKDWN_TEXT_JSON => Text::Mrkdwn { .. });
happy_json_test!(conv_filter,  test_data::CONV_FILTER_JSON => ConversationFilter { .. });

happy_json_test!(button, test_data::BUTTON_JSON => BlockElement::Button { .. });

mod test_data {
    use slack_blocks::compose::text;

    lazy_static::lazy_static! {
        static ref SAMPLE_TEXT_PLAIN: text::Text = text::Plain::from("Sample Text").into();
        static ref SAMPLE_TEXT_MRKDWN: text::Text = text::Mrkdwn::from("Sample *_markdown_*").into();

        pub static ref MRKDWN_TEXT_JSON: serde_json::Value = serde_json::json!({
            "type": "mrkdwn",
            "text": "fart"
        });

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

        pub static ref OPT_JSON: serde_json::Value = serde_json::json!({
            "text": SAMPLE_TEXT_PLAIN.clone(),
            "value": "valvalval",
            "description": SAMPLE_TEXT_PLAIN.clone(),
            "url": "https://www.url.com/",
        });

        pub static ref OPT_GROUP_JSON: serde_json::Value = serde_json::json!({
            "label": SAMPLE_TEXT_PLAIN.clone(),
            "options": [],
        });

        pub static ref CONV_FILTER_JSON: serde_json::Value = serde_json::json!({
            "include": ["mpim", "im", "public", "private"],
            "exclude_bot_users": true,
            "exclude_external_shared_channels": true,
        });
    }
}
