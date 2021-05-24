//! ## Markdown text
//! [_for more info, check out the slack api docs 🔗_](https://api.slack.com/reference/surfaces/formatting)

use serde::{Deserialize, Serialize};

/// ## Markdown text
/// [_for more info, check out the slack api docs 🔗_](https://api.slack.com/reference/surfaces/formatting)
///
/// ### Reserved Characters
///
/// Slack uses the following special characters,
/// and recommends you HTML escape them like so:
///
/// <details>
/// <summary><b>➤ Click to expand</b></summary>
///
/// |character|how to escape|
/// |---      |---          |
/// |`&`      |`&amp;`      |
/// |`<`      |`&lt;`       |
/// |`>`      |`&gt;`       |
/// </details>
///
/// ### Basic Formatting
///
/// NOTE: This is **not** an exhaustive list
///
/// This should, however, capture most basic
/// use cases without requiring that you check with
/// the Slack documentation.
///
/// For more info, please visit
/// [Slack's docs for markdown formatting 🔗](https://api.slack.com/reference/surfaces/formatting)
///
/// <details>
/// <summary><b>➤ Click to expand</b></summary>
///
/// <!-- wow - markdown tables strike again! -->
/// |slack markdown    |formatted result     |
/// |---               |---                  |
/// |`_italic_`        |_italic_             |
/// |`*bold*`          |**bold**             |
/// |`~strike~`        |<del>strike</del>    |
/// |`\n`              |line break           |
/// |`> a block quote` | <blockquote> a block quote </blockquote> |
/// |`` `some code!` ``| `some code!`        |
/// |`` ```multiline code\n2 lines!``` `` | <code>multiline code<br> 2 lines!</code> |
/// |` - li \n - li `  | <ul><li>li</li><li>li</li></ul> |
/// |<code>&lt;http://www.foo.com&#124;link name&gt;</code>| [link name](http://www.foo.com) |
/// |`:joy:` (list from [iamcal/emoji-data 🔗](https://github.com/iamcal/emoji-data)) | 😂 |
/// | link to #channel: `<#Cxxxxxx>` | [#channel](https://work.slack.com/some-public-channel) |
/// | link to @user: `<@Uxxxxxx>` | [@user](https://work.slack.com/some-user) |
/// | link to @user_group: `<!subteam^xxxxxx>` | [@user_group](https://work.slack.com/some-user-group) |
/// </details>
#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub struct Contents {
  text: String,

  #[serde(skip_serializing_if = "Option::is_none")]
  verbatim: Option<bool>,
}

impl Contents {
  /// Construct some markdown text from a string or string-like
  /// value
  ///
  /// # Arguments
  /// - `text` - The text contents to render for this `Text` object.
  ///     For some basic formatting examples, see the docs above for
  ///     the Contents struct itself, or [Slack's markdown docs 🔗].
  ///     There are no intrinsic length limits on this, those are usually
  ///     requirements of the context the text will be used in.
  ///
  /// [Slack's markdown docs 🔗]: https://api.slack.com/reference/surfaces/formatting
  ///
  /// # Example
  /// ```
  /// use slack_blocks::compose::text::{mrkdwn, Text};
  ///
  /// let text = mrkdwn::Contents::from_text("This link doesn't work! :tada: https://www.cheese.com")
  ///     .with_verbatim(true);
  /// ```
  pub fn from_text(text: impl ToString) -> Self {
    Into::<Self>::into(text.to_string())
  }

  /// Sets the `verbatim` flag
  ///
  /// # Arguments
  /// - `verbatim` - When set to false (as is default)
  ///     URLs will be auto-converted into links,
  ///     conversation names will be link-ified,
  ///     and certain mentions will be automatically parsed.
  ///     Using a value of true will skip any preprocessing
  ///     of this nature, although you can
  ///     still include manual parsing strings.
  ///
  /// # Example
  /// ```
  /// use slack_blocks::compose::text::{mrkdwn, Text};
  ///
  /// let text = mrkdwn::Contents::from_text("This link doesn't work! :tada: https://www.cheese.com")
  ///     .with_verbatim(true);
  /// ```
  pub fn with_verbatim(mut self, verbatim: bool) -> Self {
    self.verbatim = Some(verbatim);
    self
  }
}

impl AsRef<str> for Contents {
  fn as_ref(&self) -> &str {
    &self.text
  }
}

impl From<String> for Contents {
  fn from(text: String) -> Self {
    Self { text,
           verbatim: None }
  }
}

impl From<&str> for Contents {
  fn from(text: &str) -> Self {
    Self::from_text(text)
  }
}
