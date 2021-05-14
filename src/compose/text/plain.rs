use serde::{Deserialize, Serialize};

/// Literally just plain text, with the only formatting available being emojis.
#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub struct Contents {
  text: String,

  #[serde(skip_serializing_if = "Option::is_none")]
  emoji: Option<bool>,
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

  /// Sets the `emoji` flag
  ///
  /// # Arguments
  /// - `emoji` - Indicates whether emojis in a text field should be
  ///     escaped into the colon emoji format
  ///
  /// # Example
  /// ```
  /// use slack_blocks::compose::text::{plain, Text};
  ///
  /// let text = plain::Contents::from_text("Emojis!! :tada:").with_emoji(true);
  /// ```
  pub fn with_emoji(mut self, emoji: bool) -> Self {
    self.emoji = Some(emoji);
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
    Self { text, emoji: None }
  }
}

impl From<&str> for Contents {
  fn from(text: &str) -> Self {
    Self::from_text(text)
  }
}
