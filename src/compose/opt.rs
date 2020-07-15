use serde::{Deserialize, Serialize};
use validator::Validate;

use super::text;
use crate::val_helpr::ValidationResult;

// Used to _statically_
// identify `Opt`s as:
// - being created from mrkdwn
// - being created from plaintext
// - whether or not it has `url` set
pub mod marker {
    use crate::text;

    pub trait FromText<Text: Into<text::Text>> {}
    pub trait WithUrl {}
}

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize, Validate)]
pub struct Opt<Marker = ()> {
    // _       ^^^^^^
    // This allows an `Opt` to _statically_
    // identify itself as:
    // - being created from mrkdwn
    // - being created from plaintext
    // - whether or not it has `url` set

    #[validate(custom = "validate::text")]
    text: text::Text,

    #[validate(length(max = 75))]
    value: String,

    #[validate(custom = "validate::desc")]
    description: Option<text::Text>,

    #[validate(length(max = 3000))]
    url: Option<String>,

    // this prevents an unused generic
    // parameter compile error.
    //
    // This does not actually use any
    // data in memory, just asserts
    // "hey, i promise this Marker thing
    //  is used in some fashion"
    #[serde(skip)]
    __phantom: std::marker::PhantomData<Marker>,
}

// Constructor functions
impl Opt {
    pub fn from_plain_text_and_value(
        text: impl Into<text::Plain>,
        value: impl ToString,
    ) -> Opt<PlainTextOpt> {
        Opt::<PlainTextOpt> {
            text: text.into().into(),
            value: value.to_string(),
            description: None,
            url: None,
            __phantom: std::marker::PhantomData
        }
    }

    pub fn from_mrkdwn_and_value(
        text: impl Into<text::Mrkdwn>,
        value: impl ToString,
    ) -> Opt<MrkdwnOpt> {
        Opt::<MrkdwnOpt> {
            text: text.into().into(),
            value: value.to_string(),
            description: None,
            url: None,
            __phantom: std::marker::PhantomData
        }
    }
}

// Methods available to all specializations
impl<M> Opt<M> {
    pub fn with_description(mut self, desc: impl Into<text::Mrkdwn>) -> Self {
        self.description = Some(desc.into().into());
        self
    }

    pub fn validate(&self) -> ValidationResult {
        Validate::validate(self)
    }
}

// Methods available only to `Opt` created from `text::Plain`
impl<M> Opt<M>
where
    M: marker::FromText<text::Plain>,
{
    pub fn with_url(self, url: impl ToString) -> Opt<PlainTextOptWithUrl> {
        Opt::<PlainTextOptWithUrl> {
            text: self.text,
            value: self.value,
            description: self.description,
            url: Some(url.to_string()),
            __phantom: std::marker::PhantomData
        }
    }
}

pub struct MrkdwnOpt;
impl marker::FromText<text::Mrkdwn> for MrkdwnOpt {}

pub struct PlainTextOpt;
impl marker::FromText<text::Plain> for PlainTextOpt {}

pub struct PlainTextOptWithUrl;
impl marker::WithUrl for PlainTextOptWithUrl {}
impl marker::FromText<text::Plain> for PlainTextOptWithUrl {}

pub mod validate {
    use super::*;
    use crate::val_helpr::{below_len, ValidatorResult};

    pub fn text(text: &text::Text) -> ValidatorResult {
        below_len("Option Text", 75, text.as_ref())
    }

    pub fn desc(text: &text::Text) -> ValidatorResult {
        below_len("Option Description", 75, text.as_ref())
    }
}
