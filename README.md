[![crates.io](https://img.shields.io/crates/v/slack-blocks.svg)](https://crates.io/crates/slack-blocks)
[![docs.rs](https://docs.rs/slack-blocks/badge.svg)](https://docs.rs/slack-blocks/latest)
![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)

# slack-blocks

This crate brings Slack's terrific [Block Kit 🔗] to
the Rust ecosystem.

Inside, you'll find models for all of Slack's Layout Blocks,
Block Elements, and Composition Objects.

Every model has builders that leverage Rust's type system
to help you make sure what you're sending to Slack is 100% valid to them.

### Troubleshooting common compiler errors
`Method build not found for ...Builder` - Dig into the error message,
you'll find something like `RequiredMethodNotCalled<method::foo>`,
meaning you need to call `.foo()` before you can call `.build()`!

## Example
Using an example from Slack's Documentation:
```json
{
  "type": "section",
  "text": {
    "text": "*Sally* has requested you set the deadline for the Nano launch project",
    "type": "mrkdwn"
  },
  "accessory": {
    "type": "datepicker",
    "action_id": "datepicker123",
    "initial_date": "1990-04-28",
    "placeholder": {
      "type": "plain_text",
      "text": "Select a date"
    }
  }
}
```

You can use raw Builders like so:
```rust
use slack_blocks::{text::ToSlackMarkdown, blocks::Section, elems::DatePicker};

let section = Section::builder()
                      .text("*Sally* has requested you set the deadline for the Nano launch project".markdown())
                      .accessory(DatePicker::builder()
                                            .action_id("datepicker123")
                                            .initial_date((28, 4, 1990))
                                            .placeholder("Select a date")
                                            .build()
                      )
                      .build();
```

Or enable the `unstable` feature and use xml macros:
```rust
use slack_blocks::mox::*;

let pick_date = blox! {
  <date_picker action_id="datepicker123"
               placeholder="Select a date"
               initial_date=(28, 4, 1990) />
};

let section = blox! {
  <section_block accessory=pick_date>
    <text kind=plain>"*Sally* has requested you set the deadline for the Nano launch project"</text>
  </section_block>
};
```

[Block Kit 🔗]: https://api.slack.com/block-kit
[`cargo-make`]: https://github.com/sagiegurari/cargo-make/
[issues]: https://github.com/cakekindel/slack-blocks-rs/issues/
[Conventional Commits]: https://www.conventionalcommits.org/en/v1.0.0/

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
