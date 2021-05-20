use serde::{Deserialize, Serialize};

use crate::convert;

pub mod actions;
pub use actions::Contents as Actions;

pub mod context;
pub use context::Contents as Context;

pub mod file;
pub use file::Contents as File;

pub mod image;
pub use image::Contents as Image;

pub mod input;
pub use input::Contents as Input;

pub mod section;
pub use section::Contents as Section;

type ValidationResult = Result<(), validator::ValidationErrors>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Block<'a> {
  Section(Section),

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

  Image(Image),

  Actions(Actions<'a>),

  Context(Context<'a>),

  Input(Input<'a>),

  File(File),
}

use std::fmt;

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

convert!(impl<'a> From<Actions<'a>> for Block<'a>      => |a| Block::Actions(a));
convert!(impl<'a> From<Input<'a>>   for Block<'a>      => |a| Block::Input(a));
convert!(impl     From<Section>     for Block<'static> => |a| Block::Section(a));
convert!(impl     From<Image>       for Block<'static> => |a| Block::Image(a));
convert!(impl<'a> From<Context<'a>> for Block<'a>      => |a| Block::Context(a));
convert!(impl     From<File>        for Block<'static> => |a| Block::File(a));
