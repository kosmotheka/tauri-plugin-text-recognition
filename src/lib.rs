use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::TextRecognition;
#[cfg(mobile)]
use mobile::TextRecognition;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the text-recognition APIs.
pub trait TextRecognitionExt<R: Runtime> {
    fn text_recognition(&self) -> &TextRecognition<R>;
}

impl<R: Runtime, T: Manager<R>> crate::TextRecognitionExt<R> for T {
    fn text_recognition(&self) -> &TextRecognition<R> {
        self.state::<TextRecognition<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("text-recognition")
        .invoke_handler(tauri::generate_handler![commands::recognize_text])
        .setup(|app, api| {
            #[cfg(mobile)]
            let text_recognition = mobile::init(app, api)?;
            #[cfg(desktop)]
            let text_recognition = desktop::init(app, api)?;
            app.manage(text_recognition);
            Ok(())
        })
        .build()
}
