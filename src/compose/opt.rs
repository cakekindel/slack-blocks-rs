//! # Option Object
//! [slack api docs 🔗]
//!
//! An object that represents a single selectable item in a
//! - [select menu 🔗],
//! - [multi-select menu 🔗],
//! - [checkbox group 🔗],
//! - [radio button group 🔗],
//! - or [overflow menu 🔗].
//!
//! [slack api docs 🔗]: https://api.slack.com/reference/block-kit/composition-objects#option
//! [select menu 🔗]: https://api.slack.com/reference/block-kit/block-elements#select
//! [multi-select menu 🔗]: https://api.slack.com/reference/block-kit/block-elements#multi_select
//! [checkbox group 🔗]: https://api.slack.com/reference/block-kit/block-elements#checkboxes
//! [radio button group 🔗]: https://api.slack.com/reference/block-kit/block-elements#radio
//! [overflow menu 🔗]: https://api.slack.com/reference/block-kit/block-elements#overflow

use std::{borrow::Cow, marker::PhantomData};

use serde::{Deserialize, Serialize};
use validator::Validate;

use super::text;
use crate::{build::*, convert, val_helpr::ValidationResult};

/// Opt supports text::Plain and text::Mrkdwn.
#[derive(Copy,
           Clone,
           Debug,
           Deserialize,
           Hash,
           PartialEq,
           Serialize,
           Validate)]
pub struct AnyText;

/// Opt does not support urls.
#[derive(Copy,
           Clone,
           Debug,
           Deserialize,
           Hash,
           PartialEq,
           Serialize,
           Validate)]
pub struct NoUrl;

/// Opt does support urls.
#[derive(Copy,
           Clone,
           Debug,
           Deserialize,
           Hash,
           PartialEq,
           Serialize,
           Validate)]
pub struct AllowUrl;

convert!(impl From<NoUrl> for AllowUrl => |_| AllowUrl);

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
pub struct Opt<'a, T = AnyText, U = NoUrl> {
  #[validate(custom = "validate::text")]
  text: text::Text,

  #[validate(length(max = 75))]
  value: Cow<'a, str>,

  #[validate(custom = "validate::desc")]
  #[serde(skip_serializing_if = "Option::is_none")]
  description: Option<text::Text>,

  #[validate(custom = "validate::url")]
  #[serde(skip_serializing_if = "Option::is_none")]
  url: Option<Cow<'a, str>>,

  #[serde(skip)]
  marker: PhantomData<(T, U)>,
}

impl<'a, T: Into<text::Text>, U> From<Opt<'a, T, U>> for Opt<'a, AnyText, U> {
  fn from(o: Opt<'a, T, U>) -> Self {
    Opt { text: o.text,
          value: o.value,
          description: o.description,
          url: o.url,
          marker: PhantomData::<(AnyText, U)> }
  }
}

// Constructor functions
impl<'a> Opt<'a> {
  /// Build a new option composition object
  ///
  /// # Examples
  /// ```
  /// use std::convert::TryFrom;
  ///
  /// use slack_blocks::{blocks::Actions,
  ///                    compose::Opt,
  ///                    elems::{select::Static, BlockElement},
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
  ///   Static::builder().placeholder("Choose your favorite city!")
  ///                    .action_id("fave_city")
  ///                    .options(options)
  ///                    .build()
  ///                    .into();
  ///
  /// let block = Actions::try_from(select);
  /// ```
  pub fn builder() -> build::OptBuilderInit<'a> {
    build::OptBuilderInit::new()
  }

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
  /// use slack_blocks::blocks::Section;
  /// use slack_blocks::blocks::Actions;
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
  #[deprecated(since = "0.15.0", note = "Use Opt::builder instead")]
  pub fn from_plain_text_and_value(text: impl Into<text::Plain>,
                                   value: impl Into<Cow<'a, str>>)
                                   -> Opt<'a, text::Plain, NoUrl> {
    Opt { text: text.into().into(),
          value: value.into(),
          description: None,
          url: None,
          marker: std::marker::PhantomData }
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
  /// use slack_blocks::blocks::Section;
  /// use slack_blocks::blocks::Actions;
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
  #[deprecated(since = "0.15.0", note = "Use Opt::builder instead")]
  pub fn from_mrkdwn_and_value(text: impl Into<text::Mrkdwn>,
                               value: impl Into<Cow<'a, str>>)
                               -> Opt<'a, text::Mrkdwn, NoUrl> {
    Opt { text: text.into().into(),
          value: value.into(),
          description: None,
          url: None,
          marker: std::marker::PhantomData }
  }
}

