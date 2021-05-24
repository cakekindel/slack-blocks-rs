use std::borrow::Cow;

use serde::{Deserialize as De, Serialize as Ser};
use validator::Validate;

use crate::val_helpr::ValidationResult;

/// # Image Element
///
/// An element to insert an image as part of a larger block of content.
///
/// If you want a block with _only_ an image in it, you're looking for the [`image` block 🔗].
///
/// [slack api docs 🔗]
///
/// Works in [blocks 🔗]: Section, Context
///
/// [`image` block 🔗]: https://api.slack.com/reference/block-kit/blocks#image
/// [slack api docs 🔗]: https://api.slack.com/reference/block-kit/block-elements#radio
/// [blocks 🔗]: https://api.slack.com/reference/block-kit/blocks
#[derive(Clone, Debug, Hash, PartialEq, Ser, De, Validate)]
pub struct Image<'a> {
  image_url: Cow<'a, str>,
  alt_text: Cow<'a, str>,
}

impl<'a> Image<'a> {
  /// Build a new Image element.
  ///
  /// # Example
  /// see example for `build::ImageBuilder`.
  pub fn builder() -> build::ImageBuilderInit<'a> {
    build::ImageBuilderInit::new()
  }

  /// Validate that this image element agrees with Slack's model requirements.
  ///
  /// No rules are specified in the Slack docs at the time of writing so this will always succeed.
  pub fn validate(&self) -> ValidationResult {
    Ok(())
  }
}

pub mod build {
  use std::marker::PhantomData;

  use super::*;
  use crate::build::*;

  #[allow(non_camel_case_types)]
  pub mod method {
    #[derive(Copy, Clone, Debug)]
    pub struct image_url;
    #[derive(Copy, Clone, Debug)]
    pub struct alt_text;
  }

  pub type ImageBuilderInit<'a> =
    ImageBuilder<'a,
                 RequiredMethodNotCalled<method::image_url>,
                 RequiredMethodNotCalled<method::alt_text>>;

  /// Image Element builder
  ///
  /// Allows you to construct safely, with compile-time checks
  /// on required setter methods.
  ///
  /// # Required Methods
  /// `ImageBuilder::build()` is only available if these methods have been called:
  ///  - `image_url`
  ///  - `alt_text`
  ///
  /// # Example
  /// ```
  /// use slack_blocks::{blocks::{Block, Context},
  ///                    elems::Image};
  ///
  /// let img = Image::builder().image_url("foo").alt_text("bar").build();
  ///
  /// let block: Block =
  ///   Context::from_context_elements(std::iter::once(img)).into();
  ///
  /// // <send block to slack API>
  /// ```
  #[derive(Debug)]
  pub struct ImageBuilder<'a, U, A> {
    image_url: Option<Cow<'a, str>>,
    alt_text: Option<Cow<'a, str>>,
    state: PhantomData<(U, A)>,
  }

  impl<'a, U, A> ImageBuilder<'a, U, A> {
    /// Construct a new builder
    pub fn new() -> Self {
      Self { image_url: None,
             alt_text: None,
             state: PhantomData::<_> }
    }

    /// Set `image_url` (**Required**)
    ///
    /// The URL of the image to be displayed.
    pub fn image_url<S>(self,
                        image_url: S)
                        -> ImageBuilder<'a, Set<method::image_url>, A>
      where S: Into<Cow<'a, str>>
    {
      ImageBuilder { image_url: Some(image_url.into()),
                     alt_text: self.alt_text,
                     state: PhantomData::<_> }
    }

    /// Set `alt_text` (**Required**)
    ///
    /// A plain-text summary of the image.
    ///
    /// This should not contain any markup.
    pub fn alt_text<S>(self,
                       alt_text: S)
                       -> ImageBuilder<'a, U, Set<method::alt_text>>
      where S: Into<Cow<'a, str>>
    {
      ImageBuilder { image_url: self.image_url,
                     alt_text: Some(alt_text.into()),
                     state: PhantomData::<_> }
    }
  }

  impl<'a> ImageBuilder<'a, Set<method::image_url>, Set<method::alt_text>> {
    /// All done building, now give me a darn image element!
    ///
    /// > `no method name 'build' found for struct 'ImageBuilder<...>'`?
    /// Make sure all required setter methods have been called. See docs for `ImageBuilder`.
    ///
    /// ```compile_fail
    /// use slack_blocks::elems::Image;
    ///
    /// let foo = Image::builder().build(); // Won't compile!
    /// ```
    ///
    /// ```
    /// use slack_blocks::{compose::Opt, elems::Image};
    ///
    /// let foo = Image::builder().image_url("https://foo.com/bar.png")
    ///                           .alt_text("pic of bar")
    ///                           .build();
    /// ```
    pub fn build(self) -> Image<'a> {
      Image { image_url: self.image_url.unwrap(),
              alt_text: self.alt_text.unwrap() }
    }
  }
}
