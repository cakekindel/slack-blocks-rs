use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::val_helpr::ValidationResult;
use crate::text;

#[derive(Validate, Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Contents {
    text: text::Text,
    action_id: String,
    url: Option<String>,
    value: Option<String>,
    style: Option<Style>,
    confirm: Option<()> // FIX: doesn't exist yet
}

impl Contents {
    pub fn from_text_and_action_id(text: impl Into<text::Plain>, action_id: impl ToString) -> Self { todo!() }
    pub fn with_url(mut self, url: impl ToString) -> Self { todo!() }
    pub fn with_value(mut self, value: impl ToString) -> Self { todo!() }
    pub fn with_style(mut self, style: Style) -> Self { todo!() }
    fn with_confirm(confirm: ()) -> Self { todo!() } // FIX: private until usable

    pub fn validate(&self) -> ValidationResult {
        Validate::validate(self)
    }
}

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Style {
    Primary,
    Danger,
}
