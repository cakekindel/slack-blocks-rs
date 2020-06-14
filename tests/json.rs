use slack_blocks::{blocks, blocks::image, compose};
use test_case::test_case;

#[feature(concat_idents)]

macro_rules! happy_json_test {
    ($test_name:ident: $json:path => $matches:pat) => {
        #[test]
        pub fn $test_name() {
            // arrange

            // act
            let actual: blocks::Block = serde_json::from_value($json.clone()).unwrap();

            // assert
            assert_eq!(matches!(actual, $matches), true)
        }
    };
}

happy_json_test!(image_should_deserialize: test_data::IMAGE_JSON => blocks::Block::Image { .. });

mod test_data {
    use slack_blocks::{blocks, compose};

    lazy_static::lazy_static! {
        static ref SAMPLE_TEXT_PLAIN: compose::Text = compose::Text::plain("Sample Text");
        static ref SAMPLE_TEXT_MRKDWN: compose::Text = compose::Text::markdown("Sample *_markdown_*");

        pub static ref IMAGE_JSON: serde_json::Value = serde_json::json!({
            "type": "image",
            "image_url": "http://cheese.com/favicon.png",
            "alt_text": "a cheese wheel.",
            "title": SAMPLE_TEXT_PLAIN.clone(),
        });
    }
}
