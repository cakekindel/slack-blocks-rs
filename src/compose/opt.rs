use serde::{Deserialize, Serialize};
use validator::Validate;

use super::text;
use crate::val_helpr::ValidationResult;

// Used to _statically_
// identify `Opt`s as:
// - being created from mrkdwn
// - being created from plaintext
// - whether or not it has `url` set
pub mod marker {
    use crate::text;

    pub trait FromText<Text: Into<text::Text>> {}
    pub trait WithUrl {}
}

/// # Option Object
/// [slack api docs 🔗]
///
/// An object that represents a single selectable item in a
/// - [select menu 🔗],
/// - [multi-select menu 🔗],
/// - [checkbox group 🔗],
/// - [radio button group 🔗],
/// - or [overflow menu 🔗].
///
/// [slack api docs 🔗]: https://api.slack.com/reference/block-kit/composition-objects#option
/// [select menu 🔗]: https://api.slack.com/reference/block-kit/block-elements#select
/// [multi-select menu 🔗]: https://api.slack.com/reference/block-kit/block-elements#multi_select
/// [checkbox group 🔗]: https://api.slack.com/reference/block-kit/block-elements#checkboxes
/// [radio button group 🔗]: https://api.slack.com/reference/block-kit/block-elements#radio
/// [overflow menu 🔗]: https://api.slack.com/reference/block-kit/block-elements#overflow
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize, Validate)]
pub struct Opt<Marker = ()> {
    // *       ~~~~~~
    // This allows an `Opt` to _statically_
    // identify itself as:
    // - being created from mrkdwn
    // - being created from plaintext
    // - whether or not it has `url` set
    #[validate(custom = "validate::text")]
    text: text::Text,

    #[validate(length(max = 75))]
    value: String,

    #[validate(custom = "validate::desc")]
    description: Option<text::Text>,

    #[validate(length(max = 3000))]
    url: Option<String>,

    // This does not actually use any
    // data in memory, just asserts
    // "hey, i promise this Marker thing
    //  is used in some fashion"
    #[serde(skip)]
    __phantom: std::marker::PhantomData<Marker>,
}

// Constructor functions
impl Opt {
    /// Create an Option composition object from its label and
    /// a value to be sent back to your app when it is chosen.
    ///
    /// This returns an `Opt` that can be used by
    ///     overflow, select, and multi-select menus.
    ///     To construct an `Opt` that can be used by
    ///     radio buttons or checkboxes, see `from_mrkdwn_and_value`.
    ///
    /// # Arguments
    ///
    /// - `text` - A [text object 🔗] that defines the text shown in the option on the menu.
    ///     Overflow, select, and multi-select menus
    ///     can only use `plain_text` objects,
    ///     while radio buttons and checkboxes
    ///     can use `mrkdwn` text objects.
    ///     Maximum length for the `text` in this field is 75 characters.
    ///
    /// - `value` - The string value that will be passed to your app
    ///     when this option is chosen.
    ///     Maximum length for this field is 75 characters.
    ///
    /// [text object 🔗]: https://api.slack.com#text
    ///
    /// # Examples
    /// ```
    /// use slack_blocks::text;
    /// use slack_blocks::blocks::Block;
    /// use slack_blocks::blocks::section::Contents as Section;
    /// use slack_blocks::blocks::actions::Contents as Actions;
    /// use slack_blocks::compose::Opt;
    ///
    /// let cities = vec![
    ///   ("San Francisco", "san_francisco"),
    ///   ("San Diego", "san_diego"),
    ///   ("New York City", "nyc"),
    ///   ("Phoenix", "phx"),
    ///   ("Boston", "boston"),
    ///   ("Seattle", "seattle"),
    /// ]
    ///     .into_iter()
    ///     .map(|(title, short_code)| Opt::from_plain_text_and_value(title, short_code))
    ///     .collect::<Vec<_>>();
    ///
    /// let blocks: Vec<Block> = vec![
    ///   Section::from_text(text::Plain::from("Choose your favorite city...")).into(),
    ///   Actions::from_action_elements(vec![]).into() // TODO: add overflow to this example once it exists
    /// ];
    ///
    /// // < send block to slack's API >
    /// ```
    pub fn from_plain_text_and_value(
        text: impl Into<text::Plain>,
        value: impl ToString,
    ) -> Opt<PlainTextOpt> {
        Opt::<PlainTextOpt> {
            text: text.into().into(),
            value: value.to_string(),
            description: None,
            url: None,
            __phantom: std::marker::PhantomData,
        }
    }

