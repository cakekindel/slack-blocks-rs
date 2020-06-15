use serde::{Deserialize, Serialize};

pub mod select;

/// # Block Elements - interactive components
/// [slack api docs 🔗](https://api.slack.com/reference/block-kit/block-elements)
///
/// Block elements can be used inside of `section`, `context`, and `actions` [layout blocks 🔗].
/// Inputs can only be used inside of `input` blocks.
///
/// Our overview of [app surfaces that support Block Kit 🔗] shows you where those blocks might be relevant.
///
/// Finally, our [handling user interactivity guide 🔗] will help you prepare your app to allow
/// for the use of the interactive components listed below.
///
/// [app surfaces that support Block Kit 🔗]: https://api.slack.com/messaging/composing/layouts
/// [handling user interactivity guide 🔗]: https://api.slack.com/interactivity/handling
/// [layout blocks 🔗]: https://api.slack.com/reference/block-kit/blocks
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub enum BlockElement {
    Button,
    Checkboxes,
    DatePicker,
    Image,
    MultiSelect,
    OverflowMenu,
    Select(select::Contents),
    PlainInput,
    RadioButtons,
}

// TODO: move to input block mod when it exists
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub enum InputAttachment {
    Checkboxes,
    DatePicker,
    MultiSelect(select::Contents),
    Select(select::Contents),
    PlainInput,
    RadioButtons,
}
