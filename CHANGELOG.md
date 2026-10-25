# Changelog

All notable changes to this project are written here, for the people who use the plugin. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versions follow [Semantic Versioning](https://semver.org/).

## [0.1.1] - 2026-10-25

No change to the plugin itself.

### Changed

- Published from GitHub Actions with trusted publishing, so the npm package carries provenance: it says which workflow, in which repository and commit, built it.

## [0.1.0] - 2026-10-25

The first version: read the text in an image on Android.

### Added

- `recognizeText({ path })` in JavaScript, and `recognize_text` from Rust through `app.text_recognition()`. It takes a file path or a `content://` URI and returns the text, in blocks and lines, each with its box in pixels; lines also have a confidence and an angle when the engine gives them.
- On Android it reads with ML Kit through Google Play Services: the plugin adds almost nothing to the app's size, and Play Services downloads the model when the app is installed. Latin script only.
- An image with no text gives an empty result. An image that cannot be opened, a model that is not on the device yet, or a failed recognition is an error whose message says which.
- On desktop the command exists and fails with "text recognition is not supported on this platform". On iOS it is meant to do the same, but it has never been built there.
- The `text-recognition:default` permission.
- The plugin's Android manifest declares the camera query (`android.media.action.IMAGE_CAPTURE`), so an `<input type="file" capture>` in the app opens the camera: since Android 11, without it the app cannot see the camera app and Tauri falls back to the photo picker.
- An example app that takes a photo or opens an image, draws the boxes over it and lists each line.
- Tried on a phone (a moto g84): a whole back cover read in about 2 s at full size, with its ISBN; a shelf of spines in under 1 s, with the titles in plain type read and those in narrow or decorated type often not.
