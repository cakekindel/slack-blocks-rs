use std::iter::Iterator;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("{context} is of length {actual_len} which exceeds max length of {max_len}")]
    ExceedsMaxLen {
        context: String,
        max_len: usize,
        actual_len: usize,
    },
    #[error("{0:?}")]
    Multiple(Vec<ValidationError>)
}

pub type ValidationResult = Result<(), ValidationError>;

pub fn is_str_shorter_than(str: &str, max_len: usize, context: &str) -> ValidationResult {
    is_shorter_than(str.bytes().into_iter(), max_len, context)
}

pub fn is_shorter_than<A>(
    iter: impl Iterator<Item = A>,
    max_len: usize,
    context: &str,
) -> ValidationResult {
    let actual_len = iter.count();

    if actual_len > max_len {
        Err(ValidationError::ExceedsMaxLen {
            context: context.to_string(),
            max_len,
            actual_len,
        })
    } else {
        Ok(())
    }
}
