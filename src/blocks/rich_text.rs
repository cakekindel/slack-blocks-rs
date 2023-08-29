//! Undocumented rich text blocks

#![allow(missing_docs)]

use std::borrow::Cow;

use crate::text;

#[derive(Clone, Debug, serde::Deserialize, Hash, PartialEq, serde::Serialize)]
#[cfg_attr(feature = "validation", derive(Validate))]
pub struct RichText<'a> {
  #[serde(skip_serializing_if = "Option::is_none")]
  #[cfg_attr(feature = "validation",
             validate(custom = "super::validate_block_id"))]
  pub block_id: Option<Cow<'a, str>>,
  pub elements: Vec<RichTextSection>,
}

#[derive(Clone, Debug, serde::Deserialize, Hash, PartialEq, serde::Serialize)]
pub struct RichTextSection {
  #[serde(rename = "type")]
  pub ty: String,
  pub elements: Vec<text::Text>,
}
