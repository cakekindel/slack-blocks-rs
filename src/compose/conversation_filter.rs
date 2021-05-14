use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::val_helpr::ValidationResult;

/// # Filter for Conversations List
/// [slack api docs 🔗]
///
/// Provides a way to filter the list of options
/// in a [conversations select menu 🔗] or
/// [conversations multi-select menu 🔗].
///
/// [slack api docs 🔗]: https://api.slack.com/reference/block-kit/composition-objects#filter_conversations
/// [conversations select menu 🔗]: https://api.slack.comhttps://api.slack.com/reference/block-kit/block-elements#conversation_select
/// [conversations multi-select menu 🔗]: https://api.slack.comhttps://api.slack.com/reference/block-kit/block-elements#conversation_multi_select
#[derive(Clone,
           Debug,
           Default,
           Deserialize,
           Hash,
           PartialEq,
           Serialize,
           Validate)]
pub struct ConversationFilter {
  #[validate(length(min = 1, max = 4))]
  include: Option<Vec<ConversationKind>>,
  exclude_external_shared_channels: Option<bool>,
  exclude_bot_users: Option<bool>,
}

impl ConversationFilter {
  /// Create a Conversation Filter object
  /// that allows bot users & all kinds of channels;
  /// including cross-org shared channels.
  ///
  /// # Example
  /// ```
  /// use slack_blocks::compose::ConversationFilter;
  ///
  /// let filter = ConversationFilter::new();
  /// // TODO: once conversationselect is implemented
  /// // let select = ConversationSelect::from_filter(filter);
  /// ```
  pub fn new() -> Self {
    Default::default()
  }

  /// Chainable setter method that allows you to restrict
  /// the kinds of channels that will appear in the
  /// conversation select menu.
  ///
  /// For excluding cross-org shared channels, see
  /// `exclude_external_shared_channels`.
  ///
  /// For excluding DMs with bots, see `exclude_bot_users`.
  ///
  /// # Arguments
  /// - `kinds` - A **non-empty** unique collection of
  ///     `ConversationKind`s, that the select options
  ///     will be restricted to.
  ///
  /// # Example
  /// ```
  /// use slack_blocks::compose::ConversationFilter;
  /// use slack_blocks::compose::conversation_filter::ConversationKind;
  ///
  /// let filter = ConversationFilter::new()
  ///     .include_conversation_kinds(vec![
  ///         ConversationKind::PublicChannel,
  ///     ]);
  ///
  /// // TODO: once conversationselect is implemented
  /// // let select = ConversationSelect::from_filter(filter);
  /// ```
  pub fn include_conversation_kinds(mut self,
                                    kinds: impl IntoIterator<Item = ConversationKind>)
                                    -> Self {
    let mut kinds: Vec<_> = kinds.into_iter().collect();
    match kinds.len() {
      | 0 => self,
      | _ => {
        kinds.dedup();

        self.include = Some(kinds);
        self
      },
    }
  }

  /// Chainable setter method that allows cross-org
  /// shared channels to appear in the conversation
  /// select menu.
  ///
  /// Note that this setting is the default, and that
  /// calling this method is a no-op. It exists purely
  /// as declarative sugar for filter construction.
  ///
  /// For excluding cross-org shared channels, see
  /// `exclude_external_shared_channels`.
  ///
  /// # Example
  /// ```
  /// use slack_blocks::compose::ConversationFilter;
  ///
  /// let filter = ConversationFilter::new().include_external_shared_channels();
  ///
  /// // TODO: once conversationselect is implemented
  /// // let select = ConversationSelect::from_filter(filter);
  /// ```
  pub fn include_external_shared_channels(self) -> Self {
    self
  }

  /// Chainable setter method that prevents cross-workspace
  /// shared channels from appearing in the conversation
  /// select menu.
  ///
  /// # Example
  /// ```
  /// use slack_blocks::compose::ConversationFilter;
  ///
  /// let filter = ConversationFilter::new().exclude_external_shared_channels();
  ///
  /// // TODO: once conversationselect is implemented
  /// // let select = ConversationSelect::from_filter(filter);
  /// ```
  pub fn exclude_external_shared_channels(mut self) -> Self {
    self.exclude_external_shared_channels = Some(true);
    self
  }

  /// Chainable setter method that allows conversations
  /// with Bot Users to appear in the conversation
  /// select menu.
  ///
  /// This is the default behavior.
  ///
  /// For excluding bot user DMs, see `exclude_bot_users`.
  ///
  /// # Example
  /// ```
  /// use slack_blocks::compose::ConversationFilter;
  ///
  /// let filter = ConversationFilter::new().include_bot_users();
  ///
  /// // TODO: once conversationselect is implemented
  /// // let select = ConversationSelect::from_filter(filter);
  /// ```
  pub fn include_bot_users(self) -> Self {
    self
  }

  /// Chainable setter method that prevents DMs with
  /// Bot users from appearing in the conversation
  /// select menu.
  ///
  /// # Example
  /// ```
  /// use slack_blocks::compose::ConversationFilter;
  ///
  /// let filter = ConversationFilter::new().exclude_bot_users();
  ///
  /// // TODO: once conversationselect is implemented
  /// // let select = ConversationSelect::from_filter(filter);
  /// ```
  pub fn exclude_bot_users(mut self) -> Self {
    self.exclude_bot_users = Some(true);
    self
  }

  /// Validate that this Conversation Filter object
  /// agrees with Slack's model requirements.
  ///
  /// This type has runtime checks that prevent it from
  /// failing validation.
  ///
  /// # Errors
  /// - Never
  ///
  /// # Example
  /// ```
  /// use slack_blocks::compose::ConversationFilter;
  ///
  /// let filter = ConversationFilter::new().include_conversation_kinds(vec![]);
  ///
  /// assert_eq!(false, matches!(filter.validate(), Err(_)));
  /// ```
  pub fn validate(&self) -> ValidationResult {
    Validate::validate(self)
  }
}

// TODO: move this somewhere else. it is 100% gonna be used elsewhere.
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub enum ConversationKind {
  #[serde(rename = "im")]
  Dm,

  #[serde(rename = "mpim")]
  GroupDm,

  #[serde(rename = "public")]
  PublicChannel,

  #[serde(rename = "private")]
  PrivateChannel,
}
