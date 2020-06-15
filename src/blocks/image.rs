use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::compose;
use crate::val_helpr::ValidationResult;

/// # Image Block
///
/// _[slack api docs 🔗][image_docs]_
///
/// A simple image block, designed to make those cat photos really pop.
///
/// [image_docs]: https://api.slack.com/reference/block-kit/blocks#image
#[derive(Default, Validate, Debug, Serialize, Deserialize)]
pub struct Contents {
    /// The URL of the image to be displayed.
    ///
    /// Maximum length for this field is 3000 characters.
    #[validate(length(max = 3000))]
    pub image_url: String,

    /// A plain-text summary of the image.
    ///
    /// This should not contain any markup.
    ///
    /// Maximum length for this field is 2000 characters.
    #[validate(length(max = 2000))]
    pub alt_text: String,

    /// An optional title for the image in the form of a [text object 🔗][text_object]
    /// that can only be of `type: plain_text`.
    ///
    /// Maximum length for the text in this field is 2000 characters.
    ///
    /// [text_object]: https://api.slack.com/reference/messaging/composition-objects#text
    #[validate(custom = "compose::validation::text_is_plain")]
    #[validate(custom = "validation::text_max_len_2k")]
    pub title: Option<compose::Text>,

    /// A string acting as a unique identifier for a block.
    ///
    /// You can use this `block_id` when you receive an
    /// interaction payload to [identify the source of the action 🔗][handling_payloads].
    ///
    /// If not specified, a `block_id` will be generated.
    ///
    /// Maximum length for this field is 255 characters.
    ///
    /// [handling_payloads]: https://api.slack.com/interactivity/handling#payloads
    #[validate(length(max = 255))]
    pub block_id: Option<String>,
}

impl Contents {
    pub fn validate(&self) -> ValidationResult {
        Validate::validate(self)
    }
}

mod validation {
    use crate::compose;
    use crate::val_helpr::ValidatorResult;

    pub fn text_max_len_2k(text: &compose::Text) -> ValidatorResult {
        compose::validation::text_max_len(text, 2000)
    }
}
