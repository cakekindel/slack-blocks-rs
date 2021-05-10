use serde::{Deserialize, Serialize};

use crate::convert;
use crate::val_helpr::ValidationResult;

pub mod button;
pub use button::Contents as Button;

pub mod select;
pub use select::Select;

/// # Block Elements - interactive components
/// [slack api docs 🔗](https://api.slack.com/reference/block-kit/block-elements)
///
/// Block elements can be used inside of `section`, `context`, and `actions` [layout blocks 🔗].
/// Inputs can only be used inside of `input` blocks.
///
/// Our overview of [app surfaces that support Block Kit 🔗] shows you where those blocks might be relevant.
///
/// Finally, our [handling user interactivity guide 🔗] will help you prepare your app to allow
/// for the use of the interactive components listed below.
///
/// [app surfaces that support Block Kit 🔗]: https://api.slack.com/messaging/composing/layouts
/// [handling user interactivity guide 🔗]: https://api.slack.com/interactivity/handling
/// [layout blocks 🔗]: https://api.slack.com/reference/block-kit/blocks
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BlockElement<'a> {
    Button(Button),
    Checkboxes,
    DatePicker,
    Image,
    MultiSelect,
    OverflowMenu,
    PlainInput,
    RadioButtons,
    // Select
    #[serde(rename = "channels_select")]
    SelectPublicChannel(select::PublicChannel<'a>),
}

impl<'a> BlockElement<'a> {
    pub fn validate(&self) -> ValidationResult {
        match self {
            Self::Button(cts) => cts.validate(),
            Self::SelectPublicChannel(cts) => cts.validate(),
            rest => todo!("validation not implemented for {:?}", rest),
        }
    }
}

convert!(impl From<Button> for BlockElement<'static> => |b| BlockElement::Button(b));

convert!(impl<'a> From<Select<'a>> for BlockElement<'a>
    => |s| match s {
        Select::PublicChannel(s) => s.into(),
        _ => todo!()
    }
);

use select::PublicChannel as SelectPublicChannel;
convert!(impl<'a> From<SelectPublicChannel<'a>> for BlockElement<'a> => |s| BlockElement::SelectPublicChannel(s));
