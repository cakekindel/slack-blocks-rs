use std::borrow::Cow;
use serde::{Deserialize, Serialize};

use crate::text;
use crate::compose::Confirm;

/// # Public Channel Select
/// [slack api docs 🔗](https://api.slack.com/reference/block-kit/block-elements#channel_select)
///
/// This select menu will populate its options with a list of
/// public channels visible to the current user in the active workspace.
#[derive(Clone, Default, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct PublicChannel<'a> {
    placeholder: text::Plain,
    action_id: Cow<'a, str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<Confirm>,

    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<Cow<'a, str>>,
}

impl<'a> PublicChannel<'a> {
    pub fn from_placeholder_and_action_id(
        placeholder: impl Into<text::Plain>,
        action_id: impl Into<Cow<'a, str>>
    ) -> Self {
        Self {
            placeholder: placeholder.into(),
            action_id: action_id.into(),
            confirm: None,
            user_id: None,
        }
    }

    pub fn with_confirm(mut self, confirm: Confirm) -> Self {
        self.confirm = Some(confirm);
        self
    }

    pub fn with_initial_user(mut self, user_id: impl Into<Cow<'a, str>>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }
}
