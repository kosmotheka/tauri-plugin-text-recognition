use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<TextRecognition<R>> {
    Ok(TextRecognition(app.clone()))
}

/// The plugin's API on the Rust side, from [`crate::TextRecognitionExt`].
pub struct TextRecognition<R: Runtime>(AppHandle<R>);

impl<R: Runtime> TextRecognition<R> {
    /// Reads the printed text in an image. An image with no text gives an
    /// empty result, not an error.
    pub async fn recognize_text(
        &self,
        _payload: RecognizeTextRequest,
    ) -> crate::Result<RecognizedText> {
        Err(crate::Error::Unsupported)
    }
}
