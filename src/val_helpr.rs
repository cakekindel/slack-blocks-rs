use validator::ValidationError;
use std::borrow::Cow;

pub fn error(kind: &'static str, msg: String) -> ValidationError {
    let mut error = ValidationError::new(kind);
    error.add_param(Cow::from("message"), &serde_json::Value::String(msg));

    error
}

