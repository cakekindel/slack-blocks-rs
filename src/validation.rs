use thiserror::Error;

use crate::compose::{TextValidationError};

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("validation error! compose::text")]
    Text(TextValidationError),
}

pub trait Validate where Self : Sized {
    fn validate(&self) -> Result<Self, ValidationError>;
}

