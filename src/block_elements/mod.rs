use serde::{Deserialize, Serialize};

use crate::{convert, val_helpr::ValidationResult};

pub mod button;
pub mod checkboxes;
pub mod date_picker;
pub mod image;
pub mod overflow;
pub mod radio;
pub mod select;
pub mod text_input;

#[doc(inline)]
pub use button::Button;
#[doc(inline)]
pub use checkboxes::Checkboxes;
#[doc(inline)]
pub use date_picker::DatePicker;
#[doc(inline)]
pub use image::Image;
#[doc(inline)]
pub use overflow::Overflow;
#[doc(inline)]
pub use radio::Radio;
#[doc(inline)]
pub use select::Select;
#[doc(inline)]
pub use text_input::TextInput;

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
  Checkboxes(Checkboxes<'a>),
  Image(Image<'a>),
  MultiSelect,

  #[serde(rename = "datepicker")]
  DatePicker(DatePicker<'a>),

  #[serde(rename = "overflow_menu")]
  Overflow(Overflow<'a>),

  RadioButtons(Radio<'a>),

  #[serde(rename = "plain_text_input")]
  TextInput(TextInput<'a>),

  #[serde(rename = "channels_select")]
  SelectPublicChannel(select::PublicChannel<'a>),

  #[serde(rename = "conversations_select")]
  SelectConversation(select::Conversation<'a>),

  #[serde(rename = "users_select")]
  SelectUser(select::User<'a>),

  #[serde(rename = "external_select")]
  SelectExternal(select::External<'a>),

  #[serde(rename = "static_select")]
  SelectStatic(select::Static<'a>),
}

impl<'a> BlockElement<'a> {
  pub fn validate(&self) -> ValidationResult {
    match self {
      | Self::Button(cts) => cts.validate(),
      | Self::SelectPublicChannel(cts) => cts.validate(),
      | Self::SelectConversation(cts) => cts.validate(),
      | Self::SelectUser(cts) => cts.validate(),
      | Self::SelectExternal(cts) => cts.validate(),
      | Self::SelectStatic(cts) => cts.validate(),
      | Self::RadioButtons(cts) => cts.validate(),
      | Self::Overflow(cts) => cts.validate(),
      | Self::Checkboxes(cts) => cts.validate(),
      | rest => todo!("validation not implemented for {:?}", rest),
    }
  }
}

convert!(impl From<Button> for BlockElement<'static> => |b| BlockElement::Button(b));
convert!(impl<'a> From<Radio<'a>> for BlockElement<'a> => |b| BlockElement::RadioButtons(b));
convert!(impl<'a> From<TextInput<'a>> for BlockElement<'a> => |t| BlockElement::TextInput(t));
convert!(impl<'a> From<Overflow<'a>> for BlockElement<'a> => |t| BlockElement::Overflow(t));
convert!(impl<'a> From<DatePicker<'a>> for BlockElement<'a> => |t| BlockElement::DatePicker(t));
convert!(impl<'a> From<Checkboxes<'a>> for BlockElement<'a> => |t| BlockElement::Checkboxes(t));

convert!(impl<'a> From<Select<'a>> for BlockElement<'a>
    => |s| match s {
        Select::PublicChannel(s) => s.into(),
        Select::Conversation(s) => s.into(),
        Select::User(s) => s.into(),
        Select::External(s) => s.into(),
        Select::Static(s) => s.into(),
    }
);

convert!(impl<'a> From<select::Static<'a>> for BlockElement<'a> => |s| BlockElement::SelectStatic(s));
convert!(impl<'a> From<select::External<'a>> for BlockElement<'a> => |s| BlockElement::SelectExternal(s));
convert!(impl<'a> From<select::PublicChannel<'a>> for BlockElement<'a> => |s| BlockElement::SelectPublicChannel(s));
convert!(impl<'a> From<select::Conversation<'a>> for BlockElement<'a> => |s| BlockElement::SelectConversation(s));
convert!(impl<'a> From<select::User<'a>> for BlockElement<'a> => |s| BlockElement::SelectUser(s));
