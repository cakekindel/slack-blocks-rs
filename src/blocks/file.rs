use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::val_helpr::ValidationResult;

/// # File Block
///
/// [slack api docs 🔗]
///
/// Displays a [remote file 🔗]
///
/// [slack api docs 🔗]: https://api.slack.com/reference/block-kit/blocks#file
/// [remote file 🔗]: https://api.slack.com/messaging/files/remote
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize, Validate)]
pub struct Contents {
  external_id: String,
  source: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[validate(length(max = 255))]
  block_id: Option<String>,
}

impl Contents {
  /// Create a file block from a [remote file 🔗]'s external ID.
  ///
  /// # Arguments
  /// - `external_file_id` - The external unique ID for this file,
  ///     which notably is an ID in slack's system that is a reference
  ///     or hyperlink to your original resource, which is hosted
  ///     outside of Slack.
  ///     Slack does not support uploading files to send in a block
  ///     at this time.
  ///
  /// [remote file 🔗]: https://api.slack.com/messaging/files/remote
  ///
  /// # Example
  /// ```
  /// use slack_blocks::{blocks, compose};
  ///
  /// # fn upload_file_to_slack(s: &str) -> String { String::new() }
  /// # use std::error::Error;
  /// # pub fn main() -> Result<(), Box<dyn Error>> {
  /// let file_id = upload_file_to_slack("https://www.cheese.com/cheese-wheel.png");
  ///
  /// let block = blocks::file::Contents::from_external_id(file_id);
  ///
  /// // < send to slack API >
  /// # Ok(())
  /// # }
  /// ```
  pub fn from_external_id(external_file_id: impl AsRef<str>) -> Self {
    Self { external_id: external_file_id.as_ref().into(),
           source: "remote".into(),
           block_id: None }
  }

  /// Set a unique `block_id` to identify this instance of an File Block.
  ///
  /// # Arguments
  ///
  /// - `block_id` - A string acting as a unique identifier for a block.
  ///     You can use this `block_id` when you receive an interaction
  ///     payload to [identify the source of the action 🔗].
  ///     If not specified, one will be generated.
  ///     Maximum length for this field is 255 characters.
  ///     `block_id` should be unique for each message and each iteration of a message.
  ///     If a message is updated, use a new `block_id`.
  ///
  /// [identify the source of the action 🔗]: https://api.slack.com/interactivity/handling#payloads
  ///
  /// # example
  /// ```
  /// use slack_blocks::{blocks, compose};
  ///
  /// # fn upload_file_to_slack(s: &str) -> String { String::new() }
  /// # use std::error::Error;
  /// # pub fn main() -> Result<(), Box<dyn Error>> {
  /// let file_id = upload_file_to_slack("https://www.cheese.com/cheese-wheel.png");
  ///
  /// let block = blocks::file::Contents::from_external_id(file_id)
  ///     .with_block_id("my_file_in_a_block_1234");
  ///
  /// // < send to slack API >
  /// # Ok(())
  /// # }
  /// ```
  pub fn with_block_id(mut self, block_id: impl AsRef<str>) -> Self {
    self.block_id = Some(block_id.as_ref().to_string());
    self
  }

  /// Validate that this File block agrees with Slack's model requirements
  ///
  /// # Errors
  /// - If `with_block_id` was called with a block id longer
  ///     than 256 chars
  ///
  /// # Example
  /// ```
  /// use slack_blocks::{blocks, compose};
  ///
  /// # use std::error::Error;
  /// # pub fn main() -> Result<(), Box<dyn Error>> {
  /// let long_string = std::iter::repeat(' ').take(256).collect::<String>();
  ///
  /// let block = blocks::file
  ///     ::Contents
  ///     ::from_external_id("file_id")
  ///     .with_block_id(long_string);
  ///
  /// assert_eq!(true, matches!(block.validate(), Err(_)));
  ///
  /// // < send to slack API >
  /// # Ok(())
  /// # }
  /// ```
  pub fn validate(&self) -> ValidationResult {
    Validate::validate(self)
  }
}
