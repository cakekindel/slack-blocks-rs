//! TODO: rename mod to validation

use std::borrow::Cow;
use validator::ValidationError;

pub type ValidationResult = Result<(), validator::ValidationErrors>;
pub type ValidatorResult = Result<(), validator::ValidationError>;

pub fn error<StrIsh: AsRef<str>>(kind: &'static str, msg: StrIsh) -> ValidationError {
    let mut error = ValidationError::new(kind);
    error.add_param(Cow::from("message"), &serde_json::Value::String(msg.as_ref().to_string()));

    error
}
