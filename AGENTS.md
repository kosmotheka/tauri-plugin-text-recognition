# AGENTS.md

Instructions for AI coding agents working in this repository. Human contributors will find the same conventions in [CONTRIBUTING.md](CONTRIBUTING.md).

## Project

A Tauri 2 plugin that reads printed text from an image on the device (ML Kit on Android), with no network. It returns text with positions and nothing else: **no app-specific logic** (looking for an ISBN, a price, a title) goes here. That belongs to the apps that use it.

Read [docs/README.md](docs/README.md) for how the pieces fit, [docs/decisions.md](docs/decisions.md) before proposing a change of direction, and the plan in progress in [docs/plans/](docs/plans/).

## Repository layout

```
src/            Rust side: the commands, the models, desktop.rs and mobile.rs
android/        Kotlin side: the Android library that calls ML Kit
guest-js/       TypeScript API (published to npm as tauri-plugin-text-recognition-api)
permissions/    Tauri permissions for each command
examples/       A small Tauri app to try the plugin on a device
docs/           Documentation; docs/plans/ holds the work plans, in the open
```

## Conventions

- **Everything is in English**: code, comments, docs, plans, commit messages.
- **The repository is public from the first commit.** Never commit an email address other than a GitHub noreply one, a local path (`/home/…`), a key, a token or anything about the maintainer's machines or servers. Before every push, run gitleaks (see CONTRIBUTING.md).
- Clean, readable code. Comments are short and explain *why*; never restate what the code does.
- The same types on the three sides: a field added to a model goes in `src/models.rs`, in the Kotlin result and in `guest-js/index.ts`, with the same `camelCase` name.
- A new command means: the function in `src/commands.rs`, its entry in `build.rs` (`COMMANDS`) and in `generate_handler!`, the desktop and mobile implementations, the Kotlin `@Command`, the TypeScript function, and its permission in `permissions/`.
- The plugin never fails silently. An image that cannot be read is an error with a clear message; an image with no text is an empty result, not an error.
- Unit tests never need a device. Tests that do (instrumented Android tests, the example app) are run by hand and say so.
- Keep dependencies few, and pin GitHub Actions to a full commit SHA.
- **Documentation is part of the work**: a change of behaviour updates the README or `docs/`, a decision goes to `docs/decisions.md`, and a user-visible change gets a line in `CHANGELOG.md`.

## Teaching mode

Learning Rust, Tauri and how a Tauri mobile plugin crosses Rust, Kotlin and JavaScript is a goal of this project. The maintainer is an experienced web developer (PHP, Symfony, TypeScript) and new to Rust and Kotlin. When writing Rust, Kotlin or Tauri code, explain the new concepts in the conversation, relating them to PHP or TypeScript when that helps. **Not in code comments.**

## Commands

On the maintainer's machine, the toolchain runs in the `library-dev` Distrobox container (Rust, Node, the Android SDK and JDK 17):

```bash
distrobox enter library-dev -- bash -ic '<command>'
```

| Task | Command |
|---|---|
| Rust format / lint / test | `cargo fmt --all` · `cargo clippy --all-targets -- -D warnings` · `cargo test` |
| TypeScript API build | `npm install` · `npm run build` |
| Example app on an Android device (the first time, `npm run tauri android init`) | `cd examples/tauri-app && npm install && npm run tauri android dev` |
| Mobile code compiles (CI does it too) | `cargo clippy --target aarch64-linux-android -- -D warnings` |
| Kotlin compiles: build the example app | `cd examples/tauri-app && npm run tauri android build -- --debug --apk --target aarch64` |
| Secret scan before pushing | `docker run --rm -v "$PWD":/repo zricethezav/gitleaks:v8.28.0 git /repo --redact --no-banner` |
