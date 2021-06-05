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
  /// |[`text`]     | [`text::Plain`], [`text::Mrkdwn`], or [`text::Text`]|❌*|❌|
  /// |[`field`]    | [`text::Plain`], [`text::Mrkdwn`], or [`text::Text`]|❌*|✅|
  /// |[`fields`]   | [`IntoIterator`] over [`text::Text`]                |❌*|❌|
  /// |[`accessory`]| [`elems::BlockElement`]                             |✅ |❌|
  /// |[`block_id`] | [`String`] or [`&str`]                              |✅ |❌|
  ///
  /// &#42; `text`, `field(s)`, or both are required.
  ///
  /// [`text`]: blocks::section::build::SectionBuilder::text()
  /// [`field`]: blocks::section::build::SectionBuilder::field()
  /// [`fields`]: blocks::section::build::SectionBuilder::fields()
  /// [`accessory`]: blocks::section::build::SectionBuilder::accessory()
  /// [`block_id`]: blocks::section::build::SectionBuilder::block_id()
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
  /// |[`label`](blocks::input::build::InputBuilder::label())|[`text::Plain`], [`text::Mrkdwn`], or [`text::Text`] ([`<text>`](super::text()))|❌|❌|
  /// |[`element`](blocks::input::build::InputBuilder::element())|`impl Into<`[`blocks::input::SupportedElement`]`>`|❌|✅|
  /// |[`block_id`](blocks::input::build::InputBuilder::block_id())|[`String`] or [`&str`]|✅|❌|
  /// |[`hint`](blocks::input::build::InputBuilder::hint())|[`text::Plain`] ([`<text>`](super::text())), [`String`], or [`&str`]|✅|❌|
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
  /// |[`element`](blocks::context::build::ContextBuilder::element())|[`text::Text`] ([`<text>`](super::text())) or [`elems::Image`] ([`<img>`](super::img()))|❌|✅|
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
  /// ## Attributes
  /// |Attribute|Type|Optional|Available as child|
  /// |-|-|-|-|
  /// |[`action_id`]      | [`String`] or [`&str`]                      |❌|❌|
  /// |[`action_trigger`] | [`elems::text_input::ActionTrigger`]        |✅|❌|
  /// |[`placeholder`]    | [`text::Plain`] ([`<text>`](super::text())), [`String`] or [`&str`]     |✅|❌|
  /// |[`initial_value`]  | [`String`] or [`&str`]                      |✅|❌|
  /// |[`length`]         | impl [`std::ops::RangeBounds`] over [`u32`] |✅|❌|
  /// |[`min_length`]     | [`u32`]                                     |✅|❌|
  /// |[`max_length`]     | [`u32`]                                     |✅|❌|
  /// |[`multiline`]      | [`bool`]                                    |✅|❌|
  ///
  /// [`action_id`]:      elems::text_input::build::TextInputBuilder::action_id()
  /// [`action_trigger`]: elems::text_input::build::TextInputBuilder::action_trigger()
  /// [`placeholder`]:    elems::text_input::build::TextInputBuilder::placeholder()
  /// [`initial_value`]:  elems::text_input::build::TextInputBuilder::initial_value()
  /// [`length`]:         elems::text_input::build::TextInputBuilder::length()
  /// [`min_length`]:     elems::text_input::build::TextInputBuilder::min_length()
  /// [`max_length`]:     elems::text_input::build::TextInputBuilder::max_length()
  /// [`multiline`]:      elems::text_input::build::TextInputBuilder::multiline()
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
  /// ## Attributes
  /// |Attribute|Type|Optional|Available as child|
  /// |-|-|-|-|
  /// |[`src`](elems::image::build::ImageBuilder::src()) | [`String`] or [`&str`] |❌|❌|
  /// |[`alt`](elems::image::build::ImageBuilder::alt()) | [`String`] or [`&str`] |❌|❌|
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
  /// ## Attributes
  /// |Attribute|Type|Optional|Available as child|
  /// |-|-|-|-|
  /// |[`action_id`] | [`String`] or [`&str`]                                              |❌|❌|
  /// |[`text`]      | [`text::Plain`] ([`<text>`](super::text())), [`String`] or [`&str`] |❌|✅|
  /// |[`url`]       | [`String`] or [`&str`]                                              |✅|❌|
  /// |[`value`]     | [`String`] or [`&str`]                                              |✅|❌|
  /// |[`style`]     | [`elems::button::Style`] ([`btn_primary`] or [`btn_danger`])        |✅|❌|
  /// |[`confirm`]   | [`compose::Confirm`] ([`<confirm>`](super::confirm()))              |✅|❌|
  ///
  /// [`action_id`]: elems::button::build::ButtonBuilder::action_id()
  /// [`text`]: elems::button::build::ButtonBuilder::text()
  /// [`url`]: elems::button::build::ButtonBuilder::url()
  /// [`value`]: elems::button::build::ButtonBuilder::value()
  /// [`style`]: elems::button::build::ButtonBuilder::style()
  /// [`confirm`]: elems::button::build::ButtonBuilder::confirm()
  ///
  /// ## Example
  /// ```
  /// use slack_blocks::{blox::*,
  ///                    elems::button::{Button, Style}};
  ///
  /// let xml: Button = blox! {
  ///   <button action_id="dangerous" style=btn_danger>"DANGER!"</button>
  /// };
  ///
  /// let equiv = Button::builder().action_id("dangerous")
  ///                              .text("DANGER!")
  ///                              .style(Style::Danger)
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
  /// ## Attributes
  /// |Attribute|Type|Optional|Available as child|
  /// |-|-|-|-|
  /// |[`action_id`]       | [`String`] or [`&str`]                                               |❌|❌|
  /// |[`option`]          | [`compose::Opt`] ([`<option>`](super::option()))*                    |❌ _(or `options`)_|✅|
  /// |[`options`]         | [`Vec`] of [`compose::Opt`]*                                         |❌ _(or `option`)_|❌|
  /// |[`initial_options`] | [`Vec`] of [`compose::Opt`]* provided via [`option`] or [`options`]. |✅|❌|
  /// |[`confirm`]         | [`compose::Confirm`] ([`<confirm>`](super::confirm()))               |✅|❌|
  ///
  /// [`action_id`]: elems::checkboxes::build::CheckboxesBuilder::action_id()
  /// [`option`]: elems::checkboxes::build::CheckboxesBuilder::option()
  /// [`options`]: elems::checkboxes::build::CheckboxesBuilder::options()
  /// [`initial_options`]: elems::checkboxes::build::CheckboxesBuilder::initial_options()
  /// [`confirm`]: elems::checkboxes::build::CheckboxesBuilder::confirm()
  ///
  /// &#42; Options cannot have URL set, option text can be mrkdwn or plain.
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
  /// ## Attributes
  /// |Attribute|Type|Optional|Available as child|
  /// |-|-|-|-|
  /// |[`action_id`]    | [`String`] or [`&str`]                                 |❌|❌|
  /// |[`placeholder`]  | [`String`] or [`&str`]                                 |✅|❌|
  /// |[`initial_date`] | ([`u8`] _day_, [`u8`] _month_, [`u16`] _year_)         |✅|❌|
  /// |[`confirm`]      | [`compose::Confirm`] ([`<confirm>`](super::confirm())) |✅|❌|
  ///
  /// [`action_id`]: elems::date_picker::build::DatePickerBuilder::action_id()
  /// [`placeholder`]: elems::date_picker::build::DatePickerBuilder::placeholder()
  /// [`initial_date`]: elems::date_picker::build::DatePickerBuilder::initial_date()
  /// [`confirm`]: elems::date_picker::build::DatePickerBuilder::confirm()
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
  /// ## Attributes
  /// |Attribute|Type|Optional|Available as child|
  /// |-|-|-|-|
  /// |[`action_id`] | [`String`] or [`&str`]                                 |❌|❌|
  /// |[`option`]    | [`compose::Opt`] ([`<option>`](super::option()))*      |❌ _(or options)_|✅|
  /// |[`options`]   | [`Vec`] of [`compose::Opt`]*                           |❌ _(or option)_|❌|
  /// |[`confirm`]   | [`compose::Confirm`] ([`<confirm>`](super::confirm())) |✅|❌|
  ///
  /// [`action_id`]: elems::overflow::build::OverflowBuilder::action_id()
  /// [`option`]: elems::overflow::build::OverflowBuilder::option()
  /// [`options`]: elems::overflow::build::OverflowBuilder::options()
  /// [`confirm`]: elems::overflow::build::OverflowBuilder::confirm()
  ///
  /// &#42; Options **can** have URL set, option text must be plain.
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
  /// ## Attributes
  /// |Attribute|Type|Optional|Available as child|
  /// |-|-|-|-|
  /// |[`action_id`]      | [`String`] or [`&str`]                                 |❌|❌|
  /// |[`option`]         | [`compose::Opt`] ([`<option>`](super::option()))*      |❌ _(or options)_|✅|
  /// |[`options`]        | [`Vec`] of [`compose::Opt`]*                           |❌ _(or option)_|❌|
  /// |[`initial_option`] | [`compose::Opt`]*                                      |✅|❌|
  /// |[`confirm`]        | [`compose::Confirm`] ([`<confirm>`](super::confirm())) |✅|❌|
  ///
  /// [`action_id`]: elems::radio::build::RadioBuilder::action_id()
  /// [`option`]: elems::radio::build::RadioBuilder::option()
  /// [`options`]: elems::radio::build::RadioBuilder::options()
  /// [`initial_option`]: elems::radio::build::RadioBuilder::initial_option()
  /// [`confirm`]: elems::radio::build::RadioBuilder::confirm()
  ///
  /// &#42; Options **cannot** have URL set, option text can be mrkdwn or plain.
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
  /// let equiv = {
  ///   let feta = Opt::builder().value("feta").text_plain("Feta").build();
  ///
  ///   let gouda = Opt::builder().value("gouda").text_plain("Gouda").build();
  ///
  ///   let cheddar = Opt::builder().value("cheddar")
  ///                               .text_plain("Cheddar")
  ///                               .build();
  ///
  ///   let radio = Radio::builder().action_id("cheese_picker")
  ///                               .option(feta)
  ///                               .option(gouda)
  ///                               .option(cheddar)
  ///                               .build();
  ///
  ///   Input::builder().label("Pick your favorite cheese!")
  ///                   .element(radio)
  ///                   .build()
  /// };
  ///
  /// assert_eq!(xml, equiv)
  /// ```
  pub fn radio_buttons() -> elems::radio::build::RadioBuilderInit<'static> {
    elems::Radio::builder()
  }

  /// # [`elems::select`] - `<select>`
  ///
  /// Build a [`elems::select`].
  ///
  /// # Attributes
  ///
  /// This element behaves like a flow chart, allowing you to construct all
  /// Single and Multi-selects with a single XML element.
  ///
  /// Use the `kind` attribute to distinguish between
  /// single- and multi-selects.
  ///
  /// Use the `choose_from` attribute to distinguish between
  /// the many sources of options for the select menu.
  ///
  /// ## Common
  /// |Attribute|Type|Optional|Available as child|
  /// |-|-|-|-|
  /// |[`kind`]        | [`single`] (default) or [`multi`]                                            |✅|❌|
  /// |[`choose_from`] | [`users`], [`public_channels`], [`static_`], [`external`], [`conversations`] |❌|❌|
  /// |`action_id`     | [`String`] or [`&str`]                                                       |❌|❌|
  /// |`placeholder`   | [`String`] or [`&str`]                                                       |✅|❌|
  /// |`confirm`       | [`compose::Confirm`] ([`<confirm>`](super::confirm()))                       |✅|❌|
  /// |**when kind=multi**<br/>`max_selected_items` | [`u32`] |✅|❌|
  ///
  /// [`kind`]: elems::select::build::SelectBuilder::kind()
  /// [`choose_from`]: elems::select::build::SelectBuilder::choose_from()
  ///
  /// ## **When `choose_from=users`**
  /// |Attribute|Type|Optional|Available as child|
  /// |-|-|-|-|
  /// |**when kind=single**<br/>[`initial_user`]| [`String`] or [`&str`] |✅|❌|
  /// |**when kind=multi**<br/>[`initial_users`]| impl [`IntoIterator`] over [`String`] or [`&str`] |✅|❌|
  ///
  /// [`initial_user`]: elems::select::user::build::UserBuilder::initial_user()
  /// [`initial_users`]: elems::select::user::build::UserBuilder::initial_users()
  ///
  /// ## **When `choose_from=public_channels`**
  /// |Attribute|Type|Optional|Available as child|
  /// |-|-|-|-|
  /// |**when kind=single**<br/>[`initial_channel`]| [`String`] or [`&str`] |✅|❌|
  /// |**when kind=multi**<br/>[`initial_channels`]| impl [`IntoIterator`] over [`String`] or [`&str`] |✅|❌|
  ///
  /// [`initial_channel`]: elems::select::public_channel::build::PublicChannelBuilder::initial_channel()
  /// [`initial_channels`]: elems::select::public_channel::build::PublicChannelBuilder::initial_channels()
  ///
  /// ## **When `choose_from=external`**
  /// |Attribute|Type|Optional|Available as child|
  /// |-|-|-|-|
  /// |[`min_query_length`] | [`u64`] |✅|❌|
  /// |**when kind=single**<br/>[`initial_option`]| [`compose::Opt`]* |✅|❌|
  /// |**when kind=single**<br/>[`initial_option_group`]| [`compose::OptGroup`]* |✅|❌|
  /// |**when kind=multi**<br/>[`initial_options`]| impl [`IntoIterator`] over [`compose::Opt`]s*  |✅|❌|
  /// |**when kind=multi**<br/>[`initial_option_groups`]| impl [`IntoIterator`] over [`compose::OptGroup`]s*  |✅|❌|
  ///
  /// &#42; Options **cannot** have URL set, option text must be plain.
  ///
  /// [`initial_option`]: elems::select::external::build::ExternalBuilder::initial_option()
  /// [`initial_option_group`]: elems::select::external::build::ExternalBuilder::initial_option_group()
  /// [`initial_option_groups`]: elems::select::external::build::ExternalBuilder::initial_option_groups()
  /// [`initial_options`]: elems::select::external::build::ExternalBuilder::initial_options()
  /// [`min_query_length`]: elems::select::external::build::ExternalBuilder::min_query_length()
  ///
  /// ## **When `choose_from=conversations`**
  /// |Attribute|Type|Optional|Available as child|
  /// |-|-|-|-|
  /// |[`filter`] | [`compose::ConversationFilter`] |✅|❌|
  /// |[`initial_channel_current`] | [`bool`] |✅|❌|
  /// |**when kind=single**<br/>[`initial_channel`]| [`String`] or [`&str`] |✅|❌|
  /// |**when kind=multi**<br/>[`initial_channels`]| impl [`IntoIterator`] over [`String`]s or [`&str`]s |✅|❌|
  ///
  /// [`filter`]: elems::select::conversation::build::ConversationBuilder::filter()
  /// [`initial_channel_current`]: elems::select::conversation::build::ConversationBuilder::initial_channel_current()
  /// [`initial_channel`]: elems::select::conversation::build::ConversationBuilder::initial_channel()
  /// [`initial_channels`]: elems::select::conversation::build::ConversationBuilder::initial_channels()
  ///
  /// ## **When `choose_from=static_`**
  /// This is the most complex select menu.
  ///
  /// It allows you to define the options in your app
  /// by using children or `options`/`option_groups`.
  ///
  /// Note that all children must **either** be `<option>`s or `<option_group>`s.
  /// You cannot mix options and groups in the same static select menu.
  ///
  /// For single-selects that use options, you can pre-select one of them with `initial_option`.
  ///
  /// For multi-selects that use options, you can pre-select any number of them with `initial_options`.
  ///
  /// For single-selects that use option groups, you can pre-select one of them with `initial_option_group`.
  ///
  /// For multi-selects that use option groups, you can pre-select any number of them with `initial_option_groups`.
  ///
  /// |Attribute|Type|Optional|Available as child|
  /// |-|-|-|-|
  /// | Child | [`compose::Opt`] ([`<option>`](super::option())) or [`compose::OptGroup`] ([`<option_group>`](super::option_group()))* |❌ _(or `options` or `option_groups`)_|✅|
  /// |[`option_groups`] | [`Vec`] of [`compose::OptGroup`]*                                                                           |❌ _(or Child or `option_groups`)_|❌|
  /// |[`options`]       | [`Vec`] of [`compose::Opt`]*                                                                                |❌ _(or Child or `options`)_ |❌|
  /// |**when kind=single**<br/>**and select uses Opts**<br/>(not OptGroups)<br/>[`initial_option`]| [`compose::Opt`]* |✅|❌|
  /// |**when kind=single**<br/>**and select uses OptGroup**<br/>(not Opts)<br/>[`initial_option_group`]| [`compose::OptGroup`]* |✅|❌|
  /// |**when kind=multi**<br/>**and select uses Opt**<br/>(not OptGroups)<br/>[`initial_options`]| impl [`IntoIterator`] over [`compose::Opt`]* |✅|❌|
  /// |**when kind=multi**<br/>**and select uses OptGroup**<br/>(not Opts)<br/>[`initial_option_groups`]| impl [`IntoIterator`] over [`compose::OptGroup`]* |✅|❌|
  ///
  /// &#42; All children must either be options or option groups. Options **cannot** have URL set, option text must be plain.
  ///
  /// [`initial_option`]: elems::select::static_::build::StaticBuilder::initial_option()
  /// [`initial_options`]: elems::select::static_::build::StaticBuilder::initial_options()
  /// [`initial_option_group`]: elems::select::static_::build::StaticBuilder::initial_option_group()
  /// [`initial_option_groups`]: elems::select::static_::build::StaticBuilder::initial_option_groups()
  /// [`options`]: elems::select::static_::build::StaticBuilder::options()
  /// [`option_groups`]: elems::select::static_::build::StaticBuilder::option_groups()
  ///
  /// # Examples
  ///
  /// ## `choose_from=users`
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
  /// ## `choose_from=static_`
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
  /// ## Attributes
  /// |Attribute|Type|Optional|Available as child|
  /// |-|-|-|-|
  /// |[`kind`]                              | [`mrkdwn`] or [`plain`] |❌|❌|
  /// |[`text`]                              | [`&str`] or [`String`]  |❌|✅|
  /// |**When kind=plain**<br/>[`emoji`]     | [`bool`]                |✅|❌|
  /// |**When kind=mrkdwn**<br/>[`verbatim`] | [`bool`]                |✅|❌|
  ///
  /// [`kind`]: compose::text::build::TextBuilder::kind()
  /// [`text`]: compose::text::build::TextBuilder::text()
  /// [`emoji`]: compose::text::build::TextBuilder::emoji()
  /// [`verbatim`]: compose::text::build::TextBuilder::verbatim()
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
  /// ## Attributes
  /// |Attribute|Type|Optional|Available as child|
  /// |-|-|-|-|
  /// |[`value`] | [`&str`] or [`String`]  |❌|❌|
  /// |[`text`]  | [`text::Plain`] or [`text::Mrkdwn`] ([`<text>`](super::text())) |❌|✅|
  /// |[`desc`]  | [`&str`], [`String`], or [`text::Plain`] |✅|❌|
  /// |[`url`]   | [`&str`] or [`String`]  |✅|❌|
  ///
  /// [`value`]: compose::opt::build::OptBuilder::value()
  /// [`text`]: compose::opt::build::OptBuilder::text()
  /// [`desc`]: compose::opt::build::OptBuilder::desc()
  /// [`url`]: compose::opt::build::OptBuilder::url()
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
  /// ## Attributes
  /// |Attribute|Type|Optional|Available as child|
  /// |-|-|-|-|
  /// |[`label`] | [`&str`], [`String`], or [`text::Plain`]      |❌|❌|
  /// |[`option`] | [`compose::Opt`]                             |❌ _(or options)_|✅|
  /// |[`options`] | impl [`IntoIterator`] over [`compose::Opt`] |❌ _(or option)_|❌|
  ///
  /// [`label`]: compose::opt_group::build::OptGroupBuilder::label()
  /// [`option`]: compose::opt_group::build::OptGroupBuilder::option()
  /// [`options`]: compose::opt_group::build::OptGroupBuilder::options()
  ///
  /// Supports up to 100 option objects of the same type.
  ///
  /// Note that `syn-rsx` (and `mox` by extension)
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
  /// ## Attributes
  /// |Attribute|Type|Optional|Available as child|
  /// |-|-|-|-|
  /// |[`title`] | [`&str`], [`String`], or [`text::Plain`]   |❌|❌|
  /// |[`text`], [`text_plain`], or [`text_md`]|`text` wants [`text::Plain`] or [`text::Mrkdwn`]. `text_md` & `text_plain` want [`&str`] or [`String`]. |❌|❌|
  /// |[`confirm`] | [`&str`], [`String`], or [`text::Plain`] |❌|❌|
  /// |[`deny`] | [`&str`], [`String`], or [`text::Plain`]    |❌|❌|
  /// |[`style`] | [`compose::confirm::ConfirmStyle`]    |✅|❌|
  ///
  /// [`title`]: compose::confirm::build::ConfirmBuilder::title()
  /// [`text`]: compose::confirm::build::ConfirmBuilder::text()
  /// [`text_plain`]: compose::confirm::build::ConfirmBuilder::text_plain()
  /// [`text_md`]: compose::confirm::build::ConfirmBuilder::text_md()
  /// [`confirm`]: compose::confirm::build::ConfirmBuilder::confirm()
  /// [`deny`]: compose::confirm::build::ConfirmBuilder::deny()
  /// [`style`]: compose::confirm::build::ConfirmBuilder::style()
  ///
  /// ## Example
  /// ```
  /// use slack_blocks::{blox::*, compose::Confirm, text::ToSlackPlaintext};
  ///
  /// let xml = blox! {
  ///   <confirm title="Title"
  ///            text="Body".plaintext()
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
