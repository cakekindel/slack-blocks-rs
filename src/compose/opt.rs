use serde::{Deserialize,Serialize};
use super::text;

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct OptPlain(Opt);

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct OptPlainWithUrl(OptPlain);

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct OptMrkdwn(Opt);

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Opt {
    text: text::Text,
    value: String,
    description: Option<text::Text>,
    url: Option<String>
}

impl Opt {
    pub fn from_plain_text_and_value(
        text: impl Into<text::Plain>,
        value: impl ToString
    ) -> OptPlain { todo!() }

    pub fn from_mrkdwn_and_value(
        text: impl Into<text::Mrkdwn>,
        value: impl ToString,
    ) -> OptMrkdwn { todo!() }

    pub fn with_description(
        mut self,
        desc: impl Into<text::Mrkdwn>,
    ) -> Self { todo!() }

    pub fn with_url(
        mut self,
        url: impl ToString,
    ) -> OptPlainWithUrl { todo!() }
}

impl OptPlain {
    pub fn with_description(self, desc: impl Into<text::Mrkdwn>) -> Self {
        Self(self.0.with_description(desc))
    }

    pub fn with_url(self, url: impl ToString) -> OptPlainWithUrl {
        self.0.with_url(url)
    }
}

impl OptMrkdwn {
    pub fn with_description(self, desc: impl Into<text::Mrkdwn>) -> Self {
        Self(self.0.with_description(desc))
    }
}

impl From<Opt> for OptPlain {
    fn from(opt: Opt) -> Self { Self(opt) }
}

impl From<Opt> for OptMrkdwn {
    fn from(opt: Opt) -> Self { Self(opt) }
}

impl From<OptPlain> for OptPlainWithUrl {
    fn from(opt: OptPlain) -> Self { Self(opt) }
}

impl From<Opt> for OptPlainWithUrl {
    fn from(opt: Opt) -> Self { Self(opt.into()) }
}
