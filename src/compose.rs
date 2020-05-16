use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Text {
    pub text: String
}

#[derive(Debug, Error)]
pub enum TextValidationError {}

