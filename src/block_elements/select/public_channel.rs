use std::borrow::Cow;
use validator::Validate;
use serde::{Deserialize, Serialize};

use crate::text;
use crate::compose::Confirm;
use crate::val_helpr::ValidationResult;

/// # Public Channel Select
/// [slack api docs 🔗](https://api.slack.com/reference/block-kit/block-elements#channel_select)
///
/// This select menu will populate its options with a list of
/// public channels visible to the current user in the active workspace.
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize, Validate)]
pub struct PublicChannel<'a> {
    #[validate(custom = "super::validate::placeholder")]
    placeholder: text::Text,

    #[validate(length(max = 255))]
    action_id: Cow<'a, str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<Confirm>,

    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    initial_channel: Option<Cow<'a, str>>,
}

impl<'a> PublicChannel<'a> {
    /// Construct a Select element, with a data
    /// source of the Public Channels in the user's
    /// Workspace.
    ///
    /// # Arguments
    /// - `placeholder` - A [`plain_text` only text object 🔗] that defines
    ///     the placeholder text shown on the menu.
    ///     Maximum length for the `text` in this field is 150 characters.
    ///
    /// - `action_id` - An identifier for the action triggered when a menu option is selected.
    ///     You can use this when you receive an interaction payload to [identify the source of the action 🔗].
    ///     Should be unique among all other `action_id`s used elsewhere by your app.
    ///     Maximum length for this field is 255 characters.
    /// 
    /// [`plain_text` only text object 🔗]: https://api.slack.comhttps://api.slack.com/reference/block-kit/composition-objects#text
    /// [identify the source of the action 🔗]: https://api.slack.comhttps://api.slack.com/interactivity/handling#payloads
    ///
    /// # Example
    /// ```
    /// use slack_blocks::block_elements::select;
    ///
    /// let select = select::PublicChannel::from_placeholder_and_action_id(
    ///     "Pick a Channel...",
    ///     "ABC123"
    /// );
    ///
    /// let blocks: Vec<blocks::Block> = vec![
    ///     blocks::Section::from_text(text::Plain::from(
    ///         "Pick a channel to send your poll to...")
    ///     ),
    ///     blocks::Actions::from_elements(vec![select]),
    /// ];
    ///
    /// // <send to slack's API>
    /// ```
    pub fn from_placeholder_and_action_id(
        placeholder: impl Into<text::Plain>,
        action_id: impl Into<Cow<'a, str>>
    ) -> Self {
        Self {
            placeholder: placeholder.into().into(),
            action_id: action_id.into(),
            confirm: None,
            user_id: None,
            initial_channel: None,
        }
    }

    /// Optional method that allows you to add a
    /// confirmation dialog that appears after a
    /// menu item is selected.
    ///
    /// # Arguments
    /// - `confirm` - A [confirm object 🔗] that defines an
    ///     optional confirmation dialog that appears after
    ///     a menu item is selected.
    /// 
    /// [confirm object 🔗]: https://api.slack.comhttps://api.slack.com/reference/block-kit/composition-objects#confirm
    ///
    /// # Example
    /// ```
    /// use slack_blocks::block_elements::select;
    /// use slack_blocks::blocks;
    /// use slack_blocks::compose;
    ///
    /// let confirm = compose::Confirm::from_parts(
    ///     "Confirm Poll Channel",
    ///     "Are you sure this is the channel you want to send this to?",
    ///     "Yep, I'm sure",
    ///     "No way!"
    /// );
    ///
    /// let select = select::PublicChannel::from_placeholder_and_action_id(
    ///         "Pick a Channel...",
    ///         "ABC123"
    ///     )
    ///     .with_confirm(confirm);
    ///
    /// let blocks: Vec<blocks::Block> = vec![
    ///     blocks::Section::from_text(text::Plain::from(
    ///         "Pick a channel to send your poll to...")
    ///     ),
    ///     blocks::Actions::from_elements(vec![select]),
    /// ];
    ///
    /// // <send to slack's API>
    /// ```
    pub fn with_confirm(mut self, confirm: Confirm) -> Self {
        self.confirm = Some(confirm);
        self
    }

    pub fn with_initial_user(
        mut self,
        user_id: impl Into<Cow<'a, str>>
    ) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    pub fn with_initial_channel(
        mut self,
        channel_id: impl Into<Cow<'a, str>>
    ) -> Self {
        self.initial_channel = Some(channel_id.into());
        self
    }

    pub fn validate(&self) -> ValidationResult {
        Validate::validate(&self)
    }
}
