use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::val_helpr::ValidationResult;
use crate::text;

#[derive(Validate, Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Contents {
    #[validate(custom = "validate::text")]
    text: text::Text,

    #[validate(length(max = 255))]
    action_id: String,

    #[validate(length(max = 3000))]
    url: Option<String>,

    #[validate(length(max = 2000))]
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

mod validate {
    use crate::val_helpr::{below_len, ValidatorResult};
    use crate::text;

    pub fn text(text: &text::Text) -> ValidatorResult {
        below_len("Button Text", 75, text.as_ref())
    }
}
