//! # Select Menu Element
//!
//! A select menu, just as with a standard HTML `<select>` tag,
//! creates a drop down menu with a list of options for a user to choose.
//!
//! The select menu also includes type-ahead functionality, where a user can type
//! a part or all of an option string to filter the list.
//!
//! To use interactive components, you will need to make some changes to prepare your app.
//! Read our [guide to enabling interactivity 🔗].
//!
//! [Select Menu Element 🔗]: https://api.slack.com/reference/block-kit/block-elements#select
//! [guide to enabling interactivity 🔗]: https://api.slack.com/interactivity/handling

use std::borrow::Cow;

use crate::{convert, text};

mod builder;
#[doc(inline)]
pub mod multi;

#[doc(inline)]
pub mod conversation;
#[doc(inline)]
pub mod external;
#[doc(inline)]
pub mod public_channel;
#[doc(inline)]
pub mod static_;
#[doc(inline)]
pub mod user;

#[doc(inline)]
pub use builder::SelectBuilder;
#[doc(inline)]
pub use conversation::Conversation;
#[doc(inline)]
pub use external::External;
#[doc(inline)]
pub use public_channel::PublicChannel;
#[doc(inline)]
pub use static_::Static;
#[doc(inline)]
pub use user::User;

/// # Select Menu Element
///
/// A select menu, just as with a standard HTML `<select>` tag,
/// creates a drop down menu with a list of options for a user to choose.
///
/// The select menu also includes type-ahead functionality, where a user can type
/// a part or all of an option string to filter the list.
///
/// To use interactive components, you will need to make some changes to prepare your app.
/// Read our [guide to enabling interactivity 🔗].
///
/// [Select Menu Element 🔗]: https://api.slack.com/reference/block-kit/block-elements#select
/// [guide to enabling interactivity 🔗]: https://api.slack.com/interactivity/handling
#[deprecated(since = "0.16.6",
             note = "enum doesn't really provide value as wrapper type and is confusing. use select types directly and slack_blocks::BlockElement instead.")]
#[derive(Clone, Debug, Hash, PartialEq)]
pub enum Select<'a> {
  /// Static
  Static(Static<'a>),
  /// External
  External(External<'a>),
  /// User
  User(User<'a>),
  /// Conversation
  Conversation(Conversation<'a>),
  /// PublicChannel
  PublicChannel(PublicChannel<'a>),
}

impl<'a> Select<'a> {
  /// Construct a Select block elem from required parts
  ///
  /// # Arguments
  /// - `placeholder`: A plain_text [text object 🔗] that defines the placeholder text shown on the menu.
  ///                  Maximum length for the text in this field is 150 characters.
  /// - `action_id`: An identifier for the action triggered when a menu option is selected.
  ///                You can use this when you receive an interaction payload to identify the source of the action.
  ///                Should be unique among all other action_ids in the containing block.
  ///                Maximum length for this field is 255 characters.
  ///
  /// [text objects 🔗]: https://api.slack.com/reference/messaging/composition-objects#text
  /// # Example
  /// ```
  /// use slack_blocks::{blocks, blocks::Actions, elems, compose::text};
  ///
  /// # use std::error::Error;
  /// # pub fn main() -> Result<(), Box<dyn Error>> {
  /// let fields = vec![
  ///     text::Plain::from("Left column"),
  ///     text::Plain::from("Right column"),
  /// ];
  ///
  /// let select = elems::Select::from_placeholder_and_action_id("Pick a channel!", "1234")
  ///                                                     .choose_from_public_channels();
  ///
  /// let block = Actions::builder().element(select).build();
  ///
  /// // < send `block` to slack API >
  /// # Ok(())
  /// # }
  /// ```
  pub fn from_placeholder_and_action_id(placeholder: impl Into<text::Plain>,
                                        action_id: impl Into<Cow<'a, str>>)
                                        -> SelectBuilder<'a> {
    SelectBuilder::from_placeholder_and_action_id(placeholder, action_id)
  }
}

convert!(impl<'a> From<User<'a>> for Select<'a> => |u| Select::User(u));
convert!(impl<'a> From<Static<'a>> for Select<'a> => |s| Select::Static(s));
convert!(impl<'a> From<External<'a>> for Select<'a> => |e| Select::External(e));
convert!(impl<'a> From<Conversation<'a>> for Select<'a> => |e| Select::Conversation(e));
convert!(impl<'a> From<PublicChannel<'a>> for Select<'a> => |e| Select::PublicChannel(e));

/// Marker structs for whether users can select one or many list items.
///
/// Used to inform builder structs what kind of select element to build.
pub mod select_kind {
  /// Users can select many items from the list.
  #[derive(Copy, Clone, Debug)]
  pub struct Multi;

  /// Users can select one item from the list.
  #[derive(Copy, Clone, Debug)]
  pub struct Single;
}

mod validate {
  use crate::{text, val_helpr::*};

  pub(super) fn placeholder(text: &text::Text) -> ValidatorResult {
    below_len("Select Placeholder text", 150, text.as_ref())
  }
}
