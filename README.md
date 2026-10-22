# tauri-plugin-text-recognition

A [Tauri 2](https://v2.tauri.app) plugin that reads printed text from an image **on the device**, with no network: [ML Kit text recognition](https://developers.google.com/ml-kit/vision/text-recognition/v2) on Android.

> **Status: not usable yet.** The repository holds the plugin skeleton and the plan for the first version ([docs/plans/0.1.md](docs/plans/0.1.md)). Nothing is published on crates.io or npm.

It returns the text it finds, in blocks and lines, each with its position in the image. What to do with that text is up to your app: this plugin does not look for anything in particular.

It was started for [Kosmotheka](https://kosmotheka.com), a household catalogue app, to read the ISBN printed on a book when its barcode is covered by a shop label, and to read the spines in a photo of a shelf. It is meant to be useful to any Tauri app.

## Platforms

| Platform | Support |
|---|---|
| Android | Planned for 0.1 (ML Kit, Latin script) |
| iOS | Not planned yet. Contributions welcome |
| Desktop | Not planned yet: the commands return an "unsupported" error |

## Install

Not published yet. Once it is:

```sh
cargo add tauri-plugin-text-recognition
npm install tauri-plugin-text-recognition-api
```

Register it in your app:

```rust
tauri::Builder::default()
    .plugin(tauri_plugin_text_recognition::init())
```

And allow it in your capabilities (`src-tauri/capabilities/default.json`):

```json
{ "permissions": ["text-recognition:default"] }
```

## Use

The API of 0.1 is still being designed: see [the plan](docs/plans/0.1.md#the-api).

## Privacy

The image never leaves the device: ML Kit runs the model locally. If the plugin ends up using the model that Google Play Services downloads (an open question in [the plan](docs/plans/0.1.md#open-questions)), Play Services fetches it the first time and, like any Play Services component, may send usage and diagnostics data to Google. See [ML Kit's terms](https://developers.google.com/ml-kit/terms) and [its data disclosure](https://developers.google.com/ml-kit/android-data-disclosure). Your app must meet those terms.

## Documentation

- [How the plugin is built](docs/README.md)
- [Decisions](docs/decisions.md)
- [Plans](docs/plans/), kept in the open

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). To report a security problem, see [SECURITY.md](SECURITY.md).

## Licence

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT licence](LICENSE-MIT), at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, as defined in the Apache-2.0 licence, shall be dual licensed as above, without any additional terms or conditions.

ML Kit is a Google product with its own terms. This plugin does not include it: the Android library declares it as a dependency. "Kosmotheka" and its logos are trademarks of the Kosmotheka project and are not covered by these licences.
