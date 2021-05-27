//! # XML macro support
//!
//! This module provides shorthands for builder functions
//! to be used with `mox` or a similar "xml -> Builder" macro.

pub use text::build::kind::{mrkdwn, plain};

use crate::{blocks, compose, elems, text};

/// Identity trait to appease the mox macro
pub trait IntoChild: Sized {
  /// Identity function
  fn into_child(self) -> Self {
    self
  }
}

impl<T> IntoChild for T {}

pub use mox_blocks::*;
pub use mox_compose::*;
pub use mox_elems::*;

mod mox_blocks {
  use super::*;

  /// # Build a section block
  ///
  /// ## Children
  /// Accepts at least one, and up to 10 text objects as children.
  ///
  /// ## Example
  /// ```
  /// use mox::mox;
  /// use slack_blocks::{blocks::Section, mox::*, text};
  ///
  /// let xml = mox! {
  ///   <section_block>
  ///     <text kind=plain>"Foo"</text>
  ///   </section_block>
  /// };
  ///
  /// let equivalent = Section::builder().text(text::Plain::from("Foo")).build();
  ///
  /// assert_eq!(xml, equivalent);
  /// ```
  pub fn section_block(
    )
      -> blocks::section::build::SectionBuilderInit<'static>
  {
    blocks::Section::builder()
  }

  /// # Build an input block
  ///
  /// ## Children
  /// Input requires a single child of a supported block element.
  ///
  /// For the list of supported elements, see `slack_blocks::blocks::input::SupportedElement`.
  ///
  /// ## Example
  /// ```
  /// use mox::mox;
  /// use slack_blocks::{blocks::Input, elems::TextInput, mox::*, text};
  ///
  /// let xml = mox! {
  ///   <input_block label="foo">
  ///     <text_input action_id="input" />
  ///   </input_block>
  /// };
  ///
  /// let equivalent =
  ///   Input::builder().label("foo")
  ///                   .element(TextInput::builder().action_id("input").build())
  ///                   .build();
  ///
  /// assert_eq!(xml, equivalent);
  /// ```
  pub fn input_block() -> blocks::input::build::InputBuilderInit<'static> {
    blocks::Input::builder()
  }

  /// # Build a context block
  ///
  /// ## Children
  /// Allows at least one, up to 10 text or img elements.
  ///
  /// ## Example
  /// ```
  /// use mox::mox;
  /// use slack_blocks::{blocks::Context, elems::Image, mox::*, text};
  ///
  /// let xml = mox! {
  ///   <context_block>
  ///     <text kind=plain>"Enjoy this picture of bar"</text>
  ///     <img src="https://foo.com/bar.png" alt="a pic of bar" />
  ///   </context_block>
  /// };
  ///
  /// let equivalent =
  ///   Context::builder().element(text::Plain::from("Enjoy this picture of bar"))
  ///                     .element(Image::builder().src("https://foo.com/bar.png")
  ///                                              .alt("a pic of bar")
  ///                                              .build())
  ///                     .build();
  ///
  /// assert_eq!(xml, equivalent);
  /// ```
  pub fn context_block(
    )
      -> blocks::context::build::ContextBuilderInit<'static>
  {
    blocks::Context::builder()
  }

  /// # Build a file block
  ///
  /// ## Children
  /// None
  ///
  /// ## Example
  /// ```
  /// use mox::mox;
  /// use slack_blocks::{blocks::File, mox::*};
  ///
  /// let xml = mox! {
  ///   <file_block external_id="foo" />
  /// };
  ///
  /// let equivalent = File::builder().external_id("foo").build();
  ///
  /// assert_eq!(xml, equivalent);
  /// ```
  pub fn file_block() -> blocks::file::build::FileBuilderInit<'static> {
    blocks::File::builder()
  }

  /// # Build an image block
  ///
  /// ## Children
  /// Allows at least one, up to 10 text or img elements.
  ///
  /// ## Example
  /// ```
  /// use mox::mox;
  /// use slack_blocks::{blocks::Image, mox::*};
  ///
  /// let xml = mox! {
  ///   <img_block src="https://foo.com/bar.png" alt="a pic of bar" />
  /// };
  ///
  /// let equivalent = Image::builder().src("https://foo.com/bar.png")
  ///                                  .alt("a pic of bar")
  ///                                  .build();
  ///
  /// assert_eq!(xml, equivalent);
  /// ```
  pub fn img_block() -> blocks::image::build::ImageBuilderInit<'static> {
    blocks::Image::builder()
  }
}

