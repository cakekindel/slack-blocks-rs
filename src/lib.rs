//! This crate brings Slack's terrific [Block Kit 🔗] to
//! the Rust ecosystem.
//!
//! This crate should hopefully come in handy if you need to
//! build some rich functionality, or just want to send some
//! slack messages without having to know Block Kit.
//!
//! Inside, you'll find simple models with an API that is
//! thoroughly documented and (hopefully) easy to use.
//!
//! This is currently being actively developed so watch the repo for a
//! stable v1 release!
//!
//! [Block Kit 🔗]: https://api.slack.com/block-kit

#[macro_use]
extern crate validator_derive;

pub mod block_elements;
pub mod blocks;
pub mod compose;
mod val_helpr;

pub use compose::text;

#[macro_export]
#[doc(hidden)]
#[deprecated]
macro_rules! impl_from_contents {
    ($enum_name:ident, $variant:ident, $contents_type:ty) => {
        impl From<$contents_type> for $enum_name {
            fn from(contents: $contents_type) -> Self {
                $enum_name::$variant(contents)
            }
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! convert {
    (impl From<$source:ty> for $dest:ty => $closure:expr) => {
        impl From<$source> for $dest {
            fn from(src: $source) -> Self {
                $closure(src)
            }
        }
    };
}
