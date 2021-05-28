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
  /// Build a new Text object
  ///
  /// See TextBuilder for example
  pub fn builder() -> build::TextBuilderInit {
    build::TextBuilderInit::new()
  }

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

/// Text builder
pub mod build {
  use std::marker::PhantomData;

  use super::*;
  use crate::build::*;

  /// Builder methods
  #[allow(non_camel_case_types)]
  pub mod method {
    /// TextBuilder.text
    #[derive(Copy, Clone, Debug)]
    pub struct text;

    /// TextBuilder.plain or TextBuilder.mrkdwn
    #[derive(Copy, Clone, Debug)]
    pub struct plain_or_mrkdwn;
  }

  /// "Text Kind" for XML macro
  #[allow(non_camel_case_types)]
  #[cfg(feature = "xml")]
  pub mod kind {
    use super::*;
    // This trick is kind of nasty. Essentially, I want the API to look like
    // `<text plain>"Foo"</text>`, not `<text_plain>"Foo"</text_plain>`,
    // with an attribute setting the text kind.
    //
    // mox does not support nullary function attributes, so I _could_ make
    // `TextBuilder.plain()` and `TextBuilder.mrkdwn()` accept a unit argument,
    // but that would look fugly and impact non-XML use-cases.
    // (`<text plain=()>"Foo"</text>`, `Text::builder().plain(()).text("Foo").build()`)
    //
    // So I settled on having types named `mrkdwn` and `plain` exported from `crate::mox`
    // and a `TextMethod.kind()` method -
    // (`<text kind=mrkdwn></text>`)
    // this feels good to write, looks good to read, the only cons are implementation complexity
    // and potential clash with locals named `mrkdwn` or `plain`.
    /// Static marker trait tying the token "mrkdwn" to the type "Mrkdwn",
    /// and "plain" to "Plain"
    pub trait TextKind<T> {
      /// Invoke `TextBuilder.mrkdwn` or `TextBuilder.plain`
      fn set_kind<M>(self, builder: TextBuilder<Text, M>) -> TextBuilder<T, M>;
    }

    /// Markdown text
    #[derive(Copy, Clone, Debug)]
    pub struct mrkdwn;

    /// Plain text
    #[derive(Copy, Clone, Debug)]
    pub struct plain;

    impl TextKind<Mrkdwn> for mrkdwn {
      fn set_kind<M>(self,
                     builder: TextBuilder<Text, M>)
                     -> TextBuilder<Mrkdwn, M> {
        builder.mrkdwn()
      }
    }

    impl TextKind<Plain> for plain {
      fn set_kind<M>(self,
                     builder: TextBuilder<Text, M>)
                     -> TextBuilder<Plain, M> {
        builder.plain()
      }
    }
  }

  /// Initial state for Text Builder
  pub type TextBuilderInit =
    TextBuilder<Text, RequiredMethodNotCalled<method::text>>;

  /// # Text Builder
  ///
  /// Allows you to construct safely, with compile-time checks
  /// on required setter methods.
  ///
  /// # Required Methods
  /// `TextBuilder::build()` is only available if these methods have been called:
  ///  - `text`
  ///  - `plain` or `mrkdwn`
  ///
  /// ```
  /// use slack_blocks::text::Text;
  ///
  /// let foo = Text::builder().plain().text("foo").build();
  /// ```
  #[derive(Debug)]
  pub struct TextBuilder<T, TMarker> {
    text: Option<Text>,
    text_value: Option<String>,
    state: PhantomData<(T, TMarker)>,
  }

  impl<T> TextBuilder<T, RequiredMethodNotCalled<method::text>> {
    /// Construct a new text builder
    pub fn new() -> Self {
      Self { text: None,
             text_value: None,
             state: PhantomData::<_> }
    }

    /// Set `text` (**Required**)
    ///
    /// The text contents to render for this `Text` object.
    ///
    /// For some basic formatting examples, see the docs for
    /// the `text::Mrkdwn`, or [Slack's markdown docs 🔗].
    ///
    /// There are no intrinsic length limits on this, those are usually
    /// requirements of the context the text will be used in.
    ///
    /// [Slack's markdown docs 🔗]: https://api.slack.com/reference/surfaces/formatting
    pub fn text(mut self,
                t: impl AsRef<str>)
                -> TextBuilder<T, Set<method::text>> {
      let text = t.as_ref().to_string();

      match self.text {
        | Some(Text::Mrkdwn(ref mut t)) => {
          t.text = text;
        },
        | Some(Text::Plain(ref mut t)) => {
          t.text = text;
        },
        | None => self.text_value = Some(text),
      };

      TextBuilder { text: self.text,
                    text_value: self.text_value,
                    state: PhantomData::<_> }
    }

