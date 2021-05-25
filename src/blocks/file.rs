//! # File Block
//!
//! [slack api docs 🔗]
//!
//! Displays a [remote file 🔗]
//!
//! [slack api docs 🔗]: https://api.slack.com/reference/block-kit/blocks#file
//! [remote file 🔗]: https://api.slack.com/messaging/files/remote

use std::borrow::Cow;

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
pub struct File<'a> {
  external_id: Cow<'a, str>,
  source: Cow<'a, str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[validate(custom = "super::validate_block_id")]
  block_id: Option<Cow<'a, str>>,
}

impl<'a> File<'a> {
  /// Build a new File block.
  ///
  /// For example, see docs for FileBuilder.
  pub fn builder() -> build::FileBuilderInit<'a> {
    build::FileBuilderInit::new()
  }

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
  /// let block = blocks::File::from_external_id(file_id);
  ///
  /// // < send to slack API >
  /// # Ok(())
  /// # }
  /// ```
  #[deprecated(since = "0.19.3", note = "use File::builder")]
  pub fn from_external_id(external_file_id: impl Into<Cow<'a, str>>) -> Self {
    Self { external_id: external_file_id.into(),
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
  /// let block = blocks::File::from_external_id(file_id)
  ///     .with_block_id("my_file_in_a_block_1234");
  ///
  /// // < send to slack API >
  /// # Ok(())
  /// # }
  /// ```
  #[deprecated(since = "0.19.3", note = "use File::builder")]
  pub fn with_block_id(mut self, block_id: impl Into<Cow<'a, str>>) -> Self {
    self.block_id = Some(block_id.into());
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
  /// let block =
  ///   blocks::File::from_external_id("file_id").with_block_id(long_string);
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

/// File block builder
pub mod build {
  use std::marker::PhantomData;

  use super::*;
  use crate::build::*;

  /// Compile-time markers for builder methods
  #[allow(non_camel_case_types)]
  pub mod method {
    /// FileBuilder.external_id
    #[derive(Clone, Copy, Debug)]
    pub struct external_id;
  }

  /// Initial state for `FileBuilder`
  pub type FileBuilderInit<'a> =
    FileBuilder<'a, RequiredMethodNotCalled<method::external_id>>;

  /// Build an File block
  ///
  /// Allows you to construct safely, with compile-time checks
  /// on required setter methods.
  ///
  /// # Required Methods
  /// `FileBuilder::build()` is only available if these methods have been called:
  ///  - `external_id`
  ///  - `source`
  ///
  /// # Example
  /// ```
  /// use slack_blocks::{blocks::File, elems::Image, text::ToSlackPlaintext};
  ///
  /// let my_file_id: String = {
  ///   // use Slack Web API: files.remote.add to upload a file
  ///   # "foo".into()
  /// };
  ///
  /// let block = File::builder().external_id(my_file_id).build();
  /// ```
  #[derive(Debug)]
  pub struct FileBuilder<'a, ExternalId> {
    external_id: Option<Cow<'a, str>>,
    source: Option<Cow<'a, str>>,
    block_id: Option<Cow<'a, str>>,
    state: PhantomData<ExternalId>,
  }

  impl<'a, Ext> FileBuilder<'a, Ext> {
    /// Create a new FileBuilder
    pub fn new() -> Self {
      Self { external_id: None,
             source: None,
             block_id: None,
             state: PhantomData::<_> }
    }

    /// Set `external_id` (**Required**)
    ///
    /// The external unique ID for a [remote file 🔗].
    ///
    /// [remote file 🔗]: https://api.slack.com/messaging/files/remote
    pub fn external_id<S>(self,
                          external_id: S)
                          -> FileBuilder<'a, Set<method::external_id>>
      where S: Into<Cow<'a, str>>
    {
      FileBuilder { external_id: Some(external_id.into()),
                    source: self.source,
                    block_id: self.block_id,
                    state: PhantomData::<_> }
    }

    /// Set `block_id` (Optional)
    ///
    /// A string acting as a unique identifier for a block.
    ///
    /// You can use this `block_id` when you receive an interaction payload
    /// to [identify the source of the action 🔗].
    ///
    /// If not specified, a `block_id` will be generated.
    ///
    /// Maximum length for this field is 255 characters.
    ///
    /// [identify the source of the action 🔗]: https://api.slack.com/interactivity/handling#payloads
    pub fn block_id<S>(mut self, block_id: S) -> Self
      where S: Into<Cow<'a, str>>
    {
      self.block_id = Some(block_id.into());
      self
    }
  }

  impl<'a> FileBuilder<'a, Set<method::external_id>> {
    /// All done building, now give me a darn actions block!
    ///
    /// > `no method name 'build' found for struct 'FileBuilder<...>'`?
    /// Make sure all required setter methods have been called. See docs for `FileBuilder`.
    ///
    /// ```compile_fail
    /// use slack_blocks::blocks::File;
    ///
    /// let foo = File::builder().build(); // Won't compile!
    /// ```
    ///
    /// ```
    /// use slack_blocks::{blocks::File,
    ///                    compose::text::ToSlackPlaintext,
    ///                    elems::Image};
    ///
    /// let my_file_id: String = {
    ///   // use Slack Web API: files.remote.add to upload a file
    ///   # "foo".into()
    /// };
    ///
    /// let block = File::builder().external_id(my_file_id).build();
    /// ```
    pub fn build(self) -> File<'a> {
      File { external_id: self.external_id.unwrap(),
             source: "remote".into(),
             block_id: self.block_id }
    }
  }
}
