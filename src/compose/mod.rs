use serde::{Deserialize, Serialize};

use crate::quic_from;
use crate::val_helpr::ValidationResult;

pub mod text;
pub use text::Text;

pub mod opt;
pub use opt::Opt;

/// # Composition Objects
///
/// Composition objects can be used inside of [block elements 🔗] and certain message payload fields.
///
/// They are simply common JSON object patterns that you'll encounter frequently
/// when building blocks or composing messages.
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Compose {
    Text(Text),
    Option(Opt),
}

impl Compose {
    pub fn validate(&self) -> ValidationResult {
        match self {
            Self::Text(_) => Ok(()),
            Self::Option(opt) => todo!(),
        }
    }
}

quic_from!(impl From<Text> for Compose => wrap_with(Compose::Text));
quic_from!(impl From<Opt> for Compose =>  wrap_with(Compose::Option));

impl From<text::plain::Contents> for Compose {
    fn from(text: text::plain::Contents) -> Self {
        Into::<text::Text>::into(text).into()
    }
}

impl From<text::mrkdwn::Contents> for Compose {
    fn from(text: text::mrkdwn::Contents) -> Self {
        Into::<text::Text>::into(text).into()
    }
}
