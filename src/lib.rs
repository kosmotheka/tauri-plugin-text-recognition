//! Reads printed text from an image on the device, with no network: ML Kit on
//! Android. On every other platform, iOS included, the command fails with [`Error::Unsupported`].
//!
//! Register the plugin with [`init`] and allow `text-recognition:default` in
//! your capabilities. JavaScript calls it through the
//! `tauri-plugin-text-recognition-api` package; Rust code reaches it through
//! [`TextRecognitionExt`]:
//!
//! ```no_run
//! use tauri_plugin_text_recognition::{RecognizeTextRequest, TextRecognitionExt};
//!
//! async fn back_cover<R: tauri::Runtime>(
//!     app: tauri::AppHandle<R>,
//!     path: String,
//! ) -> tauri_plugin_text_recognition::Result<String> {
//!     let result = app
//!         .text_recognition()
//!         .recognize_text(RecognizeTextRequest { path })
//!         .await?;
//!     Ok(result.text)
//! }
//! ```
//!
//! See the [README](https://github.com/kosmotheka/tauri-plugin-text-recognition#readme)
//! for setup, the result and its errors.

use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(target_os = "android")]
mod android;
#[cfg(not(target_os = "android"))]
mod unsupported;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(target_os = "android")]
pub use android::TextRecognition;
#[cfg(not(target_os = "android"))]
pub use unsupported::TextRecognition;

/// Gives [`tauri::App`], [`tauri::AppHandle`], [`tauri::Window`] and any other
/// [`Manager`] access to the plugin.
pub trait TextRecognitionExt<R: Runtime> {
    fn text_recognition(&self) -> &TextRecognition<R>;
}

impl<R: Runtime, T: Manager<R>> crate::TextRecognitionExt<R> for T {
    fn text_recognition(&self) -> &TextRecognition<R> {
        self.state::<TextRecognition<R>>().inner()
    }
}

/// The plugin, to pass to `tauri::Builder::plugin`.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("text-recognition")
        .invoke_handler(tauri::generate_handler![commands::recognize_text])
        .setup(|app, api| {
            #[cfg(target_os = "android")]
            let text_recognition = android::init(app, api)?;
            #[cfg(not(target_os = "android"))]
            let text_recognition = unsupported::init(app, api)?;
            app.manage(text_recognition);
            Ok(())
        })
        .build()
}
