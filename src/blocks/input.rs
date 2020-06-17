use validator::Validate;
use serde::{Deserialize, Serialize};
use crate::compose;
use crate::block_elements::{select};

/// # Input Block
///
/// _[slack api docs 🔗][input_docs]_
///
/// A block that collects information from users -
/// it can hold one of:
///   - [a plain-text input element 🔗][input_element]
///   - [a select menu element 🔗][select_element]
///   - [a multi-select menu element 🔗][multi_select_element]
///   - [a datepicker 🔗][datepicker_element]
///
/// Read [slack's guide to using modals 🔗][modal_guide]
/// to learn how input blocks pass information to your app.
///
/// [input_docs]: https://api.slack.com/reference/block-kit/blocks#input
/// [input_element]: https://api.slack.com/reference/block-kit/block-elements#input
/// [select_element]: https://api.slack.com/reference/block-kit/block-elements#select
/// [multi_select_element]: https://api.slack.com/reference/block-kit/block-elements#multi_select
/// [datepicker_element]: https://api.slack.com/reference/block-kit/block-elements#datepicker
/// [modal_guide]: https://api.slack.com/surfaces/modals/using#gathering_input
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize, Validate)]
pub struct Contents {
    label: compose::Text,
    element: InputElement,
    block_id: Option<String>,
    hint: Option<compose::Text>,
    optional: Option<bool>,
}

impl Contents {
    /// Create an Input Block from a text Label and interactive element.
    ///
    /// ## Examples
    /// ```
    /// use slack_blocks::block_elements::select;
    /// use slack_blocks::blocks;
    /// use slack_blocks::compose;
    ///
    /// # use std::error::Error;
    /// # pub fn main() -> Result<(), Box<dyn Error>> {
    /// let label = compose::Text::plain("On a scale from 1 - 5, how angsty are you?");
    /// let input = select::Static {};
    ///
    /// let block = blocks::input::Contents::from_label_and_element(label, input);
    ///
    /// // < send to slack API >
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_label_and_element<Label: Into<compose::Text>, El: Into<InputElement>>(
        label: Label,
        element: El,
    ) -> Self {
        Contents {
            label: label.into(),
            element: element.into(),
            block_id: None,
            hint: None,
            optional: None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub enum InputElement {
    Checkboxes,
    DatePicker,
    MultiSelect,
    Select(select::Contents),
    PlainInput,
    RadioButtons,
}

impl<T> From<T> for InputElement
where T: Into<select::Contents> {
    fn from(contents: T) -> Self {
        InputElement::Select(contents.into())
    }
}
