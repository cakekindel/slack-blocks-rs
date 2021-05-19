use serde::{Deserialize, Serialize};

pub mod confirm;
pub mod conversation_filter;
pub mod opt;
pub mod opt_group;
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
  Opt(Opt<'a, T, U>),
  OptGroup(OptGroup<'a, T, U>),
}

crate::convert!(impl<'a, T, U> From<Opt<'a, T, U>> for OptOrOptGroup<'a, T, U> => |o| OptOrOptGroup::Opt(o));
crate::convert!(impl<'a, T, U> From<OptGroup<'a, T, U>> for OptOrOptGroup<'a, T, U> => |o| OptOrOptGroup::OptGroup(o));
