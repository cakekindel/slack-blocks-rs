use std::{borrow::Cow, marker::PhantomData};

use compose::{opt::NoUrl, Confirm};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{block_elements::select::static_::build,
            compose,
            text,
            val_helpr::ValidationResult};

type OptGroup<'a> = compose::OptGroup<'a, text::Plain, NoUrl>;
type Opt<'a> = compose::Opt<'a, text::Plain, NoUrl>;
type OptOrOptGroup<'a> = compose::OptOrOptGroup<'a, text::Plain, NoUrl>;

/// # Select menu with static options
///
/// [slack api docs 🔗](https://api.slack.com/reference/block-kit/block-elements#static_select)
///
/// This is the simplest form of select menu,
/// with a static list of options passed in when defining the element.
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize, Validate)]
pub struct Static<'a> {
  #[validate(custom = "crate::block_elements::select::validate::placeholder")]
  pub(in crate::block_elements::select) placeholder: text::Text,

  #[validate(length(max = 255))]
  pub(in crate::block_elements::select) action_id: Cow<'a, str>,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[validate(length(max = 100))]
  pub(in crate::block_elements::select) options: Option<Vec<Opt<'a>>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[validate(length(max = 100))]
  pub(in crate::block_elements::select) option_groups: Option<Vec<OptGroup<'a>>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  #[validate]
  pub(in crate::block_elements::select) confirm: Option<Confirm>,

  pub(in crate::block_elements::select) initial_options:
    Option<Cow<'a, [OptOrOptGroup<'a>]>>,

  #[validate(range(min = 1))]
  pub(in crate::block_elements::select) max_selected_items: Option<u32>,
}

impl<'a> Static<'a> {
  /// Build a new static select element
  ///
  /// # Examples
  /// ```
  /// use std::convert::TryFrom;
  ///
  /// use slack_blocks::{block_elements::{select::Static, BlockElement},
  ///                    blocks::{Actions, Block},
  ///                    compose::Opt,
  ///                    text};
  ///
  /// struct City {
  ///   name: String,
  ///   short_code: String,
  /// }
  ///
  /// impl City {
  ///   pub fn new(name: impl ToString, short_code: impl ToString) -> Self {
  ///     Self { name: name.to_string(),
  ///            short_code: short_code.to_string() }
  ///   }
  /// }
  ///
  /// let cities = vec![City::new("Seattle", "SEA"),
  ///                   City::new("Portland", "PDX"),
  ///                   City::new("Phoenix", "PHX")];
  ///
  /// let options =
  ///   cities.iter().map(|City { name, short_code }| {
  ///                  Opt::builder().text_plain(name).value(short_code).build()
  ///                });
  ///
  /// let select: BlockElement =
  ///   Static::builder().select_multi()
  ///                    .placeholder("Choose your favorite cities!")
  ///                    .action_id("fave_city")
  ///                    .options(options)
  ///                    .build()
  ///                    .into();
  ///
  /// let block: Block = Actions::try_from(select).unwrap().into();
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
  /// use slack_blocks::block_elements::select;
  ///
  /// let placeholder = r#"Hey I really would appreciate it if you chose
  /// a channel relatively soon, so that we can figure out
  /// where we need to send this poll, ok? it's kind of
  /// important that you specify where this poll should be
  /// sent, in case we haven't made that super clear.
  /// If you understand, could you pick a channel, already??"#;
  ///
  /// let select = select::Static::builder().placeholder(placeholder)
  ///                                       .action_id("abc123")
  ///                                       .options(std::iter::empty())
  ///                                       .build();
  ///
  /// assert!(matches!(select.validate(), Err(_)))
  /// ```
  pub fn validate(&self) -> ValidationResult {
    Validate::validate(self)
  }
}
