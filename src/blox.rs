//! # XML macro builder support

pub use elems::{button::Style::{Danger as btn_danger,
                                Primary as btn_primary},
                select::build::{choose::{multi, single},
                                data_source::{conversations,
                                              external,
                                              public_channels,
                                              static_,
                                              users}}};
pub use mox::mox as blox;
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

pub use blox_blocks::*;
pub use blox_compose::*;
pub use blox_elems::*;

mod blox_blocks {
  use super::*;

  /// Dummy builder so [blocks::Block::Divider] can be built with XML macro
  #[derive(Debug, Copy, Clone)]
  pub struct DividerBuilder;
  impl DividerBuilder {
    /// Constructs [blocks::Block::Divider]
    pub fn build(self) -> blocks::Block<'static> {
      blocks::Block::Divider
    }
  }

  /// # [`blocks::Actions`] - `<actions_block>`
  ///
  /// Build a [`blocks::Actions`]
  ///
  /// |Attribute|Type|Optional|Available as child|
  /// |-|-|-|-|
  /// |[`element`](blocks::actions::build::ActionsBuilder::element())|`impl Into<`[`blocks::actions::SupportedElement`]`>`|❌|✅|
  /// |[`block_id`](blocks::actions::build::ActionsBuilder::block_id())|[`String`] or [`&str`]|✅|❌|
  ///
  /// ## Example
  /// ```
  /// use slack_blocks::{blocks::Actions, blox::*, elems::Button, text};
  ///
  /// let xml = blox! {
  ///   <actions_block>
  ///     <button action_id="foo">"Foo"</button>
  ///     <button action_id="bar">"Bar"</button>
  ///   </actions_block>
  /// };
  ///
  /// let equivalent =
  ///   Actions::builder().element(Button::builder().action_id("foo")
  ///                                               .text("Foo")
  ///                                               .build())
  ///                     .element(Button::builder().action_id("bar")
  ///                                               .text("Bar")
  ///                                               .build())
  ///                     .build();
  ///
  /// assert_eq!(xml, equivalent);
  /// ```
  pub fn actions_block(
    )
      -> blocks::actions::build::ActionsBuilderInit<'static>
  {
    blocks::Actions::builder()
  }

  /// # [`blocks::Header`] - `<header_block>` or `<h1>`
  ///
  /// Build a [`blocks::Header`]
  ///
  /// ## Attributes
  /// |Attribute|Type|Optional|Available as child|
  /// |-|-|-|-|
  /// |[`text`](blocks::header::build::HeaderBuilder::text())|[`text::Plain`], [`String`], [`&str`]|❌|✅|
  /// |[`block_id`](blocks::header::build::HeaderBuilder::block_id())|[`String`] or [`&str`]|✅|❌|
  ///
  /// ## Example
  /// ```
  /// use slack_blocks::{blocks::Header, blox::*};
  ///
  /// let xml = blox! {
  ///   <h1>"Foo"</h1>
  /// };
  ///
  /// let equivalent = Header::builder().text("Foo").build();
  ///
  /// assert_eq!(xml, equivalent);
  /// ```
  pub fn header_block() -> blocks::header::build::HeaderBuilderInit<'static> {
    blocks::Header::builder()
  }

  /// Alias for [`header_block`]
  pub fn h1() -> blocks::header::build::HeaderBuilderInit<'static> {
    blocks::Header::builder()
  }

  /// # [`blocks::Block::Divider`] - `<divider_block />` or `<hr />`
  ///
  /// Build a [`blocks::Block::Divider`]
  ///
  /// ## Attributes
  /// None
  ///
  /// ## Example
  /// ```
  /// use slack_blocks::{blocks::Block, blox::*};
  ///
  /// let xml = blox! {
  ///   <hr />
  /// };
  ///
  /// let equivalent = Block::Divider;
  ///
  /// assert_eq!(xml, equivalent);
  /// ```
  pub fn divider_block() -> DividerBuilder {
    DividerBuilder
  }

  /// Alias for [`divider_block`]
  pub fn hr() -> DividerBuilder {
    divider_block()
  }

  /// # [`blocks::Section`] - `<section_block>`
  ///
  /// Build a [`blocks::Section`]
  ///
  /// ## Attributes
  /// |Attribute|Type|Optional|Available as child|
  /// |-|-|-|-|
  /// |[`text`](blocks::section::build::SectionBuilder::text())|[`text::Plain`], [`text::Mrkdwn`], or [`text::Text`]|❌*|❌|
  /// |[`field`](blocks::section::build::SectionBuilder::field())|[`text::Plain`], [`text::Mrkdwn`], or [`text::Text`]|❌*|✅|
  /// |[`fields`](blocks::section::build::SectionBuilder::fields())|[`IntoIterator`] over [`text::Text`]|❌*|❌|
  /// |[`accessory`](blocks::section::build::SectionBuilder::accessory())|[`elems::BlockElement`]|✅|❌|
  /// |[`block_id`](blocks::section::build::SectionBuilder::block_id())|[`String`] or [`&str`]|✅|❌|
  ///
  /// &#42; `text`, `field(s)`, or both are required.
  ///
  /// ## Example
  /// ```
  /// use slack_blocks::{blocks::Section, blox::*, text};
  ///
  /// let section_text = blox! { <text kind=plain>"Foo"</text> };
  ///
  /// let xml = blox! {
  ///   <section_block text=section_text>
  ///     <text kind=mrkdwn>"Bar"</text>
  ///   </section_block>
  /// };
  ///
  /// let equivalent = Section::builder().text(text::Plain::from("Foo"))
  ///                                    .field(text::Mrkdwn::from("Bar"))
  ///                                    .build();
  ///
  /// assert_eq!(xml, equivalent);
  /// ```
  pub fn section_block(
    )
      -> blocks::section::build::SectionBuilderInit<'static>
  {
    blocks::Section::builder()
  }

  /// # [`blocks::Input`] - `<input_block>`
  ///
  /// Build a [`blocks::Input`]
  ///
  /// ## Attributes
  /// |Attribute|Type|Optional|Available as child|
  /// |-|-|-|-|
  /// |[`label`](blocks::input::build::InputBuilder::label())|[`text::Plain`], [`text::Mrkdwn`], or [`text::Text`]|❌|❌|
  /// |[`element`](blocks::input::build::InputBuilder::element())|`impl Into<`[`blocks::input::SupportedElement`]`>`|❌|✅|
  /// |[`block_id`](blocks::input::build::InputBuilder::block_id())|[`String`] or [`&str`]|✅|❌|
  /// |[`hint`](blocks::input::build::InputBuilder::hint())|[`text::Plain`], [`String`], or [`&str`]|✅|❌|
  /// |[`dispatch_actions`](blocks::input::build::InputBuilder::dispatch_actions())|[`bool`]|✅|❌|
  /// |[`optional`](blocks::input::build::InputBuilder::optional())|[`bool`]|✅|❌|
  ///
  /// ## Example
  /// ```
  /// use slack_blocks::{blocks::Input, blox::*, elems::TextInput, text};
  ///
  /// let xml = blox! {
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

  /// # [`blocks::Context`] - `<context_block>`
  ///
  /// Build a [`blocks::Context`]
  ///
  /// ## Attributes
  /// |Attribute|Type|Optional|Available as child|
  /// |-|-|-|-|
  /// |[`element`](blocks::context::build::ContextBuilder::element())|[`text::Text`] or [`elems::Image`]|❌|✅|
  /// |[`block_id`](blocks::context::build::ContextBuilder::block_id())|[`String`] or [`&str`]|✅|❌|
  ///
  /// ## Example
  /// ```
  /// use slack_blocks::{blocks::Context, blox::*, elems::Image, text};
  ///
  /// let xml = blox! {
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

  /// # [`blocks::File`] - `<file_block>`
  ///
  /// Build a [`blocks::File`]
  ///
  /// ## Attributes
  /// |Attribute|Type|Optional|Available as child|
  /// |-|-|-|-|
  /// |[`external_id`](blocks::file::build::FileBuilder::external_id())|[`String`] or [`&str`]|❌|✅|
  /// |[`block_id`](blocks::file::build::FileBuilder::block_id())|[`String`] or [`&str`]|✅|❌|
  ///
  /// ## Example
  /// ```
  /// use slack_blocks::{blocks::File, blox::*};
  ///
  /// let xml = blox! {
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

  /// # [`blocks::Image`] - `<img_block>`
  ///
  /// Build a [`blocks::Image`]
  ///
  /// ## Attributes
  /// |Attribute|Type|Optional|Available as child|
  /// |-|-|-|-|
  /// |[`src`](blocks::image::build::ImageBuilder::src())|[`String`] or [`&str`]|❌|❌|
  /// |[`alt`](blocks::image::build::ImageBuilder::alt())|[`String`] or [`&str`]|❌|❌|
  /// |[`block_id`](blocks::file::build::FileBuilder::block_id())|[`String`] or [`&str`]|✅|❌|
  ///
  /// ## Example
  /// ```
  /// use slack_blocks::{blocks::Image, blox::*};
  ///
  /// let xml = blox! {
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

