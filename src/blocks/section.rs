use serde::{Deserialize, Serialize};
use validator::{Validate};

use crate::compose;

type ValidationResult = Result<(), validator::ValidationErrors>;

/// ### Section
///
/// There is a validation requirement from
/// Slack that a Section block **either**:
///
/// - have the `text` field populated, in which case
///   the `fields` field is optional
/// OR
/// - have the `fields` field populated, in which
///   case the `text` field is optional
///
/// Since Section blocks are a very commonly used structure
/// in Block Kit, this "Contents" enum implementation
/// was chosen to enforce that requirement at compile-time.
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Contents {
    Text(Text),
    Fields(Fields),
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct Fields {
    /// An array of [text objects 🔗][text_objects].
    /// Any text objects included with fields will be
    /// rendered in a compact format that allows for
    /// 2 columns of side-by-side text.
    ///
    /// Maximum number of items is 10.
    /// Maximum length for the text in each item is 2000 characters.
    ///
    /// [text_objects]: https://api.slack.com/reference/messaging/composition-objects#text
    #[validate(length(max = 10))]
    #[validate(custom = "validation::each_text_max_len_2k")]
    fields: Vec<compose::Text>,

    /// The text for the block,
    /// in the form of a [text object 🔗][text_objects].
    ///
    /// Maximum length for the text in this field is 3000 characters.
    ///
    /// [text_objects]: https://api.slack.com/reference/messaging/composition-objects#text
    #[validate(custom = "validation::text_max_len_3k")]
    text: Option<compose::Text>,

    /// A string acting as a unique identifier for a block.
    ///
    /// You can use this block_id when you receive an interaction payload
    /// to [identify the source of the action 🔗][handling_payloads].
    /// If not specified, one will be generated.
    ///
    /// Maximum length for this field is 255 characters.
    ///
    /// block_id should be unique for each message and each iteration of a message.
    /// If a message is updated, use a new block_id.
    ///
    /// [handling_payloads]: https://api.slack.com/interactivity/handling#payloads
    #[validate(length(max = 255))]
    block_id: Option<String>,

    /// One of the available [element objects 🔗][element_objects].
    ///
    /// [element_objects]: https://api.slack.com/reference/messaging/block-elements
    accessory: Option<()>,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct Text {
    /// The text for the block,
    /// in the form of a [text object 🔗][text_objects].
    ///
    /// Maximum length for the text in this field is 3000 characters.
    ///
    /// [text_objects]: https://api.slack.com/reference/messaging/composition-objects#text
    #[validate(custom = "validation::text_max_len_3k")]
    text: compose::Text,

    /// An array of [text objects 🔗][text_objects].
    /// Any text objects included with fields will be
    /// rendered in a compact format that allows for
    /// 2 columns of side-by-side text.
    ///
    /// Maximum number of items is 10.
    /// Maximum length for the text in each item is 2000 characters.
    ///
    /// [text_objects]: https://api.slack.com/reference/messaging/composition-objects#text
    #[validate(length(max = 10))]
    #[validate(custom = "validation::each_text_max_len_2k")]
    fields: Option<Vec<compose::Text>>,

    /// A string acting as a unique identifier for a block.
    ///
    /// You can use this block_id when you receive an interaction payload
    /// to [identify the source of the action 🔗][handling_payloads].
    /// If not specified, one will be generated.
    ///
    /// Maximum length for this field is 255 characters.
    ///
    /// block_id should be unique for each message and each iteration of a message.
    /// If a message is updated, use a new block_id.
    ///
    /// [handling_payloads]: https://api.slack.com/interactivity/handling#payloads
    block_id: Option<String>,

    /// One of the available [element objects 🔗][element_objects].
    ///
    /// [element_objects]: https://api.slack.com/reference/messaging/block-elements
    accessory: Option<()>,
}

impl Contents {
    pub fn from_fields(fields: Vec<compose::Text>) -> Self {
        Contents::Fields(Fields {
            fields,
            text: None,
            block_id: None,
            accessory: None,
        })
    }

    pub fn from_text(text: compose::Text) -> Self {
        Contents::Text(Text {
            text,
            fields: None,
            block_id: None,
            accessory: None,
        })
    }

    pub fn validate(&self) -> ValidationResult {
        match self {
            Contents::Text(text) => text.validate(),
            Contents::Fields(fields) => fields.validate()
        }
    }
}

pub mod validation {
    use crate::compose;

    pub const FIELDS_MAX_CT: usize = 10;
    pub const FIELD_MAX_LEN: usize = 2000;
    pub const TEXT_MAX_LEN: usize = 3000;
    pub const BLOCK_ID_MAX_LEN: usize = 255;

    type ValidationResult = Result<(), validator::ValidationError>;

    pub fn text_max_len_3k(text: &compose::Text) -> ValidationResult {
        compose::validation::text_max_len(text, TEXT_MAX_LEN)
    }

    pub fn each_text_max_len_2k(texts: &Vec<compose::Text>) -> ValidationResult {
        texts
            .iter()
            .map(|text| compose::validation::text_max_len(text, FIELD_MAX_LEN))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::iter::repeat;
    use test_case::test_case;

    use super::*;
    use crate::compose;

    fn string_of_len(len: usize) -> String {
        repeat(' ').take(len).collect::<String>()
    }

    fn vec_of_len<T: Clone>(item: T, len: usize) -> Vec<T> {
        repeat(item).take(len).collect::<Vec<T>>()
    }

    #[test_case(
        Contents::from_text(compose::Text::markdown(string_of_len(3001)))
        => matches Err(_);
        "fail_when_text_longer_than_3k_chars"
    )]
    #[test_case(
        Contents::from_fields(vec_of_len(compose::Text::plain("".to_string()), 11))
        => matches Err(_);
        "fail_when_more_than_10_fields"
    )]
    #[test_case(
        Contents::from_fields(vec![compose::Text::plain(string_of_len(2001))])
        => matches Err(_);
        "fail_when_field_longer_than_2k_chars"
    )]
    #[test_case(
        Contents::from_fields(vec_of_len(compose::Text::plain(string_of_len(2001)), 2))
        => matches Err(_);
        "fail_when_multiple_fields_longer_than_2k_chars"
    )]
    #[test_case(
        Contents::Fields(Fields {
            text: None,
            fields: vec![],
            block_id: Some(string_of_len(256)),
            accessory: None,
        })
        => matches Err(_);
        "fail_when_block_id_longer_than_255_chars"
    )]
    pub fn section_contents_validation_should(contents: Contents) -> ValidationResult {
        // arrange (test_case input)

        // act
        contents.validate()

        // assert (test_case output)
    }
}
