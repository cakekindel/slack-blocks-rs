use serde::{Deserialize, Serialize};

use crate::impl_from_contents;

pub mod text;
use text::Text;

pub mod opt;
use opt::Opt;

/// # Composition Objects
///
/// Composition objects can be used inside of [block elements 🔗] and certain message payload fields.
///
/// They are simply common JSON object patterns that you'll encounter frequently
/// when building blocks or composing messages.
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub enum Compose {
    Text(Text),
    Option(Opt),
}

impl_from_contents!(Compose, Text, Text);
impl_from_contents!(Compose, Option, Opt);

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
