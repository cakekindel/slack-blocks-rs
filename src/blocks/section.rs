//! # Section Block
//!
//! _[slack api docs 🔗]_
//!
//! Available in surfaces:
//!  - [modals 🔗]
//!  - [messages 🔗]
//!  - [home tabs 🔗]
//!
//! A `section` is one of the most flexible blocks available -
//! it can be used as a simple text block,
//! in combination with text fields,
//! or side-by-side with any of the available [block elements 🔗]
//!
//! [slack api docs 🔗]: https://api.slack.com/reference/block-kit/blocks#section
//! [modals 🔗]: https://api.slack.com/surfaces/modals
//! [messages 🔗]: https://api.slack.com/surfaces/messages
//! [home tabs 🔗]: https://api.slack.com/surfaces/tabs
//! [block elements 🔗]: https://api.slack.com/reference/messaging/block-elements

use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{compose::text, elems::BlockElement, val_helpr::ValidationResult};

/// # Section Block
///
/// _[slack api docs 🔗]_
///
/// Available in surfaces:
///  - [modals 🔗]
///  - [messages 🔗]
///  - [home tabs 🔗]
///
/// A `section` is one of the most flexible blocks available -
/// it can be used as a simple text block,
/// in combination with text fields,
/// or side-by-side with any of the available [block elements 🔗]
///
/// [slack api docs 🔗]: https://api.slack.com/reference/block-kit/blocks#section
/// [modals 🔗]: https://api.slack.com/surfaces/modals
/// [messages 🔗]: https://api.slack.com/surfaces/messages
/// [home tabs 🔗]: https://api.slack.com/surfaces/tabs
/// [block elements 🔗]: https://api.slack.com/reference/messaging/block-elements
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
#[cfg_attr(feature = "validation", derive(Validate))]
pub struct Section<'a> {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[cfg_attr(feature = "validation", validate(custom = "validate::fields"))]
  fields: Option<Cow<'a, [text::Text]>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[cfg_attr(feature = "validation", validate(custom = "validate::text"))]
  text: Option<text::Text>,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[cfg_attr(feature = "validation", validate(custom = "validate::block_id"))]
  block_id: Option<Cow<'a, str>>,

  /// One of the available [element objects 🔗][element_objects].
  ///
  /// [element_objects]: https://api.slack.com/reference/messaging/block-elements
  #[serde(skip_serializing_if = "Option::is_none")]
  accessory: Option<BlockElement<'a>>,
}

impl<'a> Section<'a> {
  /// Build a new section block
  ///
  /// For example, see `blocks::section::build::SectionBuilder`.
  pub fn builder() -> build::SectionBuilderInit<'a> {
    build::SectionBuilderInit::new()
  }

  /// Validate that this Section block agrees with Slack's model requirements
  ///
  /// # Errors
  /// - If `fields` contains more than 10 fields
  /// - If one of `fields` longer than 2000 chars
  /// - If `text` longer than 3000 chars
  /// - If `block_id` longer than 255 chars
  ///
  /// # Example
  /// ```
  /// use slack_blocks::{blocks, compose::text};
  ///
  /// let long_string = std::iter::repeat(' ').take(256).collect::<String>();
  ///
  /// let block = blocks::Section::builder().text(text::Plain::from("file_id"))
  ///                                       .block_id(long_string)
  ///                                       .build();
  ///
  /// assert_eq!(true, matches!(block.validate(), Err(_)));
  /// ```
  pub fn validate(&self) -> ValidationResult {
    Validate::validate(self)
  }
}

/// Section block builder
pub mod build {
  use std::marker::PhantomData;

  use super::*;
  use crate::build::*;

  /// Compile-time markers for builder methods
  #[allow(non_camel_case_types)]
  pub mod method {
    /// SectionBuilder.text
    #[derive(Clone, Copy, Debug)]
    pub struct text;
  }

  /// Initial state for `SectionBuilder`
  pub type SectionBuilderInit<'a> =
    SectionBuilder<'a, RequiredMethodNotCalled<method::text>>;

