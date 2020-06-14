use serde::{Deserialize, Serialize};

pub mod select;

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub enum BlockElement {
    Button,
    Checkboxes,
    DatePicker,
    Image,
    MultiSelect(select::Contents),
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

