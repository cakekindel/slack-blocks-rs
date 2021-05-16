use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{text, Opt};
use crate::val_helpr::ValidationResult;

/// # Option Group
/// [slack api docs 🔗]
///
/// Provides a way to group options in a [select menu 🔗] or [multi-select menu 🔗].
///
/// [select menu 🔗]: https://api.slack.com/reference/block-kit/block-elements#select
/// [multi-select menu 🔗]: https://api.slack.com/reference/block-kit/block-elements#multi_select
/// [slack api docs 🔗]: https://api.slack.com/reference/block-kit/composition-objects#option_group
/// [`plain_text` only text object 🔗]: https://api.slack.com/reference/block-kit/composition-objects#text
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize, Validate)]
pub struct OptGroup<'a, M = ()> {
  #[validate(custom = "validate::label")]
  label: text::Text,

  #[validate(length(max = 100))]
  options: Vec<Opt<'a, M>>,
}

impl<'a> OptGroup<'a> {
  /// Construct an Option Group from a label and
  /// collection of options in the group
  ///
  /// # Arguments
  /// - `label` - A [`plain_text` only text object 🔗] that defines
  ///     the label shown above this group of options.
  ///     Maximum length for the `text` in this field is 75 characters.
  /// - `opts` - An array of [option objects 🔗] that belong to
  ///     this specific group. Maximum of 100 items.
  ///
  /// [option objects 🔗]: https://api.slack.comCURRENT_PAGEoption
  /// [`plain_text` only text object 🔗]: https://api.slack.comCURRENT_PAGEtext
  ///
  /// # Example
  /// ```
  /// use slack_blocks::blocks::Block;
  /// use slack_blocks::blocks::section::Contents as Section;
  /// use slack_blocks::blocks::actions::Contents as Actions;
  /// use slack_blocks::text::{Mrkdwn};
  /// use slack_blocks::compose::{OptGroup, Opt};
  ///
  /// let prompt = "Choose your favorite city from each state!";
  ///
  /// let blocks: Vec<Block> = vec![
  ///     Section::from_text(Mrkdwn::from(prompt)).into(),
  ///     // TODO: insert option group once block elements are in place
  ///     Actions::from_action_elements(vec![]).into(),
  /// ];
  ///
  /// let groups: Vec<OptGroup<_>> = vec![
  ///     OptGroup::from_label_and_opts(
  ///         "Arizona",
  ///         vec![
  ///             Opt::from_mrkdwn_and_value("Phoenix", "az_phx"),
  ///             // etc...
  ///         ]
  ///     ),
  ///     OptGroup::from_label_and_opts(
  ///         "California",
  ///         vec![
  ///             Opt::from_mrkdwn_and_value("San Diego", "ca_sd"),
  ///             // etc...
  ///         ]
  ///     ),
  /// ];
  /// ```
  pub fn from_label_and_opts<M>(label: impl Into<text::Plain>,
                                options: impl IntoIterator<Item = Opt<'a, M>>)
                                -> OptGroup<'a, M> {
    OptGroup::<'a, M> { label: label.into().into(),
                        options: options.into_iter().collect() }
  }
}

impl<'a, M> OptGroup<'a, M> {
  /// Validate that this Option Group object
  /// agrees with Slack's model requirements
  ///
  /// # Errors
  /// - If `from_label_and_opts` was called with `label`
  ///     longer than 75 chars
  /// - If `from_label_and_opts` was called with
  ///     more than 100 options
  ///
  /// # Example
  /// ```
  /// use std::iter::repeat;
  ///
  /// use slack_blocks::compose::{Opt, OptGroup};
  ///
  /// let long_string: String = repeat(' ').take(76).collect();
  ///
  /// let opt = Opt::from_mrkdwn_and_value("San Diego", "ca_sd");
  /// let grp = OptGroup::from_label_and_opts(long_string, vec![opt]);
  ///
  /// assert_eq!(true, matches!(grp.validate(), Err(_)));
  /// ```
  pub fn validate(&self) -> ValidationResult {
    Validate::validate(self)
  }
}

mod validate {
  use super::*;
  use crate::val_helpr::{below_len, ValidatorResult};

  pub fn label(text: &text::Text) -> ValidatorResult {
    below_len("Option Group Label", 75, text.as_ref())
  }
}
