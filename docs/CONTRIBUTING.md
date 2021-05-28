 # Contributing

 If you're interested in contributing, head over to the [issues] and see what's left to
 do to get this crate fully usable and stable - at the time of writing there are a few
 big-picture things left to do:

 - Implement Block Elements ([#61](https://github.com/cakekindel/slack-blocks-rs/issues/61))
 - ~~Implement Composition Objects ([#63](https://github.com/cakekindel/slack-blocks-rs/issues/63))~~
 - Remove the `validator` crate from the public API ([#9](https://github.com/cakekindel/slack-blocks-rs/issues/9))
 - Add a `validation` crate feature ([#8](https://github.com/cakekindel/slack-blocks-rs/issues/8))

 And this doesn't block a v1.0.0, but is definitely something I'm interested in doing for this crate,
 that will make it a lot nicer to interact with:
 - Add a proc-macro of some kind that allows easy creation of block messages (#??)

 This repo follows [Conventional Commits] in order to fully automate the semver process,
 but you don't _need_ to follow this convention since the repo is configured for Squash
 commits on merge.
 
 ## Build / Test / Format
 This crate uses [`cargo-make`] for script consistency, in Makefile.toml you'll find:
   - `cargo make fmt`: Format all files according to configured style `rustfmt.toml`
   - `cargo make test`: Run all tests
   - `cargo make doctest`: Run doc tests only
   - `cargo make tdd`: Watch files for changes, and run `cargo make test` on each change
   - `cargo make ci`: Run tests, check that code is formatted and no lint violations.
                      This is run as a quality gate for all pull requests.
