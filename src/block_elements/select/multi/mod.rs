//! # Multi Select Menu Element
//!
//! A multi-select menu allows a user to select multiple items from a list of options.
//!
//! Just like regular [select menus 🔗], multi-select menus also include type-ahead functionality, where a user can type a part or all of an option string to filter the list.
//!
//! To use interactive components, you will need to make some changes to prepare your app.
//!
//! Read our [guide to enabling interactivity 🔗].
//!
//! [select menus 🔗]: https://api.slack.com/reference/block-kit/block-elements#select
//! [guide to enabling interactivity 🔗]: https://api.slack.com/interactivity/handling

// mod builder;
pub mod conversation;
pub mod external;
// pub mod public_channel;
pub mod static_;
pub mod user;

#[doc(inline)]
pub use conversation::Conversation;
#[doc(inline)]
pub use external::External;
// #[doc(inline)]
// pub use public_channel::PublicChannel;
#[doc(inline)]
pub use static_::Static;
#[doc(inline)]
pub use user::User;
