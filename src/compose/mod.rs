use serde::{Deserialize, Serialize};

pub mod text;
pub use text::Text;

pub mod opt;
pub use opt::Opt;

pub mod opt_group;
pub use opt_group::OptGroup;

pub mod confirm;
pub use confirm::{Confirm, ConfirmStyle};

pub mod conversation_filter;
pub use conversation_filter::ConversationFilter;

/// An Option or Option Group
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
#[serde(untagged)]
pub enum OptOrOptGroup<M> {
  Opt(Opt<M>),
  OptGroup(OptGroup<M>),
}

crate::convert!(impl<M> From<Opt<M>> for OptOrOptGroup<M> => |o| OptOrOptGroup::Opt(o));
crate::convert!(impl<M> From<OptGroup<M>> for OptOrOptGroup<M> => |o| OptOrOptGroup::OptGroup(o));
