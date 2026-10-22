use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<TextRecognition<R>> {
    Ok(TextRecognition(app.clone()))
}

/// Access to the text-recognition APIs.
pub struct TextRecognition<R: Runtime>(AppHandle<R>);

impl<R: Runtime> TextRecognition<R> {
    pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
        Ok(PingResponse {
            value: payload.value,
        })
    }
}