mod blox_elems {
  use super::*;

  /// # [`elems::TextInput`] - `<text_input>`
  ///
  /// Build a [`elems::TextInput`]
  ///
  /// ## Children
  /// None.
  ///
  /// ## Example
  /// ```
  /// use slack_blocks::{blox::*, elems::TextInput};
  ///
  /// let xml: TextInput = blox! {
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

  /// # [`elems::Image`] - `<img />`
  ///
  /// Build a [`elems::Image`]
  ///
  /// ## Children
  /// None.
  ///
  /// ## Example
  /// ```
  /// use slack_blocks::{blox::*, elems::Image};
  ///
  /// let xml: Image = blox! {
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

  /// # [`elems::Button`] - `<button>`
  ///
  /// Build a [`elems::Button`]
  ///
  /// ## Children
  /// The text to display in the button
  ///
  /// ## Example
  /// ```
  /// use slack_blocks::{blox::*, elems::Button};
  ///
  /// let xml: Button = blox! {
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

  /// # [`elems::Checkboxes`] - `<checkboxes>`
  ///
  /// Build a [`elems::Checkboxes`]
  ///
  /// ## Children
  /// Options to populate the checkbox group with. Min 1, max 10
  ///
  /// ## Example
  /// ```
  /// use slack_blocks::{blox::*, compose::Opt, elems::Checkboxes};
  ///
  /// let xml: Checkboxes = blox! {
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

