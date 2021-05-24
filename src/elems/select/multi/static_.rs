use std::borrow::Cow;

use compose::{opt::NoUrl, Confirm};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{compose,
            elems::select::static_::build,
            text,
            val_helpr::ValidationResult};

type OptGroup<'a> = compose::OptGroup<'a, text::Plain, NoUrl>;
type Opt<'a> = compose::Opt<'a, text::Plain, NoUrl>;
type OptOrOptGroup<'a> = compose::OptOrOptGroup<'a, text::Plain, NoUrl>;

/// # Multi-select menu with static options
///
/// [slack api docs 🔗](https://api.slack.com/reference/block-kit/block-elements#static_multi_select)
///
/// This is the simplest form of select menu,
/// with a static list of options passed in when defining the element.
///
/// Works in [blocks 🔗]: Section, Input
///
/// [slack api docs 🔗]: https://api.slack.com/reference/block-kit/block-elements#radio
/// [blocks 🔗]: https://api.slack.com/reference/block-kit/blocks
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize, Validate)]
pub struct Static<'a> {
  #[validate(custom = "crate::elems::select::validate::placeholder")]
  pub(in crate::elems::select) placeholder: text::Text,

  #[validate(length(max = 255))]
  pub(in crate::elems::select) action_id: Cow<'a, str>,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[validate(length(max = 100))]
  pub(in crate::elems::select) options: Option<Vec<Opt<'a>>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[validate(length(max = 100))]
  pub(in crate::elems::select) option_groups: Option<Vec<OptGroup<'a>>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[validate]
  pub(in crate::elems::select) confirm: Option<Confirm>,

  pub(in crate::elems::select) initial_options:
    Option<Cow<'a, [OptOrOptGroup<'a>]>>,

  #[validate(range(min = 1))]
  pub(in crate::elems::select) max_selected_items: Option<u32>,
}

impl<'a> Static<'a> {
  /// Build a new static select element
  ///
  /// # Examples
  /// ```
  /// // TODO(#130): implement once input or section can accept this
  /// ```
  pub fn builder() -> build::MultiStaticBuilderInit<'a> {
    build::MultiStaticBuilderInit::new()
  }

  /// Validate that this select element agrees with Slack's model requirements
  ///
  /// # Errors
  /// - If `from_placeholder_and_action_id` was called with
  ///     `placeholder` longer than 150 chars
  /// - If `from_placeholder_and_action_id` was called with
  ///     `action_id` longer than 255 chars
  ///
  /// # Example
  /// ```
  /// use slack_blocks::elems::select;
  ///
  /// let placeholder = r#"Hey I really would appreciate it if you chose
  /// a channel relatively soon, so that we can figure out
  /// where we need to send this poll, ok? it's kind of
  /// important that you specify where this poll should be
  /// sent, in case we haven't made that super clear.
  /// If you understand, could you pick a channel, already??"#;
  ///
  /// let select = select::multi::Static::builder().placeholder(placeholder)
  ///                                              .action_id("abc123")
  ///                                              .options(std::iter::empty())
  ///                                              .build();
  ///
  /// assert!(matches!(select.validate(), Err(_)))
  /// ```
  pub fn validate(&self) -> ValidationResult {
    Validate::validate(self)
  }
}
