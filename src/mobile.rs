use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_text_recognition);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<TextRecognition<R>> {
    #[cfg(target_os = "android")]
    let handle =
        api.register_android_plugin("app.kosmotheka.textrecognition", "TextRecognitionPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_text_recognition)?;
    Ok(TextRecognition(handle))
}

/// Access to the text-recognition APIs.
pub struct TextRecognition<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> TextRecognition<R> {
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
