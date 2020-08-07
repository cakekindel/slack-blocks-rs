use serde::{Deserialize, Serialize};

use crate::convert;

pub mod actions;
pub use actions::Contents as Actions;
pub mod context;
pub mod file;
pub mod image;
pub mod input;
pub use input::Contents as Input;
pub mod section;

type ValidationResult = Result<(), validator::ValidationErrors>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Block<'a> {
    Section(section::Contents),

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

    Image(image::Contents),

    Actions(actions::Contents<'a>),

    Context(context::Contents),

    Input(input::Contents<'a>),

    File(file::Contents),
}

use std::fmt;

impl fmt::Display for Block<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = match self {
            Block::Section { .. } => "Section",
            Block::Divider => "Divider",
            Block::Image { .. } => "Image",
            Block::Actions { .. } => "Actions",
            Block::Context { .. } => "Context",
            Block::Input { .. } => "Input",
            Block::File { .. } => "File",
        };

        write!(f, "{}", kind)
    }
}

impl<'a> Block<'a> {
    pub fn validate(&self) -> ValidationResult {
        use Block::*;

        match self {
            Section(contents) => contents.validate(),
            Image(contents) => contents.validate(),
            Actions(contents) => contents.validate(),
            Context(contents) => contents.validate(),
            Input(contents) => contents.validate(),
            File(contents) => contents.validate(),
            Divider => Ok(())
        }
    }
}

convert!(impl<'_> From<Actions> for Block => |a| Block::Actions(a));
convert!(impl<'_> From<Input>   for Block => |a| Block::Input(a));
convert!(impl From<section::Contents> for Block<'static> => |a| Block::Section(a));
convert!(impl From<image::Contents>   for Block<'static> => |a| Block::Image(a));
convert!(impl From<context::Contents> for Block<'static> => |a| Block::Context(a));
convert!(impl From<file::Contents>    for Block<'static> => |a| Block::File(a));