    /// Alias of `text` for XML macros, allowing text
    /// to be set as a string literal instead of an attribute.
    ///
    /// ```
    /// use mox::mox;
    /// use slack_blocks::mox::*;
    ///
    /// let as_attr = mox! {
    ///   <text_plain text="Foo" />
    /// };
    ///
    /// let as_child = mox! {
    ///   <text_plain>"Foo"</text_plain>
    /// };
    ///
    /// assert_eq!(as_attr, as_child);
    /// ```
    #[cfg(feature = "xml")]
    pub fn child(self,
                 t: impl AsRef<str>)
                 -> TextBuilder<T, Set<method::text>> {
      self.text(t)
    }
  }

  impl<M> TextBuilder<Text, M> {
    /// Set the kind of the text you're building (**Required**)
    ///
    /// Intended to be used as an XML attribute with `build::kind::mrkdwn` or `build::kind::plain`
    ///
    /// ```
    /// use mox::mox;
    /// use slack_blocks::{mox::*, text};
    ///
    /// let xml = mox! {<text kind=plain>"Foo"</text>};
    ///
    /// let builder = text::Plain::from("Foo");
    ///
    /// assert_eq!(xml, builder)
    /// ```
    #[cfg(feature = "xml")]
    pub fn kind<T, K>(self, kind: K) -> TextBuilder<T, M>
      where T: Into<Text>,
            K: kind::TextKind<T>
    {
      kind.set_kind(self)
    }

    /// Set the text you're building to be `plain_text` (**Required**)
    pub fn plain(self) -> TextBuilder<Plain, M> {
      let text = Some(Plain::from(self.text_value.unwrap_or_default()).into());
      TextBuilder { text,
                    text_value: None,
                    state: PhantomData::<_> }
    }

    /// Set the text you're building to be `mrkdwn` (**Required**)
    pub fn mrkdwn(self) -> TextBuilder<Mrkdwn, M> {
      let text = Some(Mrkdwn::from(self.text_value.unwrap_or_default()).into());
      TextBuilder { text,
                    text_value: None,
                    state: PhantomData::<_> }
    }
  }

  impl<M> TextBuilder<Mrkdwn, M> {
    /// Set `verbatim` (Optional)
    ///
    /// When set to false (as is default)
    /// URLs will be auto-converted into links,
    /// conversation names will be link-ified,
    /// and certain mentions will be automatically parsed.
    ///
    /// Using a value of true will skip any preprocessing
    /// of this nature, although you can
    /// still include manual parsing strings.
    pub fn verbatim(mut self) -> Self {
      if let Some(Text::Mrkdwn(ref mut m)) = self.text {
        m.verbatim = Some(true);
      }

      self
    }
  }

  impl<M> TextBuilder<Plain, M> {
    /// Set `emoji` (Optional)
    ///
    /// Indicates whether emojis in a text field should be
    /// escaped into the colon emoji format
    pub fn emoji(mut self) -> Self {
      if let Some(Text::Plain(ref mut p)) = self.text {
        p.emoji = Some(true);
      }

      self
    }
  }

  impl TextBuilder<Plain, Set<method::text>> {
    /// All done building, now give me a darn text object!
    ///
    /// > `no method name 'build' found for struct 'TextBuilder<...>'`?
    /// Make sure all required setter methods have been called. See docs for `TextBuilder`.
    ///
    /// ```compile_fail
    /// use slack_blocks::text::Text;
    ///
    /// let foo = Text::builder().build(); // Won't compile!
    /// ```
    ///
    /// ```
    /// use slack_blocks::text::Text;
    ///
    /// let foo = Text::builder().plain().emoji().text("foo :joy:").build();
    /// ```
    pub fn build(self) -> Plain {
      match self.text.unwrap() {
        | Text::Plain(p) => p,
        | _ => unreachable!("type marker says this should be plain."),
      }
    }
  }

  impl TextBuilder<Mrkdwn, Set<method::text>> {
    /// All done building, now give me a darn text object!
    ///
    /// > `no method name 'build' found for struct 'TextBuilder<...>'`?
    /// Make sure all required setter methods have been called. See docs for `TextBuilder`.
    ///
    /// ```compile_fail
    /// use slack_blocks::text::Text;
    ///
    /// let foo = Text::builder().build(); // Won't compile!
    /// ```
    ///
    /// ```
    /// use slack_blocks::text::Text;
    ///
    /// let foo = Text::builder().mrkdwn()
    ///                          .verbatim()
    ///                          .text("foo :joy:")
    ///                          .build();
    /// ```
    pub fn build(self) -> Mrkdwn {
      match self.text.unwrap() {
        | Text::Mrkdwn(p) => p,
        | _ => unreachable!("type marker says this should be markdown."),
      }
    }
  }
}

impl AsRef<str> for Text {
  fn as_ref(&self) -> &str {
    match self {
      | Self::Mrkdwn(cts) => cts.as_ref(),
      | Self::Plain(cts) => cts.as_ref(),
    }
  }
}
