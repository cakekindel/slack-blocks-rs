//! # Text Object
//! [_slack api docs 🔗_](https://api.slack.com/reference/block-kit/composition-objects#text)
//!
//! An object containing some text,
//! formatted either as `plain_text`
//! or using [`mrkdwn` 🔗](https://api.slack.com/reference/surfaces/formatting),
//! our proprietary textual markup that's just different enough
//! from Markdown to frustrate you.

use serde::{Deserialize, Serialize};

use crate::convert;

#[doc(inline)]
pub mod mrkdwn;
#[doc(inline)]
pub mod plain;

#[doc(inline)]
pub use mrkdwn::Contents as Mrkdwn;
#[doc(inline)]
pub use plain::Contents as Plain;

/// Convenience trait to provide a little more meaning than
/// a call to `"foo".into()`, and shorter than `text::Plain::from("foo")`
pub trait ToSlackPlaintext: Sized + Into<Plain> {
  /// Convert to slack plain_text
  fn plaintext(self) -> Plain {
    self.into()
  }
}

impl<T: Into<Plain>> ToSlackPlaintext for T {}

/// Convenience trait to provide a little more meaning than
/// a call to `"foo".into()`, and shorter than `text::Mrkdwn::from("foo")`
pub trait ToSlackMarkdown: Sized + Into<Mrkdwn> {
  /// Convert to slack plain_text
  fn markdown(self) -> Mrkdwn {
    self.into()
  }
}

impl<T: Into<Mrkdwn>> ToSlackMarkdown for T {}

/// # Text Object
/// [_slack api docs 🔗_](https://api.slack.com/reference/block-kit/composition-objects#text)
///
/// An object containing some text,
/// formatted either as `plain_text`
/// or using [`mrkdwn` 🔗](https://api.slack.com/reference/surfaces/formatting),
/// our proprietary textual markup that's just different enough
/// from Markdown to frustrate you.
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Text {
  /// Markdown text
  Mrkdwn(mrkdwn::Contents),
  /// Plain text
  #[serde(rename = "plain_text")]
  Plain(plain::Contents),
}

impl Text {
  /// Clone the data behind a reference, then convert it into
  /// a `Text`
  ///
  /// # Arguments
  /// - `contents` - Anything that can be cloned into a type
  ///     that is convertable to a `Text` - this includes
  ///     the `Plain` and `Mrkdwn` contents structs.
  ///     Notably, this doesn't include a conversion directly
  ///     from a reference to a `String` or a `&str` - that's
  ///     because assuming which kind of text a string represents
  ///     could lead to unexpected behavior when that kind of text
  ///     isn't valid.
  pub fn copy_from<T: Into<Self> + Clone>(contents: &T) -> Self {
    contents.clone().into()
  }
}

convert!(impl From<mrkdwn::Contents> for Text => |contents| Text::Mrkdwn(contents));
convert!(impl From<plain::Contents> for Text => |contents| Text::Plain(contents));

impl AsRef<str> for Text {
  fn as_ref(&self) -> &str {
    match self {
      | Self::Mrkdwn(cts) => cts.as_ref(),
      | Self::Plain(cts) => cts.as_ref(),
    }
  }
}
