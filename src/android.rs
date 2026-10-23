use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<TextRecognition<R>> {
    let handle =
        api.register_android_plugin("app.kosmotheka.textrecognition", "TextRecognitionPlugin")?;
    Ok(TextRecognition(handle))
}

/// The plugin's API on the Rust side, from [`crate::TextRecognitionExt`].
pub struct TextRecognition<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> TextRecognition<R> {
    /// Reads the printed text in an image. An image with no text gives an
    /// empty result, not an error.
    pub async fn recognize_text(
        &self,
        payload: RecognizeTextRequest,
    ) -> crate::Result<RecognizedText> {
        self.0
            .run_mobile_plugin_async("recognizeText", payload)
            .await
            .map_err(Into::into)
    }
}