  /// # [`elems::DatePicker`] - `<date_picker>`
  ///
  /// Build a [`elems::DatePicker`]
  ///
  /// ## Children
  /// None
  ///
  /// ## Example
  /// ```
  /// use slack_blocks::{blox::*, elems::DatePicker};
  ///
  /// let xml = blox! {
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

  /// # [`elems::Overflow`] - `<overflow>`
  ///
  /// Build a [`elems::Overflow`]
  ///
  /// ## Children
  /// Options contained in the overflow menu. Min 2, max 5.
  ///
  /// ## Example
  /// ```
  /// use slack_blocks::{blox::*, compose::Opt, elems::Overflow};
  ///
  /// let xml = blox! {
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

  /// # [`elems::Radio`] - `<radio_buttons>`
  ///
  /// Build a [`elems::Radio`]
  ///
  /// ## Children
  /// Options contained in the radio button group
  ///
  /// ## Example
  /// ```
  /// use slack_blocks::{blocks::Input, blox::*, compose::Opt, elems::Radio};
  ///
  /// let xml = blox! {
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

  /// # [`elems::select`] - `<select>`
  ///
  /// Build a [`elems::select`]
  ///
  /// # Attributes
  /// - `kind` (Optional): `single` or `multi` from `slack_blocks::blox`. Default is `single`.
  /// - `choose_from` (Required): `users`, `public_channels`, `static_`, `external`, `conversations` from `slack_blocks::blox`
  ///
  /// # Children
  /// For `static_`, 1-100 `<option>` children are allowed.
  ///
  /// For all others, none are allowed.
  ///
  /// # Errors you may encounter
  ///  - `Into<MyOpt> is not implemented for ...`: Make sure the `option` or `option_group` passed has the right `kind` of text for the select you're using.
  ///  - `AppendOptOrOptGroup<MyOpt> is not implemented for ...`: Make sure the `option` or `option_group` has no `url` and contains **plain** text.
  ///  - `Method `build` not found for...`: Make sure you've called all required methods. If you dig into the error message, you'll see `RequiredMethodNotCalled<method::foo>`, meaning you need to call `foo`.
  ///
  /// # Example - Select many Users
  /// ```
  /// use slack_blocks::elems::select;
  /// use slack_blocks::blox::*;
  ///
  /// let xml = blox! {
  ///   <select kind=multi choose_from=users placeholder="Pick some users!" action_id="foo" />
  /// };
  ///
  /// let equiv = select::multi::User::builder().placeholder("Pick some users!").action_id("foo").build();
  ///
  /// assert_eq!(xml, equiv)
  /// ```
  ///
  /// # Example - Select an option from a list defined by your app
  /// ```
  /// use slack_blocks::{elems::select, compose::Opt, blox::*};
  ///
  /// let xml = blox! {
  ///   <select choose_from=static_ placeholder="Pick your favorite cheese!" action_id="foo">
  ///     <option value="gouda"><text kind=plain>"Gouda"</text></option>
  ///     <option value="feta"><text kind=plain>"Feta"</text></option>
  ///     <option value="cheddar"><text kind=plain>"Cheddar"</text></option>
  ///     <option value="fontina"><text kind=plain>"Fontina"</text></option>
  ///   </select>
  /// };
  ///
  /// let equiv = select::Static::builder().placeholder("Pick your favorite cheese!")
  ///                                    .action_id("foo")
  ///                                    .option(Opt::builder().value("gouda").text_plain("Gouda").build())
  ///                                    .option(Opt::builder().value("feta").text_plain("Feta").build())
  ///                                    .option(Opt::builder().value("cheddar").text_plain("Cheddar").build())
  ///                                    .option(Opt::builder().value("fontina").text_plain("Fontina").build())
  ///                                    .build();
  ///
  /// assert_eq!(xml, equiv)
  /// ```
  pub fn select() -> elems::select::build::SelectBuilderInit {
    elems::select::build::SelectBuilderInit::new()
  }
}

