use std::borrow::Cow;

use crate::text;
use crate::compose::Confirm;

use super::{PublicChannel};

/// # Select Element Builder
/// Use to construct a Select element
/// and easily choose a data source
pub struct SelectBuilder<'a> {
    pub placeholder: text::Plain,
    pub action_id: Cow<'a, str>,
    pub confirm: Option<Confirm>
}

impl<'a> SelectBuilder<'a> {
    /// Construct a Select block element from required parts
    ///
    /// # Arguments
    /// - `placeholder`: A [text object 🔗]
    ///
    /// [text objects 🔗]: https://api.slack.com/reference/messaging/composition-objects#text
    ///
    /// # Example
    /// See example for `Select::from_placeholder_and_action_id`
    pub fn from_placeholder_and_action_id(
        placeholder: impl Into<text::Plain>,
        action_id: impl Into<Cow<'a, str>>
    ) -> Self {
        Self {
            placeholder: placeholder.into(),
            action_id: action_id.into(),
            confirm: None,
        }
    }

    /// Construct a Select block element from required parts
    ///
    /// # Arguments
    /// - `confirm`: A [confirm object 🔗] that defines an optional confirmation dialog that appears after a menu item is selected.
    ///
    /// [confirm object 🔗]: https://api.slack.com/reference/block-kit/composition-objects#confirm
    ///
    /// # Example
    /// ```
    /// use slack_blocks::{blocks, block_elements as element, compose::text};
    ///
    /// # use std::error::Error;
    /// # pub fn main() -> Result<(), Box<dyn Error>> {
    /// let fields = vec![
    ///     text::Plain::from("Left column"),
    ///     text::Plain::from("Right column"),
    /// ];
    ///
    /// let select = element::Select::from_placeholder_and_action_id("Pick a channel to delete!", "1234")
    ///                              .confirm(Confirm::from_parts("Are you sure?", test::Mrkdwn::from("Think hard about this."), "Yes", "No"))
    ///                              .choose_from_public_channels();
    ///
    /// let block = blocks::actions::Contents::from_elements(&[select.into()]);
    ///
    /// // < send `block` to slack API >
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_confirm(mut self, confirm: Confirm) -> Self {
        self.confirm = Some(confirm);
        self
    }

    /// Turn the builder into a Public Channel select element
    ///
    /// # Example
    /// ```
    /// use slack_blocks::{blocks, block_elements as element, compose::text};
    ///
    /// # use std::error::Error;
    /// # pub fn main() -> Result<(), Box<dyn Error>> {
    /// let fields = vec![
    ///     text::Plain::from("Left column"),
    ///     text::Plain::from("Right column"),
    /// ];
    ///
    /// let select = element::Select::from_placeholder_and_action_id("Pick a channel to delete!", "1234")
    ///                              .confirm(Confirm::from_parts("Are you sure?", test::Mrkdwn::from("Think hard about this."), "Yes", "No"))
    ///                              .choose_from_public_channels();
    ///
    /// let block = blocks::actions::Contents::from_elements(&[select.into()]);
    ///
    /// // < send `block` to slack API >
    /// # Ok(())
    /// # }
    /// ```
    pub fn choose_from_public_channels(self) -> PublicChannel<'a> {
        let sel = PublicChannel::from_placeholder_and_action_id(
            self.placeholder,
            self.action_id,
        );

        match self.confirm {
            Some(confirm) => sel.with_confirm(confirm),
            None => sel
        }
    }
}