    /// Create an Option composition object from its label and
    /// a value to be sent back to your app when it is chosen.
    ///
    /// This returns an `Opt` that can be used by
    ///     radio buttons or checkboxes.
    ///     To construct an `Opt` that can be used by
    ///     overflow, select, and multi-select menus,
    ///     see `from_plain_text_and_value`.
    ///
    /// # Arguments
    ///
    /// - `text` - A [text object 🔗] that defines the text shown in the option on the menu.
    ///     Overflow, select, and multi-select menus
    ///     can only use `plain_text` objects,
    ///     while radio buttons and checkboxes
    ///     can use `mrkdwn` text objects.
    ///     Maximum length for the `text` in this field is 75 characters.
    ///
    /// - `value` - The string value that will be passed to your app
    ///     when this option is chosen.
    ///     Maximum length for this field is 75 characters.
    ///
    /// [text object 🔗]: https://api.slack.com#text
    ///
    /// # Examples
    /// ```
    /// use slack_blocks::text;
    /// use slack_blocks::blocks::Block;
    /// use slack_blocks::blocks::section::Contents as Section;
    /// use slack_blocks::blocks::actions::Contents as Actions;
    /// use slack_blocks::compose::Opt;
    ///
    /// let options = vec![
    ///     "1",
    ///     "2",
    ///     "3",
    ///     "4",
    ///     "5",
    /// ]
    ///     .into_iter()
    ///     .map(|num| Opt::from_mrkdwn_and_value(num, num))
    ///     .collect::<Vec<_>>();
    ///
    /// let blocks: Vec<Block> = vec![
    ///   Section::from_text(text::Plain::from("On a scale from 1 to 5...")).into(),
    ///   Actions::from_action_elements(vec![]).into() // TODO: add radio buttons to this example once it exists
    /// ];
    ///
    /// // < send block to slack's API >
    /// ```
    pub fn from_mrkdwn_and_value(
        text: impl Into<text::Mrkdwn>,
        value: impl ToString,
    ) -> Opt<MrkdwnOpt> {
        Opt::<MrkdwnOpt> {
            text: text.into().into(),
            value: value.to_string(),
            description: None,
            url: None,
            __phantom: std::marker::PhantomData,
        }
    }
}

// Methods available to all specializations
impl<M> Opt<M> {
    /// Chainable setter method, that sets a description for this `Opt`.
    ///
    /// # Arguments
    ///
    /// - `desc` - A [`plain_text` only text object 🔗] that defines
    ///     a line of descriptive text shown below the `text` field
    ///     beside the radio button.
    ///     Maximum length for the `text` object within this field
    ///     is 75 characters.
    ///
    /// [`plain_text` only text object 🔗]: https://api.slack.comhttps://api.slack.com/reference/block-kit/composition-objects#text
    ///
    /// # Example
    ///
    /// ```
    /// use slack_blocks::text;
    /// use slack_blocks::blocks::Block;
    /// use slack_blocks::blocks::section::Contents as Section;
    /// use slack_blocks::blocks::actions::Contents as Actions;
    /// use slack_blocks::compose::Opt;
    ///
    /// let options = vec![
    ///     ("1", "Hated it."),
    ///     ("2", "Didn't like it."),
    ///     ("3", "It was OK."),
    ///     ("4", "Liked it!"),
    ///     ("5", "New favorite!!"),
    /// ]
    ///     .into_iter()
    ///     .map(|(num, desc)| {
    ///         Opt::from_mrkdwn_and_value(num, num)
    ///             .with_description(desc)
    ///     })
    ///     .collect::<Vec<_>>();
    ///
    /// let blocks: Vec<Block> = vec![
    ///   Section::from_text(text::Plain::from("On a scale from 1 to 5...")).into(),
    ///   Actions::from_action_elements(vec![]).into() // TODO: add radio buttons to this example once it exists
    /// ];
    ///
    /// // < send block to slack's API >
    /// ```
    pub fn with_description(mut self, desc: impl Into<text::Mrkdwn>) -> Self {
        self.description = Some(desc.into().into());
        self
    }

    /// Validate that this Option composition object
    /// agrees with Slack's model requirements
    ///
    /// # Errors
    /// - If `from_plain_text_and_value` or `from_mrkdwn_and_value`
    ///     was called with `text` longer than 75 chars
    /// - If `from_plain_text_and_value` or `from_mrkdwn_and_value`
    ///     was called with `value` longer than 75 chars
    /// - If `with_url` was called with url longer than 3000 chars
    /// - If `with_description` was called with text longer than 75 chars
    ///
    /// # Example
    /// ```
    /// use slack_blocks::compose::Opt;
    /// use std::iter::repeat;
    ///
    /// let long_string: String = repeat(' ').take(76).collect();
    ///
    /// let opt = Opt::from_plain_text_and_value("My Option", long_string);
    ///
    /// assert_eq!(true, matches!(opt.validate(), Err(_)));
    /// ```
    pub fn validate(&self) -> ValidationResult {
        Validate::validate(self)
    }
}

