use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::compose::text;
use crate::val_helpr::ValidationResult;

/// # Image Block
///
/// _[slack api docs 🔗]_
///
/// A simple image block, designed to make those cat photos really pop.
///
/// [slack api docs 🔗]: https://api.slack.com/reference/block-kit/blocks#image
#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq, Serialize, Validate)]
pub struct Contents {
    #[validate(length(max = 3000))]
    image_url: String,

    #[validate(length(max = 2000))]
    alt_text: String,

    #[validate(custom = "validate::title")]
    title: Option<text::Plain>,

    #[validate(length(max = 255))]
    block_id: Option<String>,
}

impl Contents {
    /// Create an image block, with a url and a brief description for
    /// situations where the image cannot be rendered.
    ///
    /// # Arguments
    /// - `alt_text` - A plain-text summary of the image.
    ///     This should not contain any markup.
    ///     Maximum length for this field is 2000 characters.
    ///
    /// - `image_url` - The URL of the image to be displayed.
    ///     Maximum length for this field is 3000 characters.
    ///
    /// # Example
    /// ```
    /// use slack_blocks::blocks::{Block, image};
    ///
    /// let url = "https://www.cheese.com/favicon.ico";
    /// let image: Block = image::Contents::from_alt_text_and_url("a small image of cheese.", url).into();
    ///
    /// // < send to slack api >
    /// ```
    pub fn from_alt_text_and_url(alt_text: impl ToString, image_url: impl ToString) -> Self {
        Self {
            alt_text: alt_text.to_string(),
            image_url: image_url.to_string(),
            title: None,
            block_id: None,
        }
    }

    /// Set a plain-text title to be displayed next to your image
    ///
    /// # Arguments
    /// - title - An optional title for the image in the form of a
    ///     Plaintext [text object 🔗].
    ///     Maximum length for the text in this field is 2000 characters.
    ///)
    /// [text object 🔗]: https://api.slack.com/reference/messaging/composition-objects#text
    ///
    /// # Example
    /// ```
    /// use slack_blocks::blocks::{Block, image};
    ///
    /// let url = "https://www.cheese.com/favicon.ico";
    /// let image: Block = image::Contents::from_alt_text_and_url("a small image of cheese.", url)
    ///     .with_title("here is an image of some cheese:")
    ///     .into();
    ///
    /// // < send block to slack's API >
    /// ```
    pub fn with_title(mut self, title: impl Into<text::Plain>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the `block_id` for interactions on an existing `image::Contents`
    ///
    /// # Arguments
    /// - `block_id` - A string acting as a unique identifier for a block.
    ///     You can use this `block_id` when you receive an interaction payload
    ///     to [identify the source of the action 🔗].
    ///     If not specified, a `block_id` will be generated.
    ///     Maximum length for this field is 255 characters.
    ///
    /// [identify the source of the action 🔗]: https://api.slack.com/interactivity/handling#payloads
    ///
    /// # Example
    /// ```
    /// use slack_blocks::blocks::{Block, image};
    ///
    /// let url = "https://www.cheese.com/favicon.ico";
    /// let image: Block = image::Contents::from_alt_text_and_url("a small image of cheese.", url)
    ///     .with_title("here is an image of some cheese:")
    ///     .with_block_id("msg_id_12346")
    ///     .into();
    ///
    /// // < send block to slack's API >
    /// ```
    pub fn with_block_id(mut self, block_id: impl ToString) -> Self {
        self.block_id = Some(block_id.to_string());
        self
    }

    /// Validate that this Image block agrees with Slack's model requirements
    ///
    /// # Errors
    /// - If `with_block_id` was called with a block id longer
    ///     than 255 chars
    /// - If `with_title` was called with a title longer
    ///     than 2000 chars
    /// - If `from_url_and_alt_text` was called with `alt_text` longer
    ///     than 2000 chars
    /// - If `from_url_and_alt_text` was called with `image_url` longer
    ///     than 3000 chars
    ///
    /// # Example
    /// ```
    /// use slack_blocks::blocks;
    ///
    /// let long_string = std::iter::repeat(' ').take(256).collect::<String>();
    ///
    /// let block = blocks::image
    ///     ::Contents
    ///     ::from_alt_text_and_url("", "")
    ///     .with_block_id(long_string);
    ///
    /// assert_eq!(true, matches!(block.validate(), Err(_)));
    /// ```
    pub fn validate(&self) -> ValidationResult {
        Validate::validate(self)
    }
}

mod validate {
    use crate::compose::text;
    use crate::val_helpr::{below_len, ValidatorResult};

    pub fn title(text: &text::Plain) -> ValidatorResult {
        below_len("Image Title", 2000, text.as_ref())
    }
}
