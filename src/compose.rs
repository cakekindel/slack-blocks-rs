use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::validation::ValidationResult;

/// # Text Object
/// [_slack api docs 🔗_](https://api.slack.com/reference/block-kit/composition-objects#text)
///
/// An object containing some text,
/// formatted either as `plain_text`
/// or using [`mrkdwn` 🔗](https://api.slack.com/reference/surfaces/formatting),
/// our proprietary textual markup that's just different enough
/// from Markdown to frustrate you.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Text {
    /// ## Markdown text
    /// [_for more info, check out the slack api docs 🔗_](https://api.slack.com/reference/surfaces/formatting)
    ///
    /// ### Reserved Characters
    ///
    /// Slack uses the following special characters,
    /// and recommends you HTML escape them like so:
    ///
    /// <details>
    /// <summary><b>➤ Click to expand</b></summary>
    ///
    /// |character|how to escape|
    /// |---      |---          |
    /// |`&`      |`&amp;`      |
    /// |`<`      |`&lt;`       |
    /// |`>`      |`&gt;`       |
    /// </details>
    ///
    /// ### Basic Formatting
    ///
    /// NOTE: This is **not** an exhaustive list
    ///
    /// This should, however, capture most basic
    /// use cases without requiring that you check with
    /// the Slack documentation.
    ///
    /// For more info, please visit
    /// [Slack's docs for markdown formatting 🔗](https://api.slack.com/reference/surfaces/formatting)
    ///
    /// <details>
    /// <summary><b>➤ Click to expand</b></summary>
    ///
    /// <!-- markdown tables strike again! -->
    /// |slack markdown    |formatted result     |
    /// |---               |---                  |
    /// |`_italic_`        |_italic_             |
    /// |`*bold*`          |**bold**             |
    /// |`~strike~`        |<del>strike</del>    |
    /// |`\n`              |line break           |
    /// |`> a block quote` | <blockquote> a block quote </blockquote> |
    /// |`` `some code!` ``| `some code!`        |
    /// |`` ```multiline code\n2 lines!``` `` | <code>multiline code<br> 2 lines!</code> |
    /// |` - li \n - li `  | <ul><li>li</li><li>li</li></ul> |
    /// |<code>&lt;http://www.foo.com&#124;link name&gt;</code>| [link name](http://www.foo.com) |
    /// |`:joy:` (list from [iamcal/emoji-data 🔗](https://github.com/iamcal/emoji-data)) | 😂 |
    /// | link to #channel: `<#Cxxxxxx>` | [#channel](https://work.slack.com/some-public-channel) |
    /// | link to @user: `<@Uxxxxxx>` | [@user](https://work.slack.com/some-user) |
    /// | link to @user_group: `<!subteam^xxxxxx>` | [@user_group](https://work.slack.com/some-user-group) |
    /// </details>
    #[serde(rename = "mrkdwn")]
    Markdown {
        /// The text for the block.
        ///
        /// This field accepts any of the [standard text formatting markup](#markdown-text)
        text: String,
        /// When set to false (as is default)
        /// URLs will be auto-converted into links,
        /// conversation names will be link-ified,
        /// and certain mentions will be automatically parsed.
        ///
        /// Using a value of true will skip any preprocessing
        /// of this nature, although you can
        /// still include manual parsing strings.
        verbatim: Option<bool>,
    },
    #[serde(rename = "plain_text")]
    Plain {
        /// The text for the block
        text: String,
        /// Indicates whether emojis in a text field
        /// should be escaped into the colon emoji format
        emoji: Option<bool>,
    },
}

impl Text {
    pub fn plain(text: String) -> Text {
        Text::Plain { text, emoji: None }
    }

    pub fn markdown(text: String) -> Text {
        Text::Markdown { text, verbatim: None }
    }

    pub fn text(&self) -> &str {
        use Text::*;

        match self {
            Plain { text, .. } => text,
            Markdown { text, .. } => text,
        }
    }
}