mod mox_elems {
  use super::*;

  /// # Build an text input element
  ///
  /// ## Children
  /// None.
  ///
  /// ## Example
  /// ```
  /// use mox::mox;
  /// use slack_blocks::{elems::TextInput, mox::*};
  ///
  /// let xml: TextInput = mox! {
  ///   <text_input action_id="name_input"
  ///               multiline=true
  ///               placeholder="Type your name"
  ///               length={1..=1000}
  ///   />
  /// };
  ///
  /// let equiv = TextInput::builder().action_id("name_input")
  ///                                 .multiline(true)
  ///                                 .placeholder("Type your name")
  ///                                 .length(1..=1000)
  ///                                 .build();
  ///
  /// assert_eq!(xml, equiv)
  /// ```
  pub fn text_input(
    )
      -> elems::text_input::build::TextInputBuilderInit<'static>
  {
    elems::TextInput::builder()
  }

  /// # Build an image element
  ///
  /// ## Children
  /// None.
  ///
  /// ## Example
  /// ```
  /// use mox::mox;
  /// use slack_blocks::{elems::Image, mox::*};
  ///
  /// let xml: Image = mox! {
  ///   <img src="https://foo.com/bar.png" alt="a pic of bar" />
  /// };
  ///
  /// let equiv = Image::builder().src("https://foo.com/bar.png")
  ///                             .alt("a pic of bar")
  ///                             .build();
  ///
  /// assert_eq!(xml, equiv)
  /// ```
  pub fn img() -> elems::image::build::ImageBuilderInit<'static> {
    elems::Image::builder()
  }

  /// # Build a button
  ///
  /// ## Children
  /// The text to display in the button
  ///
  /// ## Example
  /// ```
  /// use mox::mox;
  /// use slack_blocks::{elems::Button, mox::*};
  ///
  /// let xml: Button = mox! {
  ///   <button action_id="click_me">"Click me!"</button>
  /// };
  ///
  /// let equiv = Button::builder().action_id("click_me")
  ///                              .text("Click me!")
  ///                              .build();
  ///
  /// assert_eq!(xml, equiv)
  /// ```
  pub fn button() -> elems::button::build::ButtonBuilderInit<'static> {
    elems::Button::builder()
  }

  /// # Build a checkbox group
  ///
  /// ## Children
  /// Options to populate the checkbox group with. Min 1, max 10
  ///
  /// ## Example
  /// ```
  /// use mox::mox;
  /// use slack_blocks::{compose::Opt, elems::Checkboxes, mox::*};
  ///
  /// let xml: Checkboxes = mox! {
  ///   <checkboxes action_id="chex">
  ///     <option value="check_1">
  ///       <text kind=plain>"Foo"</text>
  ///     </option>
  ///     <option value="check_2">
  ///       <text kind=mrkdwn>"Bar"</text>
  ///     </option>
  ///     <option value="check_3">
  ///       <text kind=plain>"Zoo"</text>
  ///     </option>
  ///   </checkboxes>
  /// };
  ///
  /// let equiv = Checkboxes::builder().action_id("chex")
  ///                                  .option(Opt::builder().value("check_1")
  ///                                                        .text_plain("Foo")
  ///                                                        .build())
  ///                                  .option(Opt::builder().value("check_2")
  ///                                                        .text_md("Bar")
  ///                                                        .build())
  ///                                  .option(Opt::builder().value("check_3")
  ///                                                        .text_plain("Zoo")
  ///                                                        .build())
  ///                                  .build();
  ///
  /// assert_eq!(xml, equiv)
  /// ```
  pub fn checkboxes(
    )
      -> elems::checkboxes::build::CheckboxesBuilderInit<'static>
  {
    elems::Checkboxes::builder()
  }

  /// # Build a date picker
  ///
  /// ## Children
  /// None
  ///
  /// ## Example
  /// ```
  /// use mox::mox;
  /// use slack_blocks::{elems::DatePicker, mox::*};
  ///
  /// let xml = mox! {
  ///   <date_picker action_id="pick_birthday" placeholder="Pick your birthday!" />
  /// };
  ///
  /// let equiv = DatePicker::builder().action_id("pick_birthday")
  ///                                  .placeholder("Pick your birthday!")
  ///                                  .build();
  ///
  /// assert_eq!(xml, equiv)
  /// ```
  pub fn date_picker(
    )
      -> elems::date_picker::build::DatePickerBuilderInit<'static>
  {
    elems::DatePicker::builder()
  }

  /// # Build an overflow menu
  ///
  /// ## Children
  /// Options contained in the overflow menu. Min 2, max 5.
  ///
  /// ## Example
  /// ```
  /// use mox::mox;
  /// use slack_blocks::{compose::Opt, elems::Overflow, mox::*};
  ///
  /// let xml = mox! {
  ///   <overflow action_id="menu">
  ///     <option value="foo"><text kind=plain>"Foo"</text></option>
  ///     <option value="url" text_plain="Open link" url="foo.com" />
  ///   </overflow>
  /// };
  ///
  /// let equiv = Overflow::builder().action_id("menu")
  ///                                .option(Opt::builder().value("foo")
  ///                                                      .text_plain("Foo")
  ///                                                      .build())
  ///                                .option(Opt::builder().value("url")
  ///                                                      .text_plain("Open link")
  ///                                                      .url("foo.com")
  ///                                                      .build())
  ///                                .build();
  ///
  /// assert_eq!(xml, equiv)
  /// ```
  pub fn overflow() -> elems::overflow::build::OverflowBuilderInit<'static> {
    elems::Overflow::builder()
  }

  /// # Build a radio button group
  ///
  /// ## Children
  /// Options contained in the radio button group
  ///
  /// ## Example
  /// ```
  /// use mox::mox;
  /// use slack_blocks::{blocks::Input, compose::Opt, elems::Radio, mox::*};
  ///
  /// let xml = mox! {
  ///   <input_block label="Pick your favorite cheese!">
  ///     <radio_buttons action_id="cheese_picker">
  ///       <option value="feta"><text kind=plain>"Feta"</text></option>
  ///       <option value="gouda"><text kind=plain>"Gouda"</text></option>
  ///       <option value="cheddar"><text kind=plain>"Cheddar"</text></option>
  ///     </radio_buttons>
  ///   </input_block>
  /// };
  ///
  /// let radio = Radio::builder().action_id("cheese_picker")
  ///                             .option(Opt::builder().value("feta")
  ///                                                   .text_plain("Feta")
  ///                                                   .build())
  ///                             .option(Opt::builder().value("gouda")
  ///                                                   .text_plain("Gouda")
  ///                                                   .build())
  ///                             .option(Opt::builder().value("cheddar")
  ///                                                   .text_plain("Cheddar")
  ///                                                   .build())
  ///                             .build();
  ///
  /// let equiv = Input::builder().label("Pick your favorite cheese!")
  ///                             .element(radio)
  ///                             .build();
  ///
  /// assert_eq!(xml, equiv)
  /// ```
  pub fn radio_buttons() -> elems::radio::build::RadioBuilderInit<'static> {
    elems::Radio::builder()
  }
}