  /// Build an Section block
  ///
  /// Allows you to construct safely, with compile-time checks
  /// on required setter methods.
  ///
  /// # Required Methods
  /// `SectionBuilder::build()` is only available if these methods have been called:
  ///  - `text` **or** `field(s)`, both may be called.
  ///
  /// # Example
  /// ```
  /// use slack_blocks::{blocks::Section,
  ///                    elems::Image,
  ///                    text,
  ///                    text::ToSlackPlaintext};
  ///
  /// let block =
  ///   Section::builder().text("foo".plaintext())
  ///                     .field("bar".plaintext())
  ///                     .field("baz".plaintext())
  ///                     // alternatively:
  ///                     .fields(vec!["bar".plaintext(),
  ///                                  "baz".plaintext()]
  ///                                  .into_iter()
  ///                                  .map(text::Text::from)
  ///                     )
  ///                     .accessory(Image::builder().image_url("foo.png")
  ///                                                .alt_text("pic of foo")
  ///                                                .build())
  ///                     .build();
  /// ```
  #[derive(Debug)]
  pub struct SectionBuilder<'a, Text> {
    accessory: Option<BlockElement<'a>>,
    text: Option<text::Text>,
    fields: Option<Vec<text::Text>>,
    block_id: Option<Cow<'a, str>>,
    state: PhantomData<Text>,
  }

  impl<'a, E> SectionBuilder<'a, E> {
    /// Create a new SectionBuilder
    pub fn new() -> Self {
      Self { accessory: None,
             text: None,
             fields: None,
             block_id: None,
             state: PhantomData::<_> }
    }

    /// Set `accessory` (Optional)
    pub fn accessory<B>(mut self, acc: B) -> Self
      where B: Into<BlockElement<'a>>
    {
      self.accessory = Some(acc.into());
      self
    }

    /// Add `text` (**Required: this or `field(s)`**)
    ///
    /// The text for the block, in the form of a [text object 🔗].
    ///
    /// Maximum length for the text in this field is 3000 characters.
    ///
    /// [text object 🔗]: https://api.slack.com/reference/messaging/composition-objects#text
    pub fn text<T>(self, text: T) -> SectionBuilder<'a, Set<method::text>>
      where T: Into<text::Text>
    {
      SectionBuilder { accessory: self.accessory,
                       text: Some(text.into()),
                       fields: self.fields,
                       block_id: self.block_id,
                       state: PhantomData::<_> }
    }

    /// Set `fields` (**Required: this or `text`**)
    ///
    /// A collection of [text objects 🔗].
    ///
    /// Any text objects included with fields will be
    /// rendered in a compact format that allows for
    /// 2 columns of side-by-side text.
    ///
    /// Maximum number of items is 10.
    ///
    /// Maximum length for the text in each item is 2000 characters.
    ///
    /// [text objects 🔗]: https://api.slack.com/reference/messaging/composition-objects#text
    pub fn fields<I>(self, fields: I) -> SectionBuilder<'a, Set<method::text>>
      where I: IntoIterator<Item = text::Text>
    {
      SectionBuilder { accessory: self.accessory,
                       text: self.text,
                       fields: Some(fields.into_iter().collect()),
                       block_id: self.block_id,
                       state: PhantomData::<_> }
    }

    /// Append a single field to `fields`.
    pub fn field<T>(mut self, text: T) -> SectionBuilder<'a, Set<method::text>>
      where T: Into<text::Text>
    {
      let mut fields = self.fields.take().unwrap_or_default();
      fields.push(text.into());

      self.fields(fields)
    }

    /// XML macro children, appends `fields` to the Section.
    ///
    /// To set `text`, use the `text` attribute.
    /// ```
    /// use slack_blocks::{blocks::Section, blox::*, text, text::ToSlackPlaintext};
    ///
    /// let xml = blox! {
    ///   <section_block text={"Section".plaintext()}>
    ///     <text kind=plain>"Foo"</text>
    ///     <text kind=plain>"Bar"</text>
    ///   </section_block>
    /// };
    ///
    /// let equiv = Section::builder().text("Section".plaintext())
    ///                               .field("Foo".plaintext())
    ///                               .field("Bar".plaintext())
    ///                               .build();
    ///
    /// assert_eq!(xml, equiv);
    /// ```
    #[cfg(feature = "blox")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blox")))]
    pub fn child<T>(self, text: T) -> SectionBuilder<'a, Set<method::text>>
      where T: Into<text::Text>
    {
      self.field(text)
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

  impl<'a> SectionBuilder<'a, Set<method::text>> {
    /// All done building, now give me a darn actions block!
    ///
    /// > `no method name 'build' found for struct 'SectionBuilder<...>'`?
    /// Make sure all required setter methods have been called. See docs for `SectionBuilder`.
    ///
    /// ```compile_fail
    /// use slack_blocks::blocks::Section;
    ///
    /// let foo = Section::builder().build(); // Won't compile!
    /// ```
    ///
    /// ```
    /// use slack_blocks::{blocks::Section,
    ///                    compose::text::ToSlackPlaintext,
    ///                    elems::Image};
    ///
    /// let block =
    ///   Section::builder().text("foo".plaintext())
    ///                     .accessory(Image::builder().image_url("foo.png")
    ///                                                .alt_text("pic of foo")
    ///                                                .build())
    ///                     .build();
    /// ```
    pub fn build(self) -> Section<'a> {
      Section { text: self.text,
                fields: self.fields.map(|fs| fs.into()),
                accessory: self.accessory,
                block_id: self.block_id }
    }
  }
}

mod validate {
  use super::*;
  use crate::{compose::text,
              val_helpr::{below_len, ValidatorResult}};

  pub(super) fn text(text: &text::Text) -> ValidatorResult {
    below_len("Section.text", 3000, text.as_ref())
  }

  pub(super) fn block_id(text: &Cow<str>) -> ValidatorResult {
    below_len("Section.block_id", 255, text.as_ref())
  }

  pub(super) fn fields(texts: &Cow<[text::Text]>) -> ValidatorResult {
    below_len("Section.fields", 10, texts.as_ref()).and(
                                                        texts.iter()
                                                             .map(|text| {
                                                               below_len(
             "Section.fields",
             2000,
             text.as_ref())
                                                             })
                                                             .collect(),
    )
  }
}
