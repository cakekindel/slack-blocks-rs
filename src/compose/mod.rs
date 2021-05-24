//! # Composition Objects
//!
//! Composition objects can be used inside of [block elements 🔗] and certain [message payload 🔗] fields.
//!
//! They are simply common JSON object patterns that you'll encounter frequently when [building blocks 🔗] or [composing messages 🔗].
//!
//! [block elements 🔗]: https://api.slack.com/reference/block-kit/block-elements
//! [message payload 🔗]: https://api.slack.com/reference/messaging/payload
//! [building blocks 🔗]: https://api.slack.com/block-kit/building
//! [composing messages 🔗]: https://api.slack.com/messaging/composing

use serde::{Deserialize, Serialize};

#[doc(inline)]
pub mod confirm;
#[doc(inline)]
pub mod conversation_filter;
#[doc(inline)]
pub mod opt;
#[doc(inline)]
pub mod opt_group;
#[doc(inline)]
pub mod text;

#[doc(inline)]
pub use confirm::{Confirm, ConfirmStyle};
#[doc(inline)]
pub use conversation_filter::ConversationFilter;
#[doc(inline)]
pub use opt::Opt;
#[doc(inline)]
pub use opt_group::OptGroup;
#[doc(inline)]
pub use text::Text;

/// An Option or Option Group
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
#[serde(untagged)]
pub enum OptOrOptGroup<'a, T, U> {
  /// Option
  Opt(Opt<'a, T, U>),
  /// Option Group
  OptGroup(OptGroup<'a, T, U>),
}

crate::convert!(impl<'a, T, U> From<Opt<'a, T, U>> for OptOrOptGroup<'a, T, U> => |o| OptOrOptGroup::Opt(o));
crate::convert!(impl<'a, T, U> From<OptGroup<'a, T, U>> for OptOrOptGroup<'a, T, U> => |o| OptOrOptGroup::OptGroup(o));
