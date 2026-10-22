# How the plugin is built

A Tauri mobile plugin is one feature written three times, once per layer, plus the glue Tauri generates between them. This page follows a call from the app's JavaScript down to ML Kit and back.

## The path of a call

```
App (JavaScript)
  └─ guest-js/index.ts        invoke('plugin:text-recognition|recognize_text', { payload })
       └─ src/commands.rs      #[tauri::command]: receives the arguments, already deserialized
            ├─ src/desktop.rs  on Linux, Windows and macOS (today: "unsupported")
            └─ src/mobile.rs   on Android: run_mobile_plugin_async("recognizeText", payload)
                 └─ android/…/TextRecognitionPlugin.kt   @Command: reads the arguments,
                                                          calls ML Kit, resolves the call
```

The answer travels back the same way: Kotlin builds a JSON object and resolves the `Invoke`, Rust deserializes it into the model in `src/models.rs`, and Tauri serializes that model for JavaScript. **That is why the three sides must agree on every field name.**

## The pieces

- **`build.rs`** lists the commands (`COMMANDS`). From that list Tauri generates an `allow-<command>` and a `deny-<command>` permission for each one.
- **`permissions/default.toml`** is the set an app gets with `text-recognition:default`.
- **`src/lib.rs`** builds the plugin: it registers the commands and, at start-up, creates the desktop or the mobile implementation and stores it in Tauri's state, so any command can reach it with `app.text_recognition()`.
- **`src/mobile.rs`** registers the Kotlin class (`register_android_plugin`) by its package and class name.
- **`android/`** is an Android library. Tauri adds it to the app's Gradle build when the app is built for Android, so its dependencies (ML Kit) become the app's.
- **`guest-js/`** is the TypeScript API, built with Rollup into `dist-js/` and published to npm.
- **`examples/tauri-app/`** is a Tauri app that uses the plugin from the parent folder, to try it on a device.

## Errors

Every failure reaches JavaScript as a rejected promise with a message. `src/error.rs` holds the Rust error type: `Unsupported` on desktop, and `PluginInvoke` for a failure on the Kotlin side (`invoke.reject(message)`).

## Names on each side

Rust names functions in `snake_case` and Kotlin in `camelCase`, so one command has two spellings: JavaScript invokes `recognize_text` (the Rust command, and the name in the permissions), and Rust calls the Kotlin method `recognizeText`. The fields of the models are `camelCase` everywhere: `#[serde(rename_all = "camelCase")]` turns Rust's `image_width` into `imageWidth`.
