use validator::Validate;
use serde::{Deserialize, Serialize};

use crate::compose::Compose;

#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq, Serialize, Validate)]
pub struct Contents {
    /// A collection of [image elements 🔗] and [text objects 🔗].
    ///
    /// Maximum number of items is 10
    /// [image elements 🔗]: https://api.slack.com/reference/messaging/block-elements#image
    /// [text objects 🔗]: https://api.slack.com/reference/messaging/composition-objects#text
    elements: Vec<Compose>,

    /// A string acting as a unique identifier for a block.
    ///
    /// You can use this `block_id` when you receive an
    /// interaction payload to [identify the source of the action 🔗].
    ///
    /// If not specified, a `block_id` will be generated.
    ///
    /// Maximum length for this field is 255 characters.
    ///
    /// [identify the source of the action 🔗]: https://api.slack.com/interactivity/handling#payloads
    block_id: Option<String>
}
