use serde::{ser::Serializer, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

/// Why the text could not be read. JavaScript receives its message.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Any platform but Android.
    #[error("text recognition is not supported on this platform")]
    Unsupported,
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// The Android side failed: the image could not be opened, the model is
    /// not on the device yet, or recognition failed. The message says which.
    #[cfg(mobile)]
    #[error(transparent)]
    PluginInvoke(#[from] tauri::plugin::mobile::PluginInvokeError),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
