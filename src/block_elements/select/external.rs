use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use validator::Validate;

use crate::text;
use crate::compose::{OptOrOptGroup, Confirm, opt::marker::FromText};
use crate::val_helpr::ValidationResult;

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
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize, Validate)]
pub struct External<'a> {
    #[validate(custom = "super::validate::placeholder")]
    placeholder: text::Text,

    #[validate(length(max = 255))]
    action_id: Cow<'a, str>,

    initial_option: Option<OptOrOptGroup<FromText<text::Plain>>>,

    min_query_length: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate]
    confirm: Option<Confirm>,
}

impl<'a> External<'a> {
    /// Construct an External Select element, letting users choose an option from an external data source.
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
    /// use std::convert::TryFrom;
    /// use std::iter;
    ///
    /// use slack_blocks::block_elements::{BlockElement, select};
    /// use slack_blocks::blocks;
    /// use slack_blocks::text;
    /// use text::ToSlackPlaintext;
    ///
    /// let select: BlockElement = select::External
    ///                                  ::from_placeholder_and_action_id("Channel", "ABC123")
    ///                                   .into();
    ///
    /// let title = "Pick a user to ban...".plaintext();
    ///
    /// let blocks: Vec<blocks::Block> = vec![
    ///     blocks::Section::from_text(title).into(),
    ///     blocks::Actions::try_from(vec![select]).unwrap().into(),
    /// ];
    ///
    /// // <send `blocks` to slack's API>
    /// ```
    pub fn from_placeholder_and_action_id(
        placeholder: impl Into<text::Plain>,
        action_id: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            placeholder: placeholder.into().into(),
            action_id: action_id.into(),
            confirm: None,
            initial_option: None,
            min_query_length: None,
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
    /// use std::iter;
    /// use std::convert::TryFrom;
    ///
    /// use slack_blocks::{
    ///   blocks::{Block, Actions},
    ///   block_elements::{BlockElement, select::Select},
    ///   compose::{text, Confirm, text::ToSlackPlaintext},
    /// };
    ///
    /// # use std::error::Error;
    /// # pub fn main() -> Result<(), Box<dyn Error>> {
    ///
    /// let confirm = Confirm::from_parts(
    ///   "Are you sure?",
    ///   "Think hard about this.".plaintext(),
    ///   "Yes",
    ///   "No",
    /// );
    ///
    /// let select: BlockElement = Select::from_placeholder_and_action_id("Pick a user to ban!", "ban_hammer")
    ///                                   .with_confirm(confirm)
    ///                                   .choose_from_users()
    ///                                   .into();
    ///
    /// let block: Block = Actions::try_from(select).unwrap().into();
    ///
    /// // < send `block` to slack API >
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_confirm(mut self, confirm: Confirm) -> Self {
        self.confirm = Some(confirm);
        self
    }

    /// Optional method that allows you to add a
    /// confirmation dialog that appears after a
    /// menu item is selected.
    ///
    /// # Arguments
    /// - `min_query_length` - When the typeahead field is used, a request will be sent on every character change.
    ///     If you prefer fewer requests or more fully ideated queries, use the `min_query_length` attribute to tell Slack the fewest number of typed characters required before dispatch.
    ///     The default value is `3`.
    ///
    /// # Example
    /// ```
    /// use std::iter;
    /// use std::convert::TryFrom;
    ///
    /// use slack_blocks::{
    ///   blocks::{Block, Actions},
    ///   block_elements::{BlockElement, select::Select},
    ///   compose::{text, text::ToSlackPlaintext},
    /// };
    ///
    /// # use std::error::Error;
    /// # pub fn main() -> Result<(), Box<dyn Error>> {
    ///
    /// let select: BlockElement = Select::from_placeholder_and_action_id("Pick your favorite cheese!", "cheese_chosen")
    ///                                   .choose_from_external()
    ///                                   // we want a responsive typeahead because some cheese names are short!
    ///                                   .with_min_query_length(1)
    ///                                   .into();
    ///
    /// let block: Block = Actions::try_from(select).unwrap().into();
    ///
    /// // < send `block` to slack API >
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_min_query_length(mut self, min_query_length: u64) -> Self {
        self.min_query_length = Some(min_query_length);
        self
    }

    /// Optional method that allows you to add a
    /// confirmation dialog that appears after a
    /// menu item is selected.
    ///
    /// # Arguments
    /// - `min_query_length` - When the typeahead field is used, a request will be sent on every character change.
    ///     If you prefer fewer requests or more fully ideated queries, use the `min_query_length` attribute to tell Slack the fewest number of typed characters required before dispatch.
    ///     The default value is `3`.
    ///
    /// # Example
    /// ```
    /// use std::iter;
    /// use std::convert::TryFrom;
    ///
    /// use slack_blocks::{
    ///   blocks::{Block, Actions},
    ///   block_elements::{BlockElement, select::Select},
    ///   compose::{opt::Opt, text, text::ToSlackPlaintext},
    /// };
    ///
    /// # use std::error::Error;
    /// # pub fn main() -> Result<(), Box<dyn Error>> {
    ///
    /// let brie = Opt::from_plain_text_and_value("Brie", "cheese_brie");
    ///
    /// let select: BlockElement = Select::from_placeholder_and_action_id(
    ///                                       "Pick your favorite cheese!",
    ///                                       "cheese_chosen",
    ///                                   )
    ///                                   .choose_from_external()
    ///                                   .with_initial_option(brie)
    ///                                   .into();
    ///
    /// let block: Block = Actions::try_from(select).unwrap().into();
    ///
    /// // < send `block` to slack API >
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_initial_option(mut self, option: impl Into<OptOrOptGroup<FromText<text::Plain>>>) -> Self {
        self.initial_option = Some(option.into());
        self
    }

    /// Validate that this user select agrees with Slack's model requirements
    ///
    /// # Errors
    /// - If `from_placeholder_and_action_id` was called with
    ///     `placeholder` longer than 150 chars
    /// - If `from_placeholder_and_action_id` was called with
    ///     `action_id` longer than 255 chars
    ///
    /// # Example
    /// ```
    /// use slack_blocks::block_elements::select;
    ///
    /// let select = select::External::from_placeholder_and_action_id(
    ///         r#"Hey I really would appreciate it if you chose
    ///         a channel relatively soon, so that we can figure out
    ///         where we need to send this poll, ok? it's kind of
    ///         important that you specify where this poll should be
    ///         sent, in case we haven't made that super clear.
    ///         If you understand, could you pick a channel, already??"#,
    ///         "ABC123"
    ///     );
    ///
    /// assert!(matches!(select.validate(), Err(_)))
    /// ```
    pub fn validate(&self) -> ValidationResult {
      Validate::validate(self)
    }
}
