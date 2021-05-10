mod util {
    pub fn ensure_nones_omitted(json: &serde_json::Value) {
        use serde_json::Value;

        match json {
            Value::Object(map) => map.values().into_iter().for_each(ensure_nones_omitted),
            Value::Array(arr) => arr.iter().for_each(ensure_nones_omitted),
            Value::Null => panic!("contained nulls"),
            _ => (),
        }
    }
}

macro_rules! json_test {
    (fn $name:ident() { deserialize($test_data:expr) => $matches:pat }) => {
        #[test]
        pub fn $name() {
            // arrange

            // act
            let actual =
                serde_json::from_value($test_data.clone()).expect("test data should deserialize");

            // assert
            assert_eq!(matches!(actual, $matches), true);

            // TODO(orion): is this necessary? hmm..
            // roundtrip JSON to ensure no nulls were introduced
            let serialized = serde_json::to_string(&actual).unwrap();
            let deserialized = serde_json::from_str(&serialized).unwrap();

            util::ensure_nones_omitted(&deserialized);
        }
    };
}

// TODO(orion): Refactor tests to _serialize_ items into JSON, not deserialize. The crate shouldn't even be used for deserialization

mod block_tests {
    use super::*;
    use slack_blocks::blocks::Block;
    use test_data::*;

    json_test!(fn image()   { deserialize(IMAGE_JSON)   => Block::Image {..}   });
    json_test!(fn actions() { deserialize(ACTIONS_JSON) => Block::Actions {..} });
    json_test!(fn context() { deserialize(CONTEXT_JSON) => Block::Context {..} });
    json_test!(fn section() { deserialize(SECTION_JSON) => Block::Section {..} });
    json_test!(fn divider() { deserialize(DIVIDER_JSON) => Block::Divider {..} });
    json_test!(fn input()   { deserialize(INPUT_JSON)   => Block::Input {..}   });
    json_test!(fn file()    { deserialize(FILE_JSON)    => Block::File {..}    });
}

mod compose {
    use super::*;
    use slack_blocks::compose;
    use test_data::*;

    json_test!(fn option()       { deserialize(OPT_JSON)         => compose::Opt::<()> {..}          });
    json_test!(fn text()         { deserialize(MRKDWN_TEXT_JSON) => compose::Text::Mrkdwn {..}       });
    json_test!(fn confirm()      { deserialize(CONFIRM_DIALOG)   => compose::Confirm {..}            });
    json_test!(fn option_group() { deserialize(OPT_GROUP_JSON)   => compose::OptGroup::<()> {..}     });
    json_test!(fn conv_filter()  { deserialize(CONV_FILTER_JSON) => compose::ConversationFilter {..} });
}

mod block_elements {
    use super::*;
    use slack_blocks::block_elements::*;
    use test_data::*;

    json_test!(fn button() { deserialize(BUTTON_JSON) => BlockElement::Button { .. } });

    json_test!(fn public_channel_select() { deserialize(PUB_CHAN_SELECT_JSON) => BlockElement::SelectPublicChannel(_) });
}

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

        // TODO(orion): add element objects to json here when implemented
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

        // TODO(orion): add element objects to json here when implemented
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

        // TODO(orion): add element objects to json here when implemented
        pub static ref INPUT_JSON: serde_json::Value = serde_json::json!({
            "type": "input",
            "label": SAMPLE_TEXT_PLAIN.clone(),
            "element": PUB_CHAN_SELECT_JSON.clone(),
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

        pub static ref CONFIRM_DIALOG: serde_json::Value = serde_json::json!({
            "title": SAMPLE_TEXT_PLAIN.clone(),
            "text": SAMPLE_TEXT_PLAIN.clone(),
            "confirm": SAMPLE_TEXT_PLAIN.clone(),
            "deny": SAMPLE_TEXT_PLAIN.clone(),
            "style": "danger"
        });

        pub static ref PUB_CHAN_SELECT_JSON: serde_json::Value = serde_json::json!({
            "type": "channels_select",
            "placeholder": SAMPLE_TEXT_PLAIN.clone(),
            "action_id": "1234",
            "initial_channel": "C2394",
            "confirm": CONFIRM_DIALOG.clone(),
        });
    }
}
