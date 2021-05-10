use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::convert;
use crate::text;

mod builder;
pub use builder::SelectBuilder;

mod public_channel;
pub use public_channel::PublicChannel;

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
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub enum Select<'a> {
    Static(Static),
    External(External),
    User(User),
    Conversation(Conversation),
    #[serde(rename = "channels_select")]
    PublicChannel(PublicChannel<'a>),
}

impl<'a> Select<'a> {
    /// Construct a Select block element from required parts
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
    /// use slack_blocks::{blocks, blocks::actions::Contents as ActionsBlock, block_elements as element, compose::text};
    ///
    /// # use std::error::Error;
    /// # pub fn main() -> Result<(), Box<dyn Error>> {
    /// let fields = vec![
    ///     text::Plain::from("Left column"),
    ///     text::Plain::from("Right column"),
    /// ];
    ///
    /// let select: element::BlockElement = element::Select::from_placeholder_and_action_id("Pick a channel!", "1234")
    ///                                                     .choose_from_public_channels()
    ///                                                     .into();
    ///
    /// let block = ActionsBlock::from_elements(Some(select));
    ///
    /// // < send `block` to slack API >
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_placeholder_and_action_id(
        placeholder: impl Into<text::Plain>,
        action_id: impl Into<Cow<'a, str>>,
    ) -> SelectBuilder<'a> {
        SelectBuilder::from_placeholder_and_action_id(placeholder, action_id)
    }
}

convert!(impl From<User> for Select<'static> => |u| Select::User(u));
convert!(impl From<Static> for Select<'static> => |s| Select::Static(s));
convert!(impl From<External> for Select<'static> => |e| Select::External(e));
convert!(impl From<Conversation> for Select<'static> => |e| Select::Conversation(e));
convert!(impl<'a> From<PublicChannel<'a>> for Select<'a> => |e| Select::PublicChannel(e));

/// ## Select menu with static options
/// [slack api docs 🔗](https://api.slack.com/reference/block-kit/block-elements#static_select)
///
/// This is the simplest form of select menu,
/// with a static list of options passed in when defining the element.
///
#[derive(Clone, Default, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Static {}

/// ## Select menu with external data source
/// [slack api docs 🔗](https://api.slack.com/reference/block-kit/block-elements#external_select)
///
/// This select menu will load its options from an external data source,
/// allowing for a dynamic list of options.
///
/// ### Setup
/// For a guide to set up your app to use this element type, go to the Slack
/// API section for [Select menu with external data source 🔗].
///
#[derive(Clone, Default, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct External {}

/// ## Select menu with user list
/// [slack api docs 🔗](https://api.slack.com/reference/block-kit/block-elements#users_select)
///
/// This select menu will populate its options with a list of
/// Slack users visible to the current user in the active workspace.
#[derive(Clone, Default, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct User {}

/// ## Select menu with conversations list
/// [slack api docs 🔗](https://api.slack.com/reference/block-kit/block-elements#conversation_select)
///
/// This select menu will populate its options with a list of public and private channels,
/// DMs, and MPIMs visible to the current user in the active workspace.
#[derive(Clone, Default, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Conversation {}

mod validate {
    use crate::text;
    use crate::val_helpr::*;

    pub fn placeholder(text: &text::Text) -> ValidatorResult {
        below_len("Select Placeholder text", 150, text.as_ref())
    }
}
