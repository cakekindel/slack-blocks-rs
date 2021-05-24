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
//!
//! # Build / Test / Format
//! This crate uses [`cargo-make`] for script consistency, in Makefile.toml you'll find:
//!   - `cargo make fmt`: Format all files according to configured style `rustfmt.toml`
//!   - `cargo make test`: Run all tests
//!   - `cargo make doctest`: Run doc tests only
//!   - `cargo make tdd`: Watch files for changes, and run `cargo make test` on each change
//!   - `cargo make ci`: Run tests, check that code is formatted and no lint violations.
//!                      This is run as a quality gate for all pull requests.
//!
//! # Contributing
//!
//! If you're interested in contributing, head over to the [issues] and see what's left to
//! do to get this crate fully usable and stable - at the time of writing there are a few
//! big-picture things left to do:
//!
//! - Implement Block Elements ([#61](https://github.com/cakekindel/slack-blocks-rs/issues/61))
//! - ~~Implement Composition Objects ([#63](https://github.com/cakekindel/slack-blocks-rs/issues/63))~~
//! - Remove the `validator` crate from the public API ([#9](https://github.com/cakekindel/slack-blocks-rs/issues/9))
//! - Add a `validation` crate feature ([#8](https://github.com/cakekindel/slack-blocks-rs/issues/8))
//!
//! And this doesn't block a v1.0.0, but is definitely something I'm interested in doing for this crate,
//! that will make it a lot nicer to interact with:
//! - Add a proc-macro of some kind that allows easy creation of block messages (#??)
//!
//! This repo follows [Conventional Commits] in order to fully automate the semver process,
//! but you don't _need_ to follow this convention since the repo is configured for Squash
//! commits on merge.
//!
//! [`cargo-make`]: https://github.com/sagiegurari/cargo-make/
//! [issues]: https://github.com/cakekindel/slack-blocks-rs/issues/
//! [Conventional Commits]: https://www.conventionalcommits.org/en/v1.0.0/

// #![warn(missing_docs)]
// #![warn(missing_doc_code_examples)]
#![forbid(missing_copy_implementations,
          missing_debug_implementations,
          unreachable_pub,
          unsafe_code,
          unused_crate_dependencies)]
#![allow(deprecated)] // temporary

#[macro_use]
extern crate validator_derive;

pub mod blocks;
pub mod compose;
pub mod elems;

mod build;
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
