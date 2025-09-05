use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;

mod commands;
mod error;
mod models;
pub mod platform;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Media;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the media APIs.
pub trait MediaExt<R: Runtime> {
    fn media(&self) -> &Media<R>;
}

impl<R: Runtime, T: Manager<R>> crate::MediaExt<R> for T {
    fn media(&self) -> &Media<R> {
        self.state::<Media<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("media")
        .invoke_handler(tauri::generate_handler![
            commands::initialize_session,
            commands::set_metadata,
            commands::set_playback_info,
            commands::set_playback_status,
            commands::set_position,
            commands::clear_metadata,
            commands::get_metadata,
            commands::get_playback_info,
            commands::get_playback_status,
            commands::get_position,
            commands::is_enabled,
        ])
        .setup(|app, api| {
            #[cfg(desktop)]
            let media = desktop::init(app, api)?;
            app.manage(media);
            Ok(())
        })
        .build()
}
