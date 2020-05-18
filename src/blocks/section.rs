use serde::{Deserialize, Serialize};

use crate::compose::Text;
use crate::validation::{is_shorter_than, is_str_shorter_than, ValidationError, ValidationResult};

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
pub enum SectionContents {
    Text {
        /// The text for the block,
        /// in the form of a [text object 🔗][text_objects].
        ///
        /// Maximum length for the text in this field is 3000 characters.
        ///
        /// [text_objects]: https://api.slack.com/reference/messaging/composition-objects#text
        text: Text,

        /// An array of [text objects 🔗][text_objects].
        /// Any text objects included with fields will be
        /// rendered in a compact format that allows for
        /// 2 columns of side-by-side text.
        ///
        /// Maximum number of items is 10.
        /// Maximum length for the text in each item is 2000 characters.
        ///
        /// [text_objects]: https://api.slack.com/reference/messaging/composition-objects#text
        fields: Option<Vec<Text>>,

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
    },
    Fields {
        /// An array of [text objects 🔗][text_objects].
        /// Any text objects included with fields will be
        /// rendered in a compact format that allows for
        /// 2 columns of side-by-side text.
        ///
        /// Maximum number of items is 10.
        /// Maximum length for the text in each item is 2000 characters.
        ///
        /// [text_objects]: https://api.slack.com/reference/messaging/composition-objects#text
        fields: Vec<Text>,

        /// The text for the block,
        /// in the form of a [text object 🔗][text_objects].
        ///
        /// Maximum length for the text in this field is 3000 characters.
        ///
        /// [text_objects]: https://api.slack.com/reference/messaging/composition-objects#text
        text: Option<Text>,

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
    },
}

impl SectionContents {
    pub fn from_fields(fields: Vec<Text>) -> Self {
        SectionContents::Fields {
            fields,
            text: None,
            block_id: None,
            accessory: None,
        }
    }

    pub fn from_text(text: Text) -> Self {
        SectionContents::Text {
            text,
            fields: None,
            block_id: None,
            accessory: None,
        }
    }

    pub fn validate(&self) -> ValidationResult {
        use SectionContents::*;

        let mut val_results = Vec::<ValidationResult>::new();

        let (text, fields, block_id) = match self {
            Text {
                text,
                fields,
                block_id,
                ..
            } => (Some(text), fields.as_ref(), block_id),
            Fields {
                text,
                fields,
                block_id,
                ..
            } => (text.as_ref(), Some(fields), block_id),
        };

        val_results.push(is_str_shorter_than(
            &block_id
                .as_ref()
                .map(|s| s.clone())
                .unwrap_or_default()
                .as_str(),
            255,
            "Section block id",
        ));

        if let Some(text) = text {
            val_results.push(is_str_shorter_than(text.text(), 3000, "Section text"));
        }

        if let Some(fields) = fields {
            val_results.push(is_shorter_than(fields.iter(), 10, "Section fields"));

            let mut field_len_errors: Vec<ValidationResult> = fields
                .iter()
                .enumerate()
                .map(|(ix, text)| {
                    is_str_shorter_than(
                        text.text(),
                        2000,
                        format!("Section Field ix. {}", ix).as_str(),
                    )
                })
                .collect();

            val_results.append(&mut field_len_errors);
        }

        let mut val_errors: Vec<ValidationError> = val_results
            .into_iter()
            .filter(|r| r.is_err())
            .map(|r| r.unwrap_err())
            .collect();

        if val_errors.is_empty() {
            Ok(())
        } else if val_errors.len() == 1 {
            Err(val_errors.remove(0))
        } else {
            Err(ValidationError::Multiple(val_errors))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::iter::repeat;
    use test_case::test_case;

    use super::*;
    use crate::validation::ValidationError;

    fn string_of_len(len: usize) -> String {
        repeat(' ').take(len).collect::<String>()
    }

    fn vec_of_len<T: Clone>(item: T, len: usize) -> Vec<T> {
        repeat(item).take(len).collect::<Vec<T>>()
    }

    #[test_case(
        SectionContents::from_text(Text::markdown(string_of_len(3001))) => matches Err(ValidationError::ExceedsMaxLen { .. });
        "fail_when_text_longer_than_3k_chars"
    )]
    #[test_case(
        SectionContents::from_fields(vec_of_len(Text::plain("".to_string()), 11)) => matches Err(ValidationError::ExceedsMaxLen { .. });
        "fail_when_more_than_10_fields"
    )]
    pub fn section_contents_validation_should(contents: SectionContents) -> ValidationResult {
        // arrange
        // act
        contents.validate()
        // assert
    }
}