mod blox_compose {
  use super::*;

  /// # [`compose::text`] - `<text>`
  ///
  /// Build a [`compose::text`]
  ///
  /// ## Children
  /// Accepts a `String`, `&str` or anything else that implements
  /// `AsRef<str>` as a child, representing the text that will be
  /// rendered in Slack.
  ///
  /// ## Example
  /// ```
  /// use slack_blocks::{blocks::Section, blox::*, text};
  ///
  /// let xml = blox! {
  ///   <text kind=plain>"Foo"</text>
  /// };
  ///
  /// let equivalent = text::Plain::from("Foo");
  ///
  /// assert_eq!(xml, equivalent);
  /// ```
  pub fn text() -> text::build::TextBuilderInit {
    text::Text::builder()
  }

  /// # [`compose::opt`] - `<option>`
  ///
  /// Build a [`compose::opt`]
  ///
  /// ## Children
  /// The text to be displayed to the user.
  ///
  /// Note that text can't be provided as a child if you're setting `url`
  /// within an overflow menu - you need to set the text via a `text_plain` attribute.
  ///
  /// ## Example
  /// ```
  /// use slack_blocks::{blox::*, compose::Opt};
  ///
  /// let xml = blox! {
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

  /// # [`compose::opt_group`] - `<option_group>`
  ///
  /// Build a [`compose::opt_group`]
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
  /// use slack_blocks::{blox::*,
  ///                    compose::{Opt, OptGroup}};
  ///
  /// let xml = blox! {
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
  /// use slack_blocks::{blox::*,
  ///                    compose::{Opt, OptGroup}};
  ///
  /// # fn uuid() -> String {"foo".to_string()}
  /// # fn random_word() -> String {"foo".to_string()}
  /// let generate_option = || {
  ///   blox! {
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
  /// let xml = blox! {
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

  /// # [`compose::Confirm`] - `<confirm>`
  ///
  /// Build a [`compose::Confirm`]
  ///
  /// ## Children
  /// No children.
  ///
  /// ## Example
  /// ```
  /// use slack_blocks::{blox::*, compose::Confirm, text::ToSlackPlaintext};
  ///
  /// let xml = blox! {
  ///   <confirm title="Title"
  ///            text=blox!{<text kind=plain>"Body"</text>}
  ///            confirm="Yes"
  ///            deny="No"
  ///   />
  /// };
  ///
  /// let equivalent = Confirm::builder().title("Title")
  ///                                    .text("Body".plaintext())
  ///                                    .confirm("Yes")
  ///                                    .deny("No")
  ///                                    .build();
  ///
  /// assert_eq!(xml, equivalent);
  /// ```
  pub fn confirm() -> compose::confirm::build::ConfirmBuilderInit {
    compose::Confirm::builder()
  }
}
