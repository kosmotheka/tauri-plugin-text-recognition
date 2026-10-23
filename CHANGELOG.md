# Changelog

All notable changes to this project are written here, for the people who use the plugin. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versions follow [Semantic Versioning](https://semver.org/).

## [Unreleased]

The first version: read the text in an image on Android.

### Added

- `recognizeText({ path })` in JavaScript, and `recognize_text` from Rust through `app.text_recognition()`. It takes a file path or a `content://` URI and returns the text, in blocks and lines, each with its box in pixels; lines also have a confidence and an angle when the engine gives them.
- On Android it reads with ML Kit through Google Play Services: the plugin adds almost nothing to the app's size, and Play Services downloads the model when the app is installed. Latin script only.
- An image with no text gives an empty result. An image that cannot be opened, a model that is not on the device yet, or a failed recognition is an error whose message says which.
- On desktop the command exists and fails with "text recognition is not supported on this platform". On iOS it is meant to do the same, but it has never been built there.
- The `text-recognition:default` permission.
- An example app that takes a photo or opens an image, draws the boxes over it and lists each line.
