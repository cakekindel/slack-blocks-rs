use slack_blocks::{
    blocks::actions, blocks::context, blocks::image, blocks::Block, compose, compose::Text,
};

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
    };
}

// ===[ Image Block Validation ]===
bad_blocks!(
    image_with_long_url:
    Block::Image(image::Contents {
        image_url: common::string_of_len(3001),
        ..Default::default()
    })
);

bad_blocks!(
    image_with_long_alt_text:
    Block::Image(image::Contents {
        alt_text: common::string_of_len(2001),
        ..Default::default()
    })
);

bad_blocks!(
    image_with_long_block_id:
    Block::Image(image::Contents {
        block_id: Some(common::string_of_len(256)),
        ..Default::default()
    })
);

bad_blocks!(
    image_with_long_title:
    Block::Image(image::Contents {
        title: Some(Text::plain(common::string_of_len(2001))),
        ..Default::default()
    })
);

bad_blocks!(
    image_with_markdown_title:
    Block::Image(image::Contents {
        title: Some(Text::markdown("*uh oh!* :flushed:")),
        ..Default::default()
    })
);

// ===[ Actions Block Validation ]===
bad_blocks!(
    actions_with_too_many_objects:
    Block::Actions(
       common::vec_of_len(
           actions::BlockElement::Button,
           6
       ).into()
    )
);

bad_blocks!(
    actions_with_long_block_id:
    Block::Actions(
        actions::Contents::new().with_block_id(common::string_of_len(256))
    )
);

// ===[ Context Block Validation ]===
bad_blocks!(
    context_with_too_many_objects:
    Block::Context(
        common::vec_of_len(
            compose::Compose::Text(Text::markdown("fart")),
            11
        ).into()
    )
);

bad_blocks!(
    context_with_long_block_id:
    Block::Context(
        context::Contents::new().with_block_id(common::string_of_len(256))
    )
);

