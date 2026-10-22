use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::Result;
use crate::TextRecognitionExt;

#[command]
pub(crate) async fn recognize_text<R: Runtime>(
    app: AppHandle<R>,
    payload: RecognizeTextRequest,
) -> Result<RecognizedText> {
    app.text_recognition().recognize_text(payload).await
}
