# Example app

A small Tauri app to try the plugin on an Android phone: take a photo or pick an image, and it shows the text it read, with a box over each block and line, how long it took and how sure the engine was of each line.

## Run it

From this folder, in the toolchain container (see the root `AGENTS.md`), with the phone connected and USB debugging on:

```sh
(cd ../.. && npm install && npm run build)   # the plugin's TypeScript API, which this app uses
npm install
npm run tauri android init     # the first time only
npm run tauri android dev
```

On desktop the plugin answers "not supported": that is expected.

## What to try

- **Take a photo**, with the system camera, and **Choose an image**, from the files on the phone. They reach the plugin in different ways: the photo as a file in the app's cache, the chosen image as a `content://` URI.
- **Shrink to 2048 px** reads a smaller copy of the image. Compare the time and what it finds against the full image, above all on photos of a whole shelf, where the text is small.
- A book's back cover (is the ISBN line there, and right?), a shelf of spines (text turned 90°), a blurry photo, a photo with no text, and once with the phone in airplane mode.