mod mox_compose {
  use super::*;

  /// # Text
  ///
  /// ## Children
  /// Accepts a `String`, `&str` or anything else that implements
  /// `AsRef<str>` as a child, representing the text that will be
  /// rendered in Slack.
  ///
  /// ## Example
  /// ```
  /// use mox::mox;
  /// use slack_blocks::{blocks::Section, mox::*, text};
  ///
  /// let xml = mox! {
  ///   <section_block>
  ///     <text kind=plain>"Foo"</text>
  ///   </section_block>
  /// };
  ///
  /// let equivalent = Section::builder().text(text::Plain::from("Foo")).build();
  ///
  /// assert_eq!(xml, equivalent);
  /// ```
  pub fn text() -> text::build::TextBuilderInit {
    text::Text::builder()
  }

  /// # Option
  ///
  /// ## Children
  /// The text to be displayed to the user.
  ///
  /// Note that text can't be provided as a child if you're setting `url`
  /// within an overflow menu - you need to set the text via a `text_plain` attribute.
  ///
  /// ## Example
  /// ```
  /// use mox::mox;
  /// use slack_blocks::{compose::Opt, mox::*};
  ///
  /// let xml = mox! {
  ///   <option value="foo">
  ///     <text kind=plain>"Foo"</text>
  ///   </option>
  /// };
  ///
  /// let equivalent = Opt::builder().text_plain("Foo").value("foo").build();
  ///
  /// assert_eq!(xml, equivalent);
  /// ```
  pub fn option() -> compose::opt::build::OptBuilderInit<'static> {
    compose::Opt::builder()
  }

  /// # Option Group
  ///
  /// ## Children
  /// Up to 100 option objects of the same type. Note that `syn-rsx` (and `mox` by extension)
  /// do not support using iterables as children.
  ///
  /// This means that if you have a `Vec<Opt>`,
  /// you'll need to pass it to an `options` attribute instead.
  ///
  /// See Examples for an example of compile-time child opts, and runtime (Vec) opts.
  ///
  /// ## Example - Options known at compile-time
  /// ```
  /// use mox::mox;
  /// use slack_blocks::{compose::{Opt, OptGroup},
  ///                    mox::*};
  ///
  /// let xml = mox! {
  ///   <option_group label="foos_and_bars">
  ///     <option value="foo"><text kind=plain>"Foo"</text></option>
  ///     <option value="bar"><text kind=plain>"Bar"</text></option>
  ///   </option_group>
  /// };
  ///
  /// let equivalent = OptGroup::builder().label("foos_and_bars")
  ///                                     .option(Opt::builder().value("foo")
  ///                                                           .text_plain("Foo")
  ///                                                           .build())
  ///                                     .option(Opt::builder().value("bar")
  ///                                                           .text_plain("Bar")
  ///                                                           .build())
  ///                                     .build();
  ///
  /// assert_eq!(xml, equivalent);
  /// ```
  ///
  /// ## Example - Dynamic vec of options
  /// ```
  /// use mox::mox;
  /// use slack_blocks::{compose::{Opt, OptGroup},
  ///                    mox::*};
  ///
  /// # fn uuid() -> String {"foo".to_string()}
  /// # fn random_word() -> String {"foo".to_string()}
  /// let generate_option = || {
  ///   mox! {
  ///     <option value={uuid()}>
  ///       <text kind=plain>{random_word()}</text>
  ///     </option>
  ///   }
  /// };
  ///
  /// // Generate 80 random options
  /// let options = std::iter::repeat(()).take(80)
  ///                                    .map(|_| generate_option())
  ///                                    .collect::<Vec<_>>();
  ///
  /// let xml = mox! {
  ///   <option_group label="foos_and_bars" options={options.clone()} />
  /// };
  ///
  /// let equivalent = OptGroup::builder().label("foos_and_bars")
  ///                                     .options(options.clone())
  ///                                     .build();
  ///
  /// assert_eq!(xml, equivalent);
  /// ```
  pub fn option_group(
    )
      -> compose::opt_group::build::OptGroupBuilderInit<'static>
  {
    compose::OptGroup::builder()
  }
}
