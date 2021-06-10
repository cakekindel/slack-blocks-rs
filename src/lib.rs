//! This crate brings Slack's terrific [Block Kit 🔗] to
//! the Rust ecosystem.
//!
//! Inside, you'll find models for all of Slack's Layout Blocks,
//! Block Elements, and Composition Objects. Each structure has Slack's API
//! documentation copied in-place so you don't have to leave your editor to
//! remember the details of the block kit API.
//!
//! Every model has builders that leverage Rust's type system
//! to help you provide every required field, so you can be confident in your app.
//!
//! ## Troubleshooting common compiler errors
//! `Method build not found for ...Builder` - Dig into the error message,
//! you'll find something like `RequiredMethodNotCalled<method::foo>`,
//! meaning you need to call `.foo()` before you can call `.build()`!
//!
//! # Example
//! Using an example from Slack's Documentation:
//! ```json
//! {
//!   "type": "section",
//!   "text": {
//!     "text": "*Sally* has requested you set the deadline for the Nano launch project",
//!     "type": "mrkdwn"
//!   },
//!   "accessory": {
//!     "type": "datepicker",
//!     "action_id": "datepicker123",
//!     "initial_date": "1990-04-28",
//!     "placeholder": {
//!       "type": "plain_text",
//!       "text": "Select a date"
//!     }
//!   }
//! }
//! ```
//!
//! You can use raw Builders like so:
//! ```rust
//! use slack_blocks::{text::ToSlackMarkdown, blocks::Section, elems::DatePicker};
//!
//! let section = Section::builder()
//!                       .text("*Sally* has requested you set the deadline for the Nano launch project".markdown())
//!                       .accessory(DatePicker::builder()
//!                                             .action_id("datepicker123")
//!                                             .initial_date((28, 4, 1990))
//!                                             .placeholder("Select a date")
//!                                             .build()
//!                       )
//!                       .build();
//! ```
//!
//! Or enable the `unstable` feature and use xml macros:
//! ```rust
//! use slack_blocks::blox::*;
//!
//! let pick_date = blox! {
//!   <date_picker action_id="datepicker123"
//!                placeholder="Select a date"
//!                initial_date=(28, 4, 1990) />
//! };
//!
//! let section = blox! {
//!   <section_block accessory=pick_date>
//!     <text kind=plain>"*Sally* has requested you set the deadline for the Nano launch project"</text>
//!   </section_block>
//! };
//! ```
//!
//! Then you can send the block to Slack's API, for example:
//!
//! ```
//! # use slack_blocks::{text::ToSlackMarkdown, blocks::Section, elems::DatePicker};
//! # let section = Section::builder()
//! #                       .text("*Sally* has requested you set the deadline for the Nano launch project".markdown())
//! #                       .accessory(DatePicker::builder()
//! #                                             .action_id("datepicker123")
//! #                                             .initial_date((28, 4, 1990))
//! #                                             .placeholder("Select a date")
//! #                                             .build()
//! #                       )
//! #                       .build();
//! let blocks: Vec<Block> = vec![section.into()]; // using section from examples above
//!
//! let req = reqwest::Client::new()
//!                 .post("https://slack.com/api/chat.postMessage")
//!                 .header("Content-Type", "application/json")
//!                 .bearer_auth("<api token here>")
//!                 .body(serde_json::json!({
//!                   "channel": "<a channel id>",
//!                   "blocks": blocks
//!                 }).to_string())
//!                 .build()
//!                 .unwrap();
//! ```
//!
//! [Block Kit 🔗]: https://api.slack.com/block-kit
//! [`cargo-make`]: https://github.com/sagiegurari/cargo-make/
//! [issues]: https://github.com/cakekindel/slack-blocks-rs/issues/
//! [Conventional Commits]: https://www.conventionalcommits.org/en/v1.0.0/

#![doc(html_root_url = "https://docs.rs/slack-blocks/0.25.0")]
#![cfg_attr(docsrs, feature(doc_cfg))]
// #![feature(doc_cfg)] // for local docs
#![deny(missing_docs)]
#![cfg_attr(not(test),
            forbid(missing_copy_implementations,
                   missing_debug_implementations,
                   unreachable_pub,
                   unsafe_code,
                   unused_crate_dependencies))]

#[macro_use]
#[cfg(feature = "validation")]
extern crate validator_derive;

#[cfg(feature = "blox")]
#[cfg_attr(docsrs, doc(cfg(feature = "blox")))]
pub mod blox;

pub mod blocks;
pub mod compose;
pub mod elems;

mod build;
#[cfg(feature = "validation")]
mod val_helpr;

#[doc(inline)]
pub use blocks::Block;
#[doc(inline)]
pub use compose::text;
#[doc(inline)]
pub use elems::BlockElement;

mod macros {
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
    (impl<'_> From<$source:ident> for $dest:ident => $closure:expr) => {
      impl<'a> From<$source<'a>> for $dest<'a> {
        fn from(src: $source<'a>) -> $dest<'a> {
          $closure(src)
        }
      }
    };
    (impl<$lifetime:lifetime> From<$source:ty> for $dest:ty => $closure:expr) => {
      convert!(impl<$lifetime, > From<$source> for $dest => $closure);
    };
    (impl<$lifetime:lifetime, $($ty_var:ident),*> From<$source:ty> for $dest:ty => $closure:expr) => {
      impl<$lifetime, $($ty_var),*> From<$source> for $dest {
        fn from(src: $source) -> Self {
          $closure(src)
        }
      }
    };
    (impl<$($ty_var:tt),+> From<$source:ty> for $dest:ty => $closure:expr) => {
      impl<$($ty_var),+> From<$source> for $dest {
        fn from(src: $source) -> Self {
          $closure(src)
        }
      }
    };
    (impl From<impl $trait_:ident<$source:ty>> for $dest:ty => $closure:expr) => {
      impl<T> From<T> for $dest where T: $trait_<$source>
      {
        fn from(src: T) -> Self {
          $closure(src)
        }
      }
    };
    (impl<'_> From<impl $trait_:ident<$source:ident>> for $dest:ident => |$param:ident| $body:expr) => {
      impl<'a, T> From<T> for $dest<'a> where T: $trait_<$source<'a>>
      {
        fn from($param: T) -> Self {
          $body
        }
      }
    };
  }
}
