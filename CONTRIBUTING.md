# Contributing

Thank you for wanting to help. This is a small project maintained in spare time, so a short message before a large change saves work on both sides.

## Most wanted: iOS

There is no iOS code, and the maintainer cannot build or test on iOS (no Mac, no iPhone). If you can, an iOS side with Apple's Vision framework would be the most useful contribution: see "Help wanted: iOS" in the [README](README.md#help-wanted-ios), and open an issue to talk about it first.

## How

1. **Open an issue first** if the change is more than a small fix: a new platform, a new command, a change to the API.
2. **Fork** the repository and create a branch.
3. Make the change, with the checks below passing.
4. Open a **pull request** that says what changes for someone using the plugin and how you tested it (which device and Android version, if it touches Android).
5. A maintainer reviews it. Expect questions: the API is shared by every app that uses the plugin.

## Rules that matter here

- **No app-specific logic.** The plugin returns text and positions. Finding an ISBN, a date or a price is the app's job.
- **The same types on the three sides** (Rust, Kotlin, TypeScript), with the same names. See [AGENTS.md](AGENTS.md) for the full list of what a new command touches.
- **Documentation and changelog in the same pull request** when the behaviour changes.
- **No personal data in the repository**: no email address other than a GitHub noreply one, no local paths, no keys. Run the secret scan before pushing.
- Everything in English.

## Checks

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
npm install && npm run build
docker run --rm -v "$PWD":/repo zricethezav/gitleaks:v8.28.0 git /repo --redact --no-banner
```

Android changes are tested by hand with the example app in `examples/tauri-app` on a real device: say in the pull request what you tried.

## Releasing (maintainers)

1. Raise the version in `Cargo.toml` and `package.json`, and turn `[Unreleased]` in `CHANGELOG.md` into `[x.y.z] - date`.
2. Push that commit, then a tag `vx.y.z`.

The `Release` workflow does the rest: it checks that the tag, both manifests and the changelog agree, publishes to crates.io and npm with trusted publishing (no tokens: each registry trusts `release.yml` in the `release` environment of this repository), and creates the GitHub release from the changelog. Re-running it skips what is already published.

## Licence of contributions

Contributions are accepted under the project's licence, MIT OR Apache-2.0, as the README says. There is no CLA.

## Using an AI assistant

Fine. `AGENTS.md` holds the conventions in a form assistants read. You are responsible for what you submit: read it and test it as if you had written it by hand.
