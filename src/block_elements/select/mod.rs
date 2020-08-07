use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::text;
use crate::convert;

mod builder;
pub use builder::SelectBuilder;

mod public_channel;
pub use public_channel::PublicChannel as PublicChannelSelect;

mod select_ty_value {
    pub const PUBLIC_CHANNEL: &'static str = "users_select";
}

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
#[serde(tag = "type")]
pub enum Contents<'a> {
    Static(Static),
    External(External),
    User(User),
    Conversation(Conversation),
    PublicChannel(PublicChannelSelect<'a>),
}

impl<'a> Contents<'a> {
    pub fn from_placeholder_and_action_id(
        placeholder: impl Into<text::Plain>,
        action_id: impl Into<Cow<'a, str>>
    ) -> SelectBuilder<'a> {
        SelectBuilder::from_placeholder_and_action_id(placeholder, action_id)
    }
}

convert!(impl From<User> for Contents<'static> => |u| Contents::User(u));
convert!(impl From<Static> for Contents<'static> => |s| Contents::Static(s));
convert!(impl From<External> for Contents<'static> => |e| Contents::External(e));
convert!(impl From<Conversation> for Contents<'static> => |e| Contents::Conversation(e));
convert!(impl<'_> From<PublicChannelSelect> for Contents => |e| Contents::PublicChannel(e));

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

