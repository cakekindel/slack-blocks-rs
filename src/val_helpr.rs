//! TODO: rename mod to validation

use std::borrow::Cow;

use validator::ValidationError;

pub(crate) type ValidationResult = Result<(), validator::ValidationErrors>;
pub(crate) type ValidatorResult = Result<(), validator::ValidationError>;

pub(crate) fn error<StrIsh: AsRef<str>>(kind: &'static str,
                                        msg: StrIsh)
                                        -> ValidationError {
  let mut error = ValidationError::new(kind);
  error.add_param(Cow::from("message"),
                  &msg.as_ref());

  error
}

pub(crate) fn below_len(context: &'static str,
                        max_len: u16,
                        text: impl Long)
                        -> ValidatorResult {
  let len = text.len();

  if len >= max_len.into() {
    Err(error(context,
              format!("{} has a max length of {}, got {}",
                      context, max_len, len)))
  } else {
    Ok(())
  }
}

pub(crate) fn len(context: &'static str,
                  range: impl std::ops::RangeBounds<usize> + std::fmt::Debug,
                  text: impl Long)
                  -> ValidatorResult {
  let len = text.len();

  if !range.contains(&len) {
    Err(error(context,
              format!("{} must be within range {:#?}, got {}",
                      context, range, len)))
  } else {
    Ok(())
  }
}

pub(crate) trait Long {
  fn len(&self) -> usize;
}

impl Long for &crate::text::Text {
  fn len(&self) -> usize {
    self.as_ref().len()
  }
}

impl Long for &str {
  fn len(&self) -> usize {
    str::len(self)
  }
}

impl<'a> Long for &Cow<'a, str> {
  fn len(&self) -> usize {
    str::len(self)
  }
}

impl<T> Long for &[T] {
  fn len(&self) -> usize {
    self.as_ref().len()
  }
}
