pub struct Contents {
    /// An array of interactive [element objects 🔗]
    /// - [buttons 🔗]
    /// - [select menus 🔗]
    /// - [overflow menus 🔗]
    /// - [date pickers 🔗]
    ///
    /// There is a maximum of 5 elements in each action block.
    ///
    /// [element objects 🔗]: https://api.slack.com/reference/messaging/block-elements
    /// [buttons 🔗]: https://api.slack.com/reference/messaging/block-elements#button
    /// [select menus 🔗]: https://api.slack.com/reference/messaging/block-elements#select
    /// [overflow menus 🔗]: https://api.slack.com/reference/messaging/block-elements#overflow
    /// [date pickers 🔗]: https://api.slack.com/reference/messaging/block-elements#datepicker
    elements: Vec<()>,

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
    block_id: String,
}
