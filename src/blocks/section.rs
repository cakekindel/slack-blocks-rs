use serde::{Deserialize, Serialize};

use crate::compose::Text;
use crate::validation::{ValidationResult, is_shorter_than, is_str_shorter_than};

/// Data to render in a Section
///
/// This enum exists because of a requirement from
/// Slack that a Section block **either**:
///
/// - have the `text` field populated, in which case
///   the `fields` field is optional
/// OR
/// - have the `fields` field populated, in which
///   case the `text` field is optional
///
/// Since Section blocks are a very commonly used structure
/// in Block Kit, this "Contents" enum was chosen to enforce
/// the above requirement at compile-time.
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum SectionContents {
    Text {
        text: Text,
        fields: Option<Vec<Text>>,
        block_id: Option<String>,
        accessory: Option<()>,
    },
    Fields {
        fields: Vec<Text>,
        text: Option<Text>,
        block_id: Option<String>,
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

        let (text, fields) = match self {
            Text { text, fields, .. } => (Some(text), fields.as_ref()),
            Fields { text, fields, .. } => (text.as_ref(), Some(fields))
        };

        if let Some(text) = text {
            is_str_shorter_than(text.text(), 3000, "Section text")?;
        }

        if let Some(fields) = fields {
            is_shorter_than(fields.into_iter(), 10, "Section fields")?;
        }

        Ok(())
    }
}

#[cfg(tests)]
mod tests {
    #[test_case(
        SectionContents::from_text(Text::markdown(string_of_len(3001))) => matches Err(ValidationError::Text(TextValidationError::ExceedsMaxLen { .. }));
        "fail_when_text_longer_than_3k_chars"
    )]
    pub fn section_contents_validation_should() {}
}

