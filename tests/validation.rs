use slack_block_kit::{blocks::Block::*, blocks::image, compose::Text};

mod common;

macro_rules! bad_blocks {
    ($test_name:ident: $data:expr) => {
        #[test]
        pub fn $test_name() {
            // arrange
            let data = $data;

            // act
            let val_result = data.validate();

            // assert
            match val_result {
                Ok(_) => panic!("validation didn't fail!"),
                Err(_) => (),
            }
        }
    }
}

// ===[ Image Block Validation ]===
bad_blocks!(
    image_with_long_url:
    Image(image::Contents {
        image_url: common::string_of_len(3001),
        title: None,
        block_id: None,
        alt_text: String::new(),
    })
);

bad_blocks!(
    image_with_long_alt_text:
    Image(image::Contents {
        alt_text: common::string_of_len(2001),
        ..Default::default()
    })
);

bad_blocks!(
    image_with_long_block_id:
    Image(image::Contents {
        block_id: Some(common::string_of_len(256)),
        ..Default::default()
    })
);

bad_blocks!(
    image_with_long_title:
    Image(image::Contents {
        title: Some(Text::plain(common::string_of_len(2001))),
        ..Default::default()
    })
);

bad_blocks!(
    image_with_markdown_title:
    Image(image::Contents {
        title: Some(Text::markdown("*uh oh!* :flushed:")),
        ..Default::default()
    })
);