// Methods available only to `Opt` created from `text::Plain`
impl<M> Opt<M>
where
    M: marker::FromText<text::Plain>,
{
    /// Chainable setter method, that sets a description for this `Opt`.
    ///
    /// **The `url` attribute is only available in [overflow menus 🔗]**.
    ///
    /// If you're using `url`, you'll still receive an [interaction payload 🔗]
    /// and will need to [send an acknowledgement response 🔗].
    ///
    /// # Arguments
    /// - `url` - A URL to load in the user's browser when the option is clicked.
    ///     Maximum length for this field is 3000 characters.
    ///
    /// [overflow menus 🔗]: https://api.slack.com/reference/block-kit/block-elements#overflow
    /// [interaction payload 🔗]: https://api.slack.com/interactivity/handling#payloads
    /// [send an acknowledgement response 🔗]: https://api.slack.com/interactivity/handling#acknowledgment_response
    ///
    /// # Example
    /// ```
    /// use slack_blocks::text;
    /// use slack_blocks::blocks::Block;
    /// use slack_blocks::blocks::section::Contents as Section;
    /// use slack_blocks::blocks::actions::Contents as Actions;
    /// use slack_blocks::compose::Opt;
    ///
    /// let cities = vec![
    ///   ("San Francisco", "san_francisco", "https://www.sftravel.com/"),
    ///   ("San Diego", "san_diego", "https://www.sandiego.org/explore.aspx"),
    ///   ("New York City", "nyc", "https://www.nycgo.com/"),
    ///   ("Phoenix", "phx", "https://www.visitphoenix.com/"),
    ///   ("Boston", "boston", "https://www.boston.gov/visiting-boston"),
    ///   ("Seattle", "seattle", "https://visitseattle.org/"),
    /// ]
    ///     .into_iter()
    ///     .map(|(title, short_code, travel_link)| {
    ///         Opt::from_plain_text_and_value(title, short_code)
    ///             .with_url(travel_link)
    ///     })
    ///     .collect::<Vec<_>>();
    ///
    /// let blocks: Vec<Block> = vec![
    ///   Section::from_text(text::Plain::from("Choose your favorite city...")).into(),
    ///   Actions::from_action_elements(vec![]).into() // TODO: add overflow to this example once it exists
    /// ];
    ///
    /// // < send block to slack's API >
    /// ```
    pub fn with_url(self, url: impl ToString) -> Opt<PlainTextOptWithUrl> {
        Opt::<PlainTextOptWithUrl> {
            text: self.text,
            value: self.value,
            description: self.description,
            url: Some(url.to_string()),
            __phantom: std::marker::PhantomData,
        }
    }
}

/// A unit struct that indicates an `Opt` containing Mrkdwn text.
///
/// This is used in trait bounds of Block Elements to restrict
/// which kinds of Options they support.
#[derive(Clone, Debug, Hash, PartialEq)]
pub struct MrkdwnOpt;
impl marker::FromText<text::Mrkdwn> for MrkdwnOpt {}

/// A unit struct that indicates an `Opt` containing Plain text.
///
/// This is used in trait bounds of Block Elements to restrict
/// which kinds of Options they support.
#[derive(Clone, Debug, Hash, PartialEq)]
pub struct PlainTextOpt;
impl marker::FromText<text::Plain> for PlainTextOpt {}

/// A unit struct that indicates an `Opt` containing Plain text,
/// with `url` set.
///
/// This is used in trait bounds of Block Elements to restrict
/// which kinds of Options they support.
#[derive(Clone, Debug, Hash, PartialEq)]
pub struct PlainTextOptWithUrl;
impl marker::WithUrl for PlainTextOptWithUrl {}
impl marker::FromText<text::Plain> for PlainTextOptWithUrl {}

pub mod validate {
    use super::*;
    use crate::val_helpr::{below_len, ValidatorResult};

    pub fn text(text: &text::Text) -> ValidatorResult {
        below_len("Option Text", 75, text.as_ref())
    }

    pub fn desc(text: &text::Text) -> ValidatorResult {
        below_len("Option Description", 75, text.as_ref())
    }
}