impl<'a, U> Opt<'a, text::Plain, U> {
  /// Ensure the type flags of the opt say "AllowUrl", used to mix NoUrl and AllowUrl in overflow menus.
  pub(crate) fn as_allow_url(self) -> Opt<'a, text::Plain, AllowUrl> {
    Opt { text: self.text,
          value: self.value,
          description: self.description,
          url: self.url,
          marker: PhantomData::<(text::Plain, AllowUrl)> }
  }
}

// Methods available to all specializations
impl<'a, T, U> Opt<'a, T, U> {
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
  /// [`plain_text` only text object 🔗]: https://api.slack.com/reference/block-kit/composition-objects#text
  ///
  /// # Example
  ///
  /// ```
  /// use slack_blocks::text;
  /// use slack_blocks::blocks::Block;
  /// use slack_blocks::blocks::Section;
  /// use slack_blocks::blocks::Actions;
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
  #[deprecated(since = "0.15.0", note = "Use Opt::builder instead")]
  pub fn with_description(mut self, desc: impl Into<text::Plain>) -> Self {
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
  /// use std::iter::repeat;
  ///
  /// use slack_blocks::compose::Opt;
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
impl<'a, U> Opt<'a, text::Plain, U> {
  /// Chainable setter method, that sets a url for this `Opt`.
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
  /// use slack_blocks::blocks::Section;
  /// use slack_blocks::blocks::Actions;
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
  #[deprecated(since = "0.15.0", note = "Use Opt::builder instead")]
  pub fn with_url(self,
                  url: impl Into<Cow<'a, str>>)
                  -> Opt<'a, text::Plain, AllowUrl> {
    Opt { text: self.text,
          value: self.value,
          description: self.description,
          url: Some(url.into()),
          marker: std::marker::PhantomData }
  }
}

/// Opt builder
pub mod build {
  use std::marker::PhantomData;

  use super::*;

  /// Required builder methods
  #[allow(non_camel_case_types)]
  pub mod method {
    /// OptBuilder.value
    #[derive(Copy, Clone, Debug)]
    pub struct value;
    /// OptBuilder.text
    #[derive(Copy, Clone, Debug)]
    pub struct text;
    /// OptBuilder.url
    #[derive(Copy, Clone, Debug)]
    pub struct url;
  }

  /// Initial state for OptBuilder
  pub type OptBuilderInit<'a> =
    OptBuilder<'a,
               RequiredMethodNotCalled<method::text>,
               RequiredMethodNotCalled<method::value>,
               OptionalMethodNotCalled<method::url>>;

  /// Option builder
  ///
  /// Allows you to construct a Option composition object safely, with compile-time checks
  /// on required setter methods.
  ///
  /// # Required Methods
  /// `Opt::build()` is only available if these methods have been called:
  ///  - `text` or `text_plain` or `text_md`
  ///  - `value`
  ///
  /// # Example
  /// ```
  /// use std::convert::TryFrom;
  ///
  /// use slack_blocks::{blocks::{Actions, Block},
  ///                    compose::Opt,
  ///                    elems::{select::Static, BlockElement}};
  /// let langs = vec![("Rust", "rs"), ("Haskell", "hs"), ("NodeJS", "node")];
  ///
  /// let langs =
  ///   langs.into_iter().map(|(name, code)| {
  ///                      Opt::builder().text_plain(name).value(code).build()
  ///                    });
  ///
  /// let select: BlockElement =
  ///   Static::builder().placeholder("Choose your favorite programming language!")
  ///                    .options(langs)
  ///                    .action_id("lang_chosen")
  ///                    .build()
  ///                    .into();
  ///
  /// let block: Block =
  ///   Actions::try_from(select).expect("actions supports select elements")
  ///                            .into();
  ///
  /// // <send block to API>
  /// ```
  #[derive(Debug)]
  pub struct OptBuilder<'a, Text, Value, Url> {
    text: Option<text::Text>,
    value: Option<Cow<'a, str>>,
    description: Option<text::Text>,
    url: Option<Cow<'a, str>>,
    state: PhantomData<(Text, Value, Url)>,
  }

  impl<T, V, U> OptBuilder<'static, T, V, U> {
    /// Construct a new OptBuilder
    pub fn new() -> OptBuilderInit<'static> {
      OptBuilderInit { text: None,
                       value: None,
                       description: None,
                       url: None,
                       state: PhantomData::<_> }
    }
  }

  impl<'a, T, V, U> OptBuilder<'a, T, V, U> {
    /// Change the marker type params to some other arbitrary marker type params
    fn cast_state<T2, V2, U2>(self) -> OptBuilder<'a, T2, V2, U2> {
      OptBuilder { text: self.text,
                   value: self.value,
                   description: self.description,
                   url: self.url,
                   state: PhantomData::<_> }
    }

    /// Set `value` (**Required**)
    ///
    /// The string value that will be passed to your app
    /// when this option is chosen.
    ///
    /// Maximum length for this field is 75 characters.
    pub fn value<S>(mut self,
                    value: S)
                    -> OptBuilder<'a, T, Set<method::value>, U>
      where S: Into<Cow<'a, str>>
    {
      self.value = Some(value.into());
      self.cast_state()
    }

    /// Set `description` (Optional)
    ///
    /// A [`plain_text` only text object 🔗] that defines
    /// a line of descriptive text shown below the `text` field
    /// beside the radio button.
    ///
    /// Maximum length for the `text` object within this field
    /// is 75 characters.
    ///
    /// [`plain_text` only text object 🔗]: https://api.slack.com/reference/block-kit/composition-objects#text
    pub fn desc<S>(mut self, desc: S) -> OptBuilder<'a, T, V, U>
      where S: Into<text::Plain>
    {
      self.description = Some(desc.into().into());
      self.cast_state()
    }
  }

  impl<'a, V, U> OptBuilder<'a, RequiredMethodNotCalled<method::text>, V, U> {
    /// Alias for `text`, allowing you to set the text of the option like so:
    /// ```
    /// use mox::mox;
    /// use slack_blocks::{compose::Opt, mox::*, text};
    ///
    /// let xml = mox! {
    ///   <option value="foo">
    ///     <text kind=plain>"Foo"</text>
    ///   </option>
    /// };
    ///
    /// let equiv = Opt::builder().value("foo")
    ///                           .text(text::Plain::from("Foo"))
    ///                           .build();
    ///
    /// assert_eq!(xml, equiv)
    /// ```
    #[cfg(feature = "xml")]
    #[cfg_attr(docsrs, doc(cfg(feature = "xml")))]
    pub fn child<T: Into<text::Text>>(
      self,
      text: T)
      -> OptBuilder<'a, Set<(method::text, T)>, V, U> {
      self.text(text)
    }

    /// Set `text` (**Required**)
    ///
    /// A [text object 🔗] that defines the text shown in the option on the menu.
    /// Overflow, select, and multi-select menus
    /// can only use `plain_text` objects,
    /// while radio buttons and checkboxes
    /// can use `mrkdwn` text objects.
    ///
    /// Maximum length for the `text` in this field is 75 characters.
    ///
    /// [text object 🔗]: https://api.slack.com#text
    pub fn text<Txt>(mut self,
                     text: Txt)
                     -> OptBuilder<'a, Set<(method::text, Txt)>, V, U>
      where Txt: Into<text::Text>
    {
      self.text = Some(text.into());
      self.cast_state()
    }

    /// Set `text` (**Required**)
    ///
    /// A [text object 🔗] that defines the text shown in the option on the menu.
    /// Overflow, select, and multi-select menus
    /// can only use `plain_text` objects,
    /// while radio buttons and checkboxes
    /// can use `mrkdwn` text objects.
    ///
    /// Maximum length for the `text` in this field is 75 characters.
    ///
    /// [text object 🔗]: https://api.slack.com#text
    pub fn text_plain<Txt>(
      self,
      text: Txt)
      -> OptBuilder<'a, Set<(method::text, text::Plain)>, V, U>
      where Txt: Into<text::Plain>
    {
      self.text(text.into())
    }

    /// Set `text` (**Required**)
    ///
    /// A [text object 🔗] that defines the text shown in the option on the menu.
    /// Overflow, select, and multi-select menus
    /// can only use `plain_text` objects,
    /// while radio buttons and checkboxes
    /// can use `mrkdwn` text objects.
    ///
    /// Maximum length for the `text` in this field is 75 characters.
    ///
    /// [text object 🔗]: https://api.slack.com#text
    pub fn text_md<Txt>(
      self,
      text: Txt)
      -> OptBuilder<'a, Set<(method::text, text::Mrkdwn)>, V, U>
      where Txt: Into<text::Mrkdwn>
    {
      self.text(text.into())
    }
  }

  impl<'a, V, U> OptBuilder<'a, Set<(method::text, text::Plain)>, V, U> {
    /// Set `url` (Optional)
    ///
    /// The URL will be loaded in the user's browser when the option is clicked.
    ///
    /// Maximum length for this field is 3000 characters.
    ///
    /// The `url` attribute is only available in [overflow menus 🔗]
    ///
    /// If you're using `url`, you'll still receive an [interaction payload 🔗]
    /// and will need to [send an acknowledgement response 🔗].
    ///
    /// [overflow menus 🔗]: https://api.slack.com/reference/block-kit/block-elements#overflow
    /// [interaction payload 🔗]: https://api.slack.com/interactivity/handling#payloads
    /// [send an acknowledgement response 🔗]: https://api.slack.com/interactivity/handling#acknowledgment_response
    pub fn url<S>(
      mut self,
      url: S)
      -> OptBuilder<'a, Set<(method::text, text::Plain)>, V, Set<method::url>>
      where S: Into<Cow<'a, str>>
    {
      self.url = Some(url.into());
      self.cast_state()
    }

    /// Flag opt as being usable in an `AllowUrl` context without setting Url explicitly.
    pub fn no_url(
      self)
      -> OptBuilder<'a, Set<(method::text, text::Plain)>, V, Set<method::url>>
    {
      self.cast_state()
    }
  }

  impl<'a>
    OptBuilder<'a,
               Set<(method::text, text::Plain)>,
               Set<method::value>,
               Set<method::url>>
  {
    /// All done building, now give me a darn option!
    ///
    /// > `no method name 'build' found for struct 'compose::opt::build::OptBuilder<...>'`?
    ///
    /// Make sure all required setter methods have been called. See docs for `OptBuilder`.
    ///
    /// ```compile_fail
    /// use slack_blocks::compose::Opt;
    ///
    /// let sel = Opt::builder().build(); // Won't compile!
    /// ```
    ///
    /// ```
    /// use slack_blocks::compose::Opt;
    ///
    /// let opt = Opt::builder().text_plain("cheese")
    ///                         .value("cheese")
    ///                         .url("https://cheese.com")
    ///                         .build();
    /// ```
    pub fn build(self) -> Opt<'a, text::Plain, AllowUrl> {
      Opt { text: self.text.unwrap(),
            value: self.value.unwrap(),
            url: self.url,
            description: self.description,
            marker: PhantomData::<_> }
    }
  }

  impl<'a, T: Into<text::Text>>
    OptBuilder<'a,
               Set<(method::text, T)>,
               Set<method::value>,
               OptionalMethodNotCalled<method::url>>
  {
    /// All done building, now give me a darn option!
    ///
    /// > `no method name 'build' found for struct 'compose::opt::build::OptBuilder<...>'`?
    ///
    /// Make sure all required setter methods have been called. See docs for `OptBuilder`.
    ///
    /// ```compile_fail
    /// use slack_blocks::compose::Opt;
    ///
    /// let sel = Opt::builder().text_plain("foo")
    ///                         .build();
    /// /*                       ^^^^^ method not found in
    ///                          `OptBuilder<'_, Set<(text, text::Plain)>, RequiredMethodNotCalled<value>>`
    /// */
    /// ```
    ///
    /// ```
    /// use slack_blocks::compose::Opt;
    ///
    /// let opt = Opt::builder().text_md("cheese").value("cheese").build();
    /// ```
    pub fn build(self) -> Opt<'a, T, NoUrl> {
      Opt { text: self.text.unwrap(),
            value: self.value.unwrap(),
            url: self.url,
            description: self.description,
            marker: PhantomData::<_> }
    }
  }
}

mod validate {
  use super::*;
  use crate::val_helpr::{below_len, ValidatorResult};

  pub(super) fn text(text: &text::Text) -> ValidatorResult {
    below_len("Option Text", 75, text.as_ref())
  }

  pub(super) fn desc(text: &text::Text) -> ValidatorResult {
    below_len("Option Description", 75, text.as_ref())
  }

  pub(super) fn url(text: &Cow<'_, str>) -> ValidatorResult {
    below_len("URL", 3000, text.as_ref())
  }
}
