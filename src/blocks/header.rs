//! # Header Block
//!
//! [slack api docs 🔗]
//!
//! A header is a plain-text block that displays in a larger, bold font.
//!
//! Use it to delineate between different groups of content in your app's surfaces.
//!
//! [slack api docs 🔗]: https://api.slack.com/reference/block-kit/blocks#header

use std::borrow::Cow;

use serde::{Deserialize, Serialize};


use crate::text;
#[cfg(feature = "validation")]
use crate::val_helpr::*;

/// # Header Block
///
/// [slack api docs 🔗]
///
/// A header is a plain-text block that displays in a larger, bold font.
///
/// Use it to delineate between different groups of content in your app's surfaces.
///
/// [slack api docs 🔗]: https://api.slack.com/reference/block-kit/blocks#header
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
#[cfg_attr(feature = "validation", derive(Validate))]
pub struct Header<'a> {
  #[cfg_attr(feature = "validation", validate(custom = "validate_text"))]
  text: text::Text,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[cfg_attr(feature = "validation",
             validate(custom = "super::validate_block_id"))]
  block_id: Option<Cow<'a, str>>,
}

#[cfg(feature = "validation")]
fn validate_text(t: &text::Text) -> ValidatorResult {
  below_len("text", 150, t)
}

impl<'a> Header<'a> {
  /// Build a new Header block.
  ///
  /// Alias for [`build::HeaderBuilder::new()`].
  pub fn builder() -> build::HeaderBuilderInit<'a> {
    build::HeaderBuilder::new()
  }

  /// Validate that this Header block agrees with Slack's model requirements
  ///
  /// # Errors
  /// - If `text` longer than 150 chars
  /// - If `block_id` longer than 256 chars
  ///
  /// # Example
  /// ```
  /// # use validator::Validate;
  /// use slack_blocks::{blocks::Header, compose};
  ///
  /// let long_string = |len| std::iter::repeat(' ').take(len).collect::<String>();
  /// let assert_invalid = |block: &dyn Validate| match block.validate() {
  ///   | Ok(()) => panic!("validation should have failed"),
  ///   | Err(_) => (),
  /// };
  ///
  /// // block_id
  /// let block = Header::builder().text("foo")
  ///                              .block_id(long_string(256))
  ///                              .build();
  ///
  /// assert_invalid(&block);
  ///
  /// // text
  /// let block = Header::builder().text(long_string(151))
  ///                              .block_id("foo")
  ///                              .build();
  ///
  /// assert_invalid(&block);
  /// ```
  #[cfg(feature = "validation")]
  #[cfg_attr(docsrs, doc(cfg(feature = "validation")))]
  pub fn validate(&self) -> ValidationResult {
    todo!()
  }
}

pub mod build {
  //! [HeaderBuilder] and related items

  use std::marker::PhantomData;

  use super::*;
  use crate::build::*;

  /// Methods of [`HeaderBuilder`]
  #[allow(non_camel_case_types)]
  pub mod method {
    /// [`super::HeaderBuilder::text()`]
    #[derive(Debug, Clone, Copy)]
    pub struct text;
  }

  /// Initial state for [HeaderBuilder]
  pub type HeaderBuilderInit<'a> =
    HeaderBuilder<'a, RequiredMethodNotCalled<method::text>>;

  /// Build an Header block
  ///
  /// Allows you to construct safely, with compile-time checks
  /// on required setter methods.
  ///
  /// # Required Methods
  /// `HeaderBuilder::build()` is only available if these methods have been called:
  ///  - `text`
  ///
  /// # Example
  /// ```
  /// use slack_blocks::blocks::Header;
  ///
  /// let block = Header::builder().block_id("foo").text("bar").build();
  /// ```
  #[derive(Clone, Debug)]
  pub struct HeaderBuilder<'a, T> {
    text: Option<text::Text>,
    block_id: Option<Cow<'a, str>>,
    state: PhantomData<T>,
  }

  impl<'a, T> HeaderBuilder<'a, T> {
    /// Construct a new HeaderBuilder
    pub fn new() -> Self {
      Self { text: None,
             block_id: None,
             state: PhantomData::<_> }
    }

    /// Set `text` (**Required**)
    ///
    /// The text for the block, in the form of a [`plain_text` text object 🔗].
    ///
    /// Maximum length for the `text` in this field is 150 characters.
    ///
    /// [`plain_text` text object 🔗]: https://api.slack.com/reference/messaging/composition-objects#text
    pub fn text(self,
                text: impl Into<text::Plain>)
                -> HeaderBuilder<'a, Set<method::text>> {
      HeaderBuilder { text: Some(text.into().into()),
                      block_id: self.block_id,
                      state: PhantomData::<_> }
    }

    /// XML child alias for [`Self::text()`]
    #[cfg(feature = "blox")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blox")))]
    pub fn child(self,
                 text: impl Into<text::Plain>)
                 -> HeaderBuilder<'a, Set<method::text>> {
      self.text(text)
    }

    /// A string acting as a unique identifier for a block.
    ///
    /// If not specified, one will be generated.
    ///
    /// Maximum length for this field is 255 characters.
    /// `block_id` should be unique for each message and each iteration of a message.
    ///
    /// If a message is updated, use a new `block_id`.
    pub fn block_id(mut self, block_id: impl Into<Cow<'a, str>>) -> Self {
      self.block_id = Some(block_id.into());
      self
    }
  }

  impl<'a> HeaderBuilder<'a, Set<method::text>> {
    /// All done building, now give me a darn actions block!
    ///
    /// > `no method name 'build' found for struct 'FileBuilder<...>'`?
    /// Make sure all required setter methods have been called. See docs for `FileBuilder`.
    ///
    /// ```compile_fail
    /// use slack_blocks::blocks::Header;
    ///
    /// let foo = Header::builder().build(); // Won't compile!
    /// ```
    ///
    /// ```
    /// use slack_blocks::blocks::Header;
    ///
    /// let block = Header::builder().text("Foo").build();
    /// ```
    pub fn build(self) -> Header<'a> {
      Header { text: self.text.unwrap(),
               block_id: self.block_id }
    }
  }
}
