use std::borrow::Cow;

use crate::text;
use crate::compose::Confirm;

use super::{PublicChannel};

/// # Select Element Builder
/// Use to construct a Select element
/// and easily choose a data source
pub struct SelectBuilder<'a> {
    pub placeholder: text::Plain,
    pub action_id: Cow<'a, str>,
    pub confirm: Option<Confirm>
}

impl<'a> SelectBuilder<'a> {
    pub fn from_placeholder_and_action_id(
        placeholder: impl Into<text::Plain>,
        action_id: impl Into<Cow<'a, str>>
    ) -> Self {
        Self {
            placeholder: placeholder.into(),
            action_id: action_id.into(),
            confirm: None,
        }
    }

    pub fn with_confirm(mut self, confirm: Confirm) -> Self {
        self.confirm = Some(confirm);
        self
    }

    pub fn choose_from_public_channels(self) -> PublicChannel<'a> {
        let sel = PublicChannel::from_placeholder_and_action_id(
            self.placeholder,
            self.action_id,
        );

        match self.confirm {
            Some(confirm) => sel.with_confirm(confirm),
            None => sel
        }
    }
}
