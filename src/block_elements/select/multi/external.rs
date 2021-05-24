//! # Multi-Select menu with external data source
//! [slack api docs 🔗]
//!
//! This select menu will load its options from an external data source,
//! allowing for a dynamic list of options.
//!
//! ## Setup
//! [Slack API doc guide for setting up an external data source 🔗](https://api.slack.com/reference/block-kit/block-elements#external_select__setup)
//!
//! [slack api docs 🔗]: https://api.slack.com/reference/block-kit/block-elements#external_select

use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{compose,
            compose::{opt::NoUrl, Confirm},
            elems::select::external::build,
            text,
            val_helpr::ValidationResult};

type OptOrOptGroup<'a> = compose::OptOrOptGroup<'a, text::Plain, NoUrl>;

/// # Multi-Select menu with external data source
/// [slack api docs 🔗]
///
/// This select menu will load its options from an external data source,
/// allowing for a dynamic list of options.
///
/// ## Setup
/// [Slack API doc guide for setting up an external data source 🔗](https://api.slack.com/reference/block-kit/block-elements#external_select__setup)
///
/// [slack api docs 🔗]: https://api.slack.com/reference/block-kit/block-elements#external_select
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize, Validate)]
pub struct External<'a> {
  #[validate(custom = "crate::elems::select::validate::placeholder")]
  pub(in crate::elems::select) placeholder: text::Text,

  #[validate(length(max = 255))]
  pub(in crate::elems::select) action_id: Cow<'a, str>,

  pub(in crate::elems::select) min_query_length: Option<u64>,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[validate]
  pub(in crate::elems::select) confirm: Option<Confirm>,

  pub(in crate::elems::select) initial_options:
    Option<Cow<'a, [OptOrOptGroup<'a>]>>,

  #[validate(range(min = 1))]
  pub(in crate::elems::select) max_selected_items: Option<u32>,
}

impl<'a> External<'a> {
  /// Build a new external multi-select element
  ///
  /// # Examples
  /// ```
  /// // TODO(#130)
  /// ```
  pub fn builder() -> build::MultiExternalBuilderInit<'a> {
    build::MultiExternalBuilderInit::new()
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
  /// let placeholder = r#"Hey I really would appreciate it if you chose
  ///         a channel relatively soon, so that we can figure out
  ///         where we need to send this poll, ok? it's kind of
  ///         important that you specify where this poll should be
  ///         sent, in case we haven't made that super clear.
  ///         If you understand, could you pick a channel, already??"#;
  ///
  /// let select = select::multi::External::builder().placeholder(placeholder)
  ///                                                .action_id("ABC123")
  ///                                                .build();
  ///
  /// assert!(matches!(select.validate(), Err(_)))
  /// ```
  pub fn validate(&self) -> ValidationResult {
    Validate::validate(self)
  }
}
