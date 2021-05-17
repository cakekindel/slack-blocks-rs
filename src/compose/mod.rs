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
pub enum OptOrOptGroup<'a, M> {
  Opt(Opt<'a, M>),
  OptGroup(OptGroup<'a, M>),
}

crate::convert!(impl<'a, M> From<Opt<'a, M>> for OptOrOptGroup<'a, M> => |o| OptOrOptGroup::Opt(o));
crate::convert!(impl<'a, M> From<OptGroup<'a, M>> for OptOrOptGroup<'a, M> => |o| OptOrOptGroup::OptGroup(o));
