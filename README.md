# tauri-plugin-text-recognition

A [Tauri 2](https://v2.tauri.app) plugin that reads printed text from an image **on the device**, with no network: [ML Kit text recognition](https://developers.google.com/ml-kit/vision/text-recognition/v2) on Android.

> **Status: 0.1 is published** on [crates.io](https://crates.io/crates/tauri-plugin-text-recognition) and [npm](https://www.npmjs.com/package/tauri-plugin-text-recognition-api), and used by [Kosmotheka](https://kosmotheka.com) on Android phones. What 0.1 covers and what was tried: [docs/plans/0.1.md](docs/plans/0.1.md).

Give it an image file and it returns the text it finds, in blocks and lines, each with its position in the image. What to do with that text is up to your app: the plugin does not look for anything in particular.

It was started for [Kosmotheka](https://kosmotheka.com), a household catalogue app, to read the ISBN printed on a book when its barcode is covered by a shop label, and to read the spines in a photo of a shelf. It is meant to be useful to any Tauri app.

## Platforms

| Platform | Support |
|---|---|
| Android | ML Kit through Google Play Services, Latin script (English, Spanish, French, German and other languages written in Latin letters). Devices without Play Services are not supported |
| iOS | **Not written and never tried**: see [Help wanted: iOS](#help-wanted-ios) |
| Desktop | Not planned yet: the command fails with "text recognition is not supported on this platform" |

On desktop the command exists, so a desktop and Android app can call it everywhere and decide what to show when it is not supported.

## Install

Add the Rust crate to your app's `src-tauri` and the JavaScript API to the app:

```sh
cd src-tauri && cargo add tauri-plugin-text-recognition
npm install tauri-plugin-text-recognition-api
```

Register the plugin (`src-tauri/src/lib.rs`):

```rust
tauri::Builder::default()
    .plugin(tauri_plugin_text_recognition::init())
```

And allow it in your capabilities (`src-tauri/capabilities/default.json`):

```json
{ "permissions": ["text-recognition:default"] }
```

On Android there is nothing else to set up: no permission to declare, and Tauri adds ML Kit to your app's build. Google Play Services downloads the model when your app is installed.

## Use

```ts
import { recognizeText } from 'tauri-plugin-text-recognition-api'

const result = await recognizeText({ path })

console.log(result.text)   // all the text, blocks separated by a blank line
for (const block of result.blocks) {
  for (const line of block.lines) {
    console.log(line.text, line.bounds, line.confidence)
  }
}
```

`path` is a file path or an Android `content://` URI.

### A complete example: the ISBN on a book's back cover

The user picks a photo with [Tauri's dialog plugin](https://v2.tauri.app/plugin/dialog/), which returns a `content://` URI on Android, and the app looks for an ISBN in the lines it gets back. Looking for the ISBN is the app's job, not the plugin's.

```ts
import { open } from '@tauri-apps/plugin-dialog'
import { recognizeText } from 'tauri-plugin-text-recognition-api'

async function isbnFromPhoto(): Promise<string | null> {
  const path = await open({ filters: [{ name: 'Images', extensions: ['jpg', 'jpeg', 'png', 'webp'] }] })
  if (!path) return null

  const { blocks } = await recognizeText({ path })
  for (const block of blocks) {
    for (const line of block.lines) {
      const match = line.text.replace(/[\s-]/g, '').match(/97[89]\d{10}/)
      if (match) return match[0]   // check the digit before trusting it
    }
  }
  return null
}
```

The [example app](examples/tauri-app) does more: it takes a photo with the camera, draws a box over each block and line, and shows how long the reading took.

### Where the image comes from

- **A file the user picks**: the dialog plugin's `open()` gives a `content://` URI. Pass it as it is.
- **A photo from the camera**: on Android, an `<input type="file" accept="image/*" capture="environment">` opens the system camera with no camera permission in your app (the plugin declares the camera query Android 11 and later need for that). It gives you a `File` in JavaScript, not a path: write it to your app's cache with [the fs plugin](https://v2.tauri.app/plugin/file-system/) and pass that path. The example app does exactly this.
- **A file your app already has**: pass its path.

Large photos are decoded whole. For a back cover, a copy around 2000 px on its longer side is usually enough and faster; for a photo of a whole shelf, where the text is small, try both. The example app has a switch to compare.

### From Rust

The same call is available on the Rust side, through any `AppHandle`, `App` or `Window`:

```rust
use tauri_plugin_text_recognition::{RecognizeTextRequest, TextRecognitionExt};

let result = app
    .text_recognition()
    .recognize_text(RecognizeTextRequest { path })
    .await?;
```

## The result

```ts
interface RecognizedText {
  text: string            // all the text, blocks separated by a blank line
  imageWidth: number      // the image as the engine saw it, after its rotation is applied
  imageHeight: number
  blocks: TextBlock[]
}

interface TextBlock {
  text: string
  bounds: Bounds | null   // when the engine can tell where it is
  lines: TextLine[]
}

interface TextLine {
  text: string
  bounds: Bounds | null       // when the engine can tell where it is
  confidence: number | null   // 0 to 1, when the engine gives it
  angle: number | null        // degrees, when the engine gives it
}

interface Bounds { left: number; top: number; width: number; height: number }   // pixels
```

- Positions are in pixels of the image **upright**: a phone photo stored sideways with a rotation tag is read and measured the way it is shown. `imageWidth` and `imageHeight` are that upright size, so an SVG with `viewBox="0 0 {imageWidth} {imageHeight}"` laid over the `<img>` puts each box in place, whatever size the image is shown at.
- A box can start a little outside the image (a negative `left` or `top`) when the text touches its edge.
- `angle` is how much a line is turned: around 90 or -90 for the spines of books standing on a shelf.

## Errors

The promise is rejected with a message:

| When | Message starts with |
|---|---|
| On desktop | `text recognition is not supported on this platform` |
| The file does not exist, cannot be read or is not an image | `Could not open the image at …` |
| Play Services has not downloaded the model yet (just after the install, or with no connection since) | `The text recognition model is not on this device yet` |
| ML Kit fails for any other reason | `Text recognition failed: …` |

**An image with no text is not an error**: it gives an empty `text` and an empty `blocks`.

## Help wanted: iOS

The plugin has no iOS code: there, the command is meant to fail with "not supported", as on desktop. But the maintainer has no Mac or iPhone, so it has never been built or tried on iOS, not even that an iOS app builds with the plugin in it. **If your app targets iOS, check it yourself first.**

It would be great if someone could cover that part. Apple's [Vision framework](https://developer.apple.com/documentation/vision/recognizing-text-in-images) reads text on the device, like ML Kit does on Android, and the result types are already shared by every platform. If you can build and test on iOS, please open an issue first: see [CONTRIBUTING.md](CONTRIBUTING.md).

## Privacy

The image never leaves the device: ML Kit runs the model locally. The model comes from Google Play Services, which downloads it when your app is installed and, like any Play Services component, may send usage and diagnostics data to Google. See [ML Kit's terms](https://developers.google.com/ml-kit/terms) and [its data disclosure](https://developers.google.com/ml-kit/android-data-disclosure). Your app must meet those terms, and may need to mention it in Google Play's data safety form.

## Documentation

- [How the plugin is built](docs/README.md): the path of a call from JavaScript to ML Kit and back
- [Decisions](docs/decisions.md)
- [Plans](docs/plans/), kept in the open
- [Changelog](CHANGELOG.md)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). To report a security problem, see [SECURITY.md](SECURITY.md).

## Licence

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT licence](LICENSE-MIT), at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, as defined in the Apache-2.0 licence, shall be dual licensed as above, without any additional terms or conditions.

ML Kit is a Google product with its own terms. This plugin does not include it: the Android library declares it as a dependency. "Kosmotheka" and its logos are trademarks of the Kosmotheka project and are not covered by these licences.
