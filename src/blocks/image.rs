//! # Image Block
//!
//! _[slack api docs 🔗]_
//!
//! A simple image block, designed to make those cat photos really pop.
//!
//! [slack api docs 🔗]: https://api.slack.com/reference/block-kit/blocks#image

use std::borrow::Cow;

use serde::{Deserialize, Serialize};
  #[cfg(feature = "validation")]
use validator::Validate;

use crate::{compose::text, };
#[cfg(feature = "validation")]
use crate::val_helpr::ValidationResult;

/// # Image Block
///
/// _[slack api docs 🔗]_
///
/// A simple image block, designed to make those cat photos really pop.
///
/// [slack api docs 🔗]: https://api.slack.com/reference/block-kit/blocks#image
#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
#[cfg_attr(feature = "validation", derive(Validate))]
pub struct Image<'a> {
  #[cfg_attr(feature = "validation", validate(length(max = 3000)))]
  image_url: Cow<'a, str>,

  #[cfg_attr(feature = "validation", validate(length(max = 2000)))]
  alt_text: Cow<'a, str>,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[cfg_attr(feature = "validation", validate(custom = "validate::title"))]
  title: Option<text::Text>,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[cfg_attr(feature = "validation",
             validate(custom = "super::validate_block_id"))]
  block_id: Option<Cow<'a, str>>,
}

impl<'a> Image<'a> {
  /// Build a new Image block.
  ///
  /// For example, see docs for ImageBuilder.
  pub fn builder() -> build::ImageBuilderInit<'a> {
    build::ImageBuilderInit::new()
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
  /// let block = blocks::Image::builder().image_url("")
  ///                                     .alt("")
  ///                                     .block_id(long_string)
  ///                                     .build();
  ///
  /// assert_eq!(true, matches!(block.validate(), Err(_)));
  /// ```
  #[cfg(feature = "validation")]
  #[cfg_attr(docsrs, doc(cfg(feature = "validation")))]
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
    /// ImageBuilder.image_url or src
    #[derive(Clone, Copy, Debug)]
    pub struct url;

    /// ImageBuilder.alt_text or alt
    #[derive(Clone, Copy, Debug)]
    pub struct alt;
  }

  /// Initial state for `ImageBuilder`
  pub type ImageBuilderInit<'a> =
    ImageBuilder<'a,
                 RequiredMethodNotCalled<method::url>,
                 RequiredMethodNotCalled<method::alt>>;

  /// Build an Image block
  ///
  /// Allows you to construct safely, with compile-time checks
  /// on required setter methods.
  ///
  /// # Required Methods
  /// `ImageBuilder::build()` is only available if these methods have been called:
  ///  - `external_id`
  ///  - `source`
  ///
  /// # Example
  /// ```
  /// use slack_blocks::{blocks::Image, text::ToSlackPlaintext};
  ///
  /// let block = Image::builder().image_url("https://foo.com/bar.png")
  ///                             .alt_text("pic of bar")
  ///                             .build();
  /// ```
  #[derive(Debug)]
  pub struct ImageBuilder<'a, Url, Alt> {
    image_url: Option<Cow<'a, str>>,
    alt_text: Option<Cow<'a, str>>,
    title: Option<text::Text>,
    block_id: Option<Cow<'a, str>>,
    state: PhantomData<(Url, Alt)>,
  }

  impl<'a, Url, Alt> ImageBuilder<'a, Url, Alt> {
    /// Create a new ImageBuilder
    pub fn new() -> Self {
      Self { image_url: None,
             alt_text: None,
             title: None,
             block_id: None,
             state: PhantomData::<_> }
    }

    /// Set `title` (Optional)
    ///
    /// An optional title for the image in the form of a
    /// Plaintext [text object 🔗].
    ///
    /// Maximum length for the text in this field is 2000 characters.
    ///
    /// [text object 🔗]: https://api.slack.com/reference/messaging/composition-objects#text
    pub fn title<T>(mut self, text: T) -> Self
      where T: Into<text::Plain>
    {
      self.title = Some(text.into().into());
      self
    }

    /// Alias for `image_url`.
    pub fn src<S>(self, image_url: S) -> ImageBuilder<'a, Set<method::url>, Alt>
      where S: Into<Cow<'a, str>>
    {
      self.image_url(image_url)
    }

    /// Set `image_url` (**Required**)
    ///
    /// The URL of the image to be displayed.
    ///
    /// Maximum length for this field is 3000 characters.
    pub fn image_url<S>(self,
                        image_url: S)
                        -> ImageBuilder<'a, Set<method::url>, Alt>
      where S: Into<Cow<'a, str>>
    {
      ImageBuilder { image_url: Some(image_url.into()),
                     alt_text: self.alt_text,
                     title: self.title,
                     block_id: self.block_id,
                     state: PhantomData::<_> }
    }

    /// Set `alt_text` (**Required**)
    ///
    /// A plain-text summary of the image.
    ///
    /// This should not contain any markup.
    ///
    /// Maximum length for this field is 2000 characters.
    pub fn alt_text<S>(self,
                       alt_text: S)
                       -> ImageBuilder<'a, Url, Set<method::alt>>
      where S: Into<Cow<'a, str>>
    {
      ImageBuilder { alt_text: Some(alt_text.into()),
                     image_url: self.image_url,
                     title: self.title,
                     block_id: self.block_id,
                     state: PhantomData::<_> }
    }

    /// Alias for `alt_text`.
    pub fn alt<S>(self, alt_text: S) -> ImageBuilder<'a, Url, Set<method::alt>>
      where S: Into<Cow<'a, str>>
    {
      self.alt_text(alt_text)
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

  impl<'a> ImageBuilder<'a, Set<method::url>, Set<method::alt>> {
    /// All done building, now give me a darn actions block!
    ///
    /// > `no method name 'build' found for struct 'ImageBuilder<...>'`?
    /// Make sure all required setter methods have been called. See docs for `ImageBuilder`.
    ///
    /// ```compile_fail
    /// use slack_blocks::blocks::Image;
    ///
    /// let foo = Image::builder().build(); // Won't compile!
    /// ```
    ///
    /// ```
    /// use slack_blocks::{blocks::Image, compose::text::ToSlackPlaintext};
    ///
    /// let block = Image::builder().image_url("https://foo.com/bar.png")
    ///                             .alt_text("pic of bar")
    ///                             .build();
    /// ```
    pub fn build(self) -> Image<'a> {
      Image { image_url: self.image_url.unwrap(),
              alt_text: self.alt_text.unwrap(),
              title: self.title,
              block_id: self.block_id }
    }
  }
}

#[cfg(feature = "validation")]
mod validate {
  use crate::{compose::text,
              val_helpr::{below_len, ValidatorResult}};

  pub(super) fn title(text: &text::Text) -> ValidatorResult {
    below_len("Image Title", 2000, text.as_ref())
  }
}
