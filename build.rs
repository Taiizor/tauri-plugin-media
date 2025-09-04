const COMMANDS: &[&str] = &[
    "ping",
    "initialize_session",
    "set_metadata",
    "set_playback_info",
    "set_playback_status",
    "set_position",
    "clear_metadata",
    "get_metadata",
    "get_playback_info",
    "get_playback_status",
    "get_position",
    "is_enabled",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
