//! # Context Block
//!
//! _[slack api docs 🔗][context_docs]_
//!
//! Displays message context, which can include both images and text.
//!
//! [context_docs]: https://api.slack.com/reference/block-kit/blocks#context

use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{convert,
            elems::{BlockElement, Image},
            text,
            val_helpr::ValidationResult};

/// # Context Block
///
/// _[slack api docs 🔗][context_docs]_
///
/// Displays message context, which can include both images and text.
///
/// [context_docs]: https://api.slack.com/reference/block-kit/blocks#context
#[derive(Clone,
           Debug,
           Default,
           Deserialize,
           Hash,
           PartialEq,
           Serialize,
           Validate)]
pub struct Context<'a> {
  #[validate(length(max = 10))]
  elements: Vec<ImageOrText<'a>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[validate(custom = "super::validate_block_id")]
  block_id: Option<Cow<'a, str>>,
}

impl<'a> Context<'a> {
  /// Build a new Context block.
  ///
  /// For example, see docs for ContextBuilder.
  pub fn builder() -> build::ContextBuilderInit<'a> {
    build::ContextBuilderInit::new()
  }

  /// Validate that this Context block agrees with Slack's model requirements
  ///
  /// # Errors
  /// - If `with_block_id` was called with a block id longer
  ///     than 255 chars
  /// - If `from_elements`, `from_context_elements`, or `with_element` was called with
  ///     more than 10 objects
  ///
  /// # Example
  /// ```
  /// use slack_blocks::{blocks::Context, text::ToSlackPlaintext};
  ///
  /// let long_string = std::iter::repeat(' ').take(256).collect::<String>();
  ///
  /// let block = Context::builder().element("foo".plaintext())
  ///                               .block_id(long_string)
  ///                               .build();
  ///
  /// assert_eq!(true, matches!(block.validate(), Err(_)));
  /// ```
  pub fn validate(&self) -> ValidationResult {
    Validate::validate(self)
  }
}

/// Context block builder
pub mod build {
  use std::marker::PhantomData;

  use super::*;
  use crate::build::*;

  /// Compile-time markers for builder methods
  #[allow(non_camel_case_types)]
  pub mod method {
    /// ContextBuilder.elements
    #[derive(Clone, Copy, Debug)]
    pub struct elements;
  }

  /// Initial state for `ContextBuilder`
  pub type ContextBuilderInit<'a> =
    ContextBuilder<'a, RequiredMethodNotCalled<method::elements>>;

  /// Build an Context block
  ///
  /// Allows you to construct safely, with compile-time checks
  /// on required setter methods.
  ///
  /// # Required Methods
  /// `ContextBuilder::build()` is only available if these methods have been called:
  ///  - `element`
  ///
  /// # Example
  /// ```
  /// use slack_blocks::{blocks::Context, elems::Image, text::ToSlackPlaintext};
  ///
  /// let block = Context::builder().element("foo".plaintext())
  ///                               .element(Image::builder().image_url("foo.png")
  ///                                                        .alt_text("pic of foo")
  ///                                                        .build())
  ///                               .build();
  /// ```
  #[derive(Debug)]
  pub struct ContextBuilder<'a, Elements> {
    elements: Option<Vec<ImageOrText<'a>>>,
    block_id: Option<Cow<'a, str>>,
    state: PhantomData<Elements>,
  }

  impl<'a, E> ContextBuilder<'a, E> {
    /// Create a new ContextBuilder
    pub fn new() -> Self {
      Self { elements: None,
             block_id: None,
             state: PhantomData::<_> }
    }

    /// Alias of `element` for appending an element with an XML child.
    #[cfg(feature = "xml")]
    #[cfg_attr(docsrs, doc(cfg(feature = "xml")))]
    pub fn child<El>(self,
                     element: El)
                     -> ContextBuilder<'a, Set<method::elements>>
      where El: Into<ImageOrText<'a>>
    {
      self.element(element)
    }

    /// Add an `element` (**Required**, can be called many times)
    ///
    /// A composition object; Must be image elements or text objects.
    ///
    /// Maximum number of items is 10.
    pub fn element<El>(self,
                       element: El)
                       -> ContextBuilder<'a, Set<method::elements>>
      where El: Into<ImageOrText<'a>>
    {
      let mut elements = self.elements.unwrap_or_default();
      elements.push(element.into());

      ContextBuilder { block_id: self.block_id,
                       elements: Some(elements),
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

  impl<'a> ContextBuilder<'a, Set<method::elements>> {
    /// All done building, now give me a darn actions block!
    ///
    /// > `no method name 'build' found for struct 'ContextBuilder<...>'`?
    /// Make sure all required setter methods have been called. See docs for `ContextBuilder`.
    ///
    /// ```compile_fail
    /// use slack_blocks::blocks::Context;
    ///
    /// let foo = Context::builder().build(); // Won't compile!
    /// ```
    ///
    /// ```
    /// use slack_blocks::{blocks::Context,
    ///                    compose::text::ToSlackPlaintext,
    ///                    elems::Image};
    ///
    /// let block = Context::builder().element("foo".plaintext())
    ///                               .element(Image::builder().image_url("foo.png")
    ///                                                        .alt_text("pic of foo")
    ///                                                        .build())
    ///                               .build();
    /// ```
    pub fn build(self) -> Context<'a> {
      Context { elements: self.elements.unwrap(),
                block_id: self.block_id }
    }
  }
}

impl<'a> From<Vec<ImageOrText<'a>>> for Context<'a> {
  fn from(elements: Vec<ImageOrText<'a>>) -> Self {
    Self { elements,
           ..Default::default() }
  }
}

/// The Composition objects supported by this block
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
#[allow(missing_docs)]
#[serde(untagged)]
pub enum ImageOrText<'a> {
  Text(text::Text),
  Image(BlockElement<'a>),
}

convert!(impl From<text::Text> for ImageOrText<'static> => |txt| ImageOrText::Text(txt));
convert!(impl<'a> From<Image<'a>> for ImageOrText<'a> => |i| ImageOrText::Image(BlockElement::from(i)));
convert!(impl From<text::Plain> for ImageOrText<'static> => |t| text::Text::from(t).into());
convert!(impl From<text::Mrkdwn> for ImageOrText<'static> => |t| text::Text::from(t).into());
