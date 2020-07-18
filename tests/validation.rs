use slack_blocks::{
    block_elements, block_elements::BlockElement, blocks::actions, blocks::context, blocks::file,
    blocks::image, blocks::input, blocks::section, blocks::Block, compose::text, compose::Opt,
};

mod common;

macro_rules! should_fail {
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

// # Blocks
// ## Section Block Validation
should_fail!(
    section_with_long_block_id:
    Block::Section(
        section::Contents::from_text(text::Plain::from("")).with_block_id(common::string_of_len(256))
    )
);

should_fail!(
    section_with_long_text:
    Block::Section(
        section::Contents::from_text(text::Plain::from(common::string_of_len(3001)))
    )
);

should_fail!(
    section_with_long_field:
        Block::Section(
            section::Contents::from_fields(vec![text::Plain::from(common::string_of_len(2001))]),
        )
);

should_fail!(
    section_with_many_fields:
    Block::Section(
        section::Contents::from_fields(common::vec_of_len(text::Plain::from(""), 11))
    )
);

// ## File Block Validation
should_fail!(
    file_with_long_block_id:
    Block::File(
        file::Contents::from_external_id("").with_block_id(common::string_of_len(256))
    )
);

// ## Image Block Validation
should_fail!(
    image_with_long_url:
    Block::Image(image::Contents::from_alt_text_and_url("", common::string_of_len(3001)))
);

should_fail!(
    image_with_long_alt_text:
    Block::Image(image::Contents::from_alt_text_and_url(common::string_of_len(2001), ""))
);

should_fail!(
    image_with_long_block_id:
    Block::Image(
        image::Contents::from_alt_text_and_url("", "")
            .with_block_id(common::string_of_len(256))
    )
);

should_fail!(
    image_with_long_title:
    Block::Image(
        image::Contents::from_alt_text_and_url("", "")
            .with_title(common::string_of_len(2001))
    )
);

// ## Actions Block Validation
should_fail!(
    actions_with_too_many_objects:
    Block::Actions(
       common::vec_of_len(
           actions::BlockElement::Checkboxes,
           6
       ).into()
    )
);

should_fail!(
    actions_with_long_block_id:
    Block::Actions(
        actions::Contents::new().with_block_id(common::string_of_len(256))
    )
);

// ## Context Block Validation
should_fail!(
    context_with_too_many_objects:
    Block::Context(
        common::vec_of_len(
            text::Plain::from("fart").into(),
            11
        ).into()
    )
);

should_fail!(
    context_with_long_block_id:
    Block::Context(
        context::Contents::new().with_block_id(common::string_of_len(256))
    )
);

// ## Input Block Validation
should_fail!(
    input_with_long_label:
    Block::Input(
        input::Contents
            ::from_label_and_element(
                common::string_of_len(2001),
                block_elements::select::Static {}
            )
    )
);

should_fail!(
    input_with_long_hint:
    Block::Input(
        input::Contents
            ::from_label_and_element(
                "",
                block_elements::select::Static {}
            )
            .with_hint(common::string_of_len(2001))
    )
);

should_fail!(
    input_with_long_block_id:
    Block::Input(
        input::Contents
            ::from_label_and_element(
                "",
                block_elements::select::Static {}
            )
            .with_block_id(common::string_of_len(256))
    )
);

// # Composition Objects

// ## Option
should_fail!(
    option_plain_with_long_text:
    Opt::from_plain_text_and_value(common::string_of_len(76), "")
);
should_fail!(
    option_mrkdwn_with_long_text:
    Opt::from_mrkdwn_and_value(common::string_of_len(76), "")
);

should_fail!(
    option_with_long_value:
    Opt::from_plain_text_and_value("", common::string_of_len(76))
);

should_fail!(
    option_with_long_description:
    Opt::from_plain_text_and_value("", "")
        .with_description(common::string_of_len(76))
);

should_fail!(
    option_with_long_url:
    Opt::from_plain_text_and_value("", "")
        .with_url(common::string_of_len(3001))
);

// # Block Elements

// ## Button
should_fail!(
    button_with_long_text:
    BlockElement::Button(
        block_elements::Button
            ::from_text_and_action_id(common::string_of_len(76), "")
    )
);

should_fail!(
    button_with_long_action_id:
    BlockElement::Button(
        block_elements::Button
            ::from_text_and_action_id("", common::string_of_len(256))
    )
);

should_fail!(
    button_with_long_url:
    BlockElement::Button(
        block_elements::Button
            ::from_text_and_action_id("", "")
            .with_url(common::string_of_len(3001))
    )
);

should_fail!(
    button_with_long_value:
    BlockElement::Button(
        block_elements::Button
            ::from_text_and_action_id("", "")
            .with_value(common::string_of_len(2001))
    )
);
