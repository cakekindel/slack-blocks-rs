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

#[doc(inline)]
pub mod actions;
#[doc(inline)]
pub use actions::Actions;

#[doc(inline)]
pub mod context;
#[doc(inline)]
pub use context::Context;

#[doc(inline)]
pub mod file;
#[doc(inline)]
pub use file::File;

#[doc(inline)]
pub mod image;
#[doc(inline)]
pub use image::Image;

#[doc(inline)]
pub mod input;
#[doc(inline)]
pub use input::Input;

#[doc(inline)]
pub mod section;
#[doc(inline)]
pub use section::Section;

type ValidationResult = Result<(), validator::ValidationErrors>;

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
#[derive(Serialize, Deserialize, Debug)]
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

  /// # File Block
  File(File<'a>),
}

impl fmt::Display for Block<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let kind = match self {
      | Block::Section { .. } => "Section",
      | Block::Divider => "Divider",
      | Block::Image { .. } => "Image",
      | Block::Actions { .. } => "Actions",
      | Block::Context { .. } => "Context",
      | Block::Input { .. } => "Input",
      | Block::File { .. } => "File",
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
  /// let img = Image::from_alt_text_and_url(long_string, "foo.com");
  ///
  /// assert!(matches!(img.validate(), Err(_)), "validation should fail!")
  /// ```
  pub fn validate(&self) -> ValidationResult {
    use Block::*;

    match self {
      | Section(contents) => contents.validate(),
      | Image(contents) => contents.validate(),
      | Actions(contents) => contents.validate(),
      | Context(contents) => contents.validate(),
      | Input(contents) => contents.validate(),
      | File(contents) => contents.validate(),
      | Divider => Ok(()),
    }
  }
}

convert!(impl<'a> From<Actions<'a>> for Block<'a> => |a| Block::Actions(a));
convert!(impl<'a> From<Input<'a>>   for Block<'a> => |a| Block::Input(a));
convert!(impl<'a> From<Section<'a>> for Block<'a> => |a| Block::Section(a));
convert!(impl<'a> From<Image<'a>>   for Block<'a> => |a| Block::Image(a));
convert!(impl<'a> From<Context<'a>> for Block<'a> => |a| Block::Context(a));
convert!(impl<'a> From<File<'a>>    for Block<'a> => |a| Block::File(a));

fn validate_block_id(id: &std::borrow::Cow<str>)
                     -> crate::val_helpr::ValidatorResult {
  crate::val_helpr::below_len("block_id", 255, id)
}
