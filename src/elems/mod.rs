//! # Block Elements - interactive components
//! [slack api docs 🔗](https://api.slack.com/reference/block-kit/block-elements)
//!
//! Block elements can be used inside of `section`, `context`, and `actions` [layout blocks 🔗].
//! Inputs can only be used inside of `input` blocks.
//!
//! Our overview of [app surfaces that support Block Kit 🔗] shows you where those blocks might be relevant.
//!
//! Finally, our [handling user interactivity guide 🔗] will help you prepare your app to allow
//! for the use of the interactive components listed below.
//!
//! [app surfaces that support Block Kit 🔗]: https://api.slack.com/messaging/composing/layouts
//! [handling user interactivity guide 🔗]: https://api.slack.com/interactivity/handling
//! [layout blocks 🔗]: https://api.slack.com/reference/block-kit/blocks

use serde::{Deserialize, Serialize};

use crate::{convert, val_helpr::ValidationResult};

#[doc(inline)]
pub mod button;
#[doc(inline)]
pub mod checkboxes;
#[doc(inline)]
pub mod date_picker;
#[doc(inline)]
pub mod image;
#[doc(inline)]
pub mod overflow;
#[doc(inline)]
pub mod radio;
#[doc(inline)]
pub mod select;
#[doc(inline)]
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
  /// # Button Element
  Button(Button<'a>),
  /// # Checkboxes Element
  Checkboxes(Checkboxes<'a>),
  /// # Image Element
  Image(Image<'a>),

  /// # DatePicker Element
  #[serde(rename = "datepicker")]
  DatePicker(DatePicker<'a>),

  /// # Overflow Element
  #[serde(rename = "overflow_menu")]
  Overflow(Overflow<'a>),

  /// # Radio Button Group
  RadioButtons(Radio<'a>),

  /// # Text Input Element
  #[serde(rename = "plain_text_input")]
  TextInput(TextInput<'a>),

  /// # Select a public channel
  #[serde(rename = "channels_select")]
  SelectPublicChannel(select::PublicChannel<'a>),

  /// # Select any conversation (DM, Group DM, Public Channel, Private Channel)
  #[serde(rename = "conversations_select")]
  SelectConversation(select::Conversation<'a>),

  /// # Select a user from the workspace
  #[serde(rename = "users_select")]
  SelectUser(select::User<'a>),

  /// # Select options loaded from an external data source
  #[serde(rename = "external_select")]
  SelectExternal(select::External<'a>),

  /// # Select options configured by your application
  #[serde(rename = "static_select")]
  SelectStatic(select::Static<'a>),

  /// # Select multiple options configured by your application
  #[serde(rename = "multi_static_select")]
  MultiSelectStatic(select::multi::Static<'a>),

  /// # Select multiple users from the workspace
  #[serde(rename = "multi_user_select")]
  MultiSelectUser(select::multi::User<'a>),

  /// # Select multiple options loaded from an external data source
  #[serde(rename = "multi_external_select")]
  MultiSelectExternal(select::multi::External<'a>),

  /// # Select multiple conversations (DM, Group DM, Public Channel, Private Channel)
  #[serde(rename = "multi_conversations_select")]
  MultiSelectConversation(select::multi::Conversation<'a>),

  /// # Select multiple conversations (DM, Group DM, Public Channel, Private Channel)
  #[serde(rename = "multi_channel_select")]
  MultiSelectPublicChannel(select::multi::PublicChannel<'a>),
}

impl<'a> BlockElement<'a> {
  /// Validate that this block element agrees with Slack's model requirements.
  ///
  /// ```
  /// use slack_blocks::elems::{BlockElement, Button};
  ///
  /// let text = std::iter::repeat('a').take(76).collect::<String>();
  /// let btn = Button::from_text_and_action_id(text, "");
  ///
  /// let elem = BlockElement::from(btn);
  ///
  /// assert!(matches!(elem.validate(), Err(_)))
  /// ```
  pub fn validate(&self) -> ValidationResult {
    use BlockElement::*;

    match self {
      | Button(cts) => cts.validate(),
      | SelectPublicChannel(cts) => cts.validate(),
      | SelectConversation(cts) => cts.validate(),
      | SelectUser(cts) => cts.validate(),
      | SelectExternal(cts) => cts.validate(),
      | SelectStatic(cts) => cts.validate(),
      | MultiSelectPublicChannel(cts) => cts.validate(),
      | MultiSelectConversation(cts) => cts.validate(),
      | MultiSelectUser(cts) => cts.validate(),
      | MultiSelectExternal(cts) => cts.validate(),
      | MultiSelectStatic(cts) => cts.validate(),
      | RadioButtons(cts) => cts.validate(),
      | Overflow(cts) => cts.validate(),
      | Checkboxes(cts) => cts.validate(),
      | Image(cts) => cts.validate(),
      | DatePicker(cts) => cts.validate(),
      | TextInput(cts) => cts.validate(),
    }
  }
}

convert!(impl<'a> From<Button<'a>> for BlockElement<'a> => |b| BlockElement::Button(b));
convert!(impl<'a> From<Radio<'a>> for BlockElement<'a> => |b| BlockElement::RadioButtons(b));
convert!(impl<'a> From<TextInput<'a>> for BlockElement<'a> => |t| BlockElement::TextInput(t));
convert!(impl<'a> From<Overflow<'a>> for BlockElement<'a> => |t| BlockElement::Overflow(t));
convert!(impl<'a> From<DatePicker<'a>> for BlockElement<'a> => |t| BlockElement::DatePicker(t));
convert!(impl<'a> From<Checkboxes<'a>> for BlockElement<'a> => |t| BlockElement::Checkboxes(t));
convert!(impl<'a> From<Image<'a>> for BlockElement<'a> => |t| BlockElement::Image(t));

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

convert!(impl<'a> From<select::multi::Static<'a>> for BlockElement<'a> => |s| BlockElement::MultiSelectStatic(s));
convert!(impl<'a> From<select::multi::User<'a>> for BlockElement<'a> => |s| BlockElement::MultiSelectUser(s));
convert!(impl<'a> From<select::multi::Conversation<'a>> for BlockElement<'a> => |s| BlockElement::MultiSelectConversation(s));
convert!(impl<'a> From<select::multi::External<'a>> for BlockElement<'a> => |s| BlockElement::MultiSelectExternal(s));
