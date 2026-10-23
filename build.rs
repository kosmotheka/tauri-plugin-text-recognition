const COMMANDS: &[&str] = &["recognize_text"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .build();
}
