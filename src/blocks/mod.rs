//! # Layout Blocks
//!
//! Blocks are a series of components that can be combined
//! to create visually rich and compellingly interactive messages.
//!
//! Read our guide to [building block layouts 🔗] to learn where and how to use each of these components.
//!
//! You can include up to 50 blocks in each message, and 100 blocks in modals or home tabs.
//!
//! [building block layouts 🔗]: https://api.slack.com/block-kit/building

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::convert;

pub mod actions;
#[doc(inline)]
pub use actions::Actions;

pub mod context;
#[doc(inline)]
pub use context::Context;

pub mod file;
#[doc(inline)]
pub use file::File;

pub mod image;
#[doc(inline)]
pub use image::Image;

pub mod input;
#[doc(inline)]
pub use input::Input;

pub mod section;
#[doc(inline)]
pub use section::Section;

pub mod header;
#[doc(inline)]
pub use header::Header;

pub mod rich_text;
#[doc(inline)]
pub use rich_text::RichText;

/// # Layout Blocks
///
/// Blocks are a series of components that can be combined
/// to create visually rich and compellingly interactive messages.
///
/// Read our guide to [building block layouts 🔗] to learn where and how to use each of these components.
///
/// You can include up to 50 blocks in each message, and 100 blocks in modals or home tabs.
///
/// [building block layouts 🔗]: https://api.slack.com/block-kit/building
#[derive(Hash, PartialEq, Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Block<'a> {
  /// # Section Block
  Section(Section<'a>),

  /// # Divider Block
  ///
  /// _[slack api docs 🔗][divider_docs]_
  ///
  /// A content divider, like an `<hr>`,
  /// to split up different blocks inside of a message.
  ///
  /// The divider block is nice and neat, requiring no fields.
  ///
  /// [divider_docs]: https://api.slack.com/reference/block-kit/blocks#divider
  Divider,

  /// # Image Block
  Image(Image<'a>),

  /// # Actions Block
  Actions(Actions<'a>),

  /// # Context Block
  Context(Context<'a>),

  /// # Input Block
  Input(Input<'a>),

  /// # Input Block
  Header(Header<'a>),

  /// # File Block
  File(File<'a>),

  /// # Rich Text Block
  RichText(RichText<'a>),
}

impl fmt::Display for Block<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let kind = match self {
      | Block::Header { .. } => "Header",
      | Block::Section { .. } => "Section",
      | Block::Divider => "Divider",
      | Block::Image { .. } => "Image",
      | Block::Actions { .. } => "Actions",
      | Block::Context { .. } => "Context",
      | Block::Input { .. } => "Input",
      | Block::File { .. } => "File",
      | Block::RichText { .. } => "RichText",
    };

    write!(f, "{}", kind)
  }
}

impl<'a> Block<'a> {
  /// Validate that this block agrees with Slack's model requirements.
  ///
  /// ```
  /// use slack_blocks::{blocks, blocks::Image};
  ///
  /// let long_string = std::iter::repeat('a').take(2001).collect::<String>();
  ///
  /// let img = Image::builder().src("foo.com").alt(long_string).build();
  ///
  /// assert!(matches!(img.validate(), Err(_)), "validation should fail!")
  /// ```
  #[cfg(feature = "validation")]
  #[cfg_attr(docsrs, doc(cfg(feature = "validation")))]
  pub fn validate(&self) -> crate::val_helpr::ValidationResult {
    use Block::*;

    match self {
      | Section(contents) => contents.validate(),
      | Image(contents) => contents.validate(),
      | Actions(contents) => contents.validate(),
      | Context(contents) => contents.validate(),
      | Input(contents) => contents.validate(),
      | Header(contents) => contents.validate(),
      | File(contents) => contents.validate(),
      | RichText(contents) => validator::Validate::validate(contents),
      | Divider => Ok(()),
    }
  }
}

convert!(impl<'a> From<RichText<'a>> for Block<'a> => Block::RichText);
convert!(impl<'a> From<Actions<'a>>  for Block<'a> => Block::Actions);
convert!(impl<'a> From<Input<'a>>    for Block<'a> => Block::Input);
convert!(impl<'a> From<Section<'a>>  for Block<'a> => Block::Section);
convert!(impl<'a> From<Image<'a>>    for Block<'a> => Block::Image);
convert!(impl<'a> From<Context<'a>>  for Block<'a> => Block::Context);
convert!(impl<'a> From<File<'a>>     for Block<'a> => Block::File);
convert!(impl<'a> From<Header<'a>>   for Block<'a> => Block::Header);

/// Error yielded when `TryFrom` is called on an unsupported block element.
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct UnsupportedElement<'a> {
  context: String,
  element: crate::elems::BlockElement<'a>,
}

impl<'a> std::fmt::Display for UnsupportedElement<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f,
           "(In {}) Block element not supported: {:#?}",
           self.context, self.element)
  }
}

impl<'a> std::error::Error for UnsupportedElement<'a> {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    None
  }
}

#[cfg(feature = "validation")]
fn validate_block_id(id: &std::borrow::Cow<str>)
                     -> crate::val_helpr::ValidatorResult {
  crate::val_helpr::below_len("block_id", 255, id)
}
