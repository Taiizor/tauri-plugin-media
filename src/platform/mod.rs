use crate::models::*;
use std::error::Error as StdError;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

pub trait MediaController {
    fn initialize_session(
        &mut self,
        app_id: String,
        app_name: String,
    ) -> Result<(), Box<dyn StdError>>;
    fn set_metadata(&mut self, metadata: MediaMetadata) -> Result<(), Box<dyn StdError>>;
    fn set_playback_info(&mut self, info: PlaybackInfo) -> Result<(), Box<dyn StdError>>;
    fn set_playback_status(&mut self, status: PlaybackStatus) -> Result<(), Box<dyn StdError>>;
    fn set_position(&mut self, position: f64) -> Result<(), Box<dyn StdError>>;
    fn clear_metadata(&mut self) -> Result<(), Box<dyn StdError>>;
    fn set_event_handler(&mut self, handler: Box<dyn Fn(MediaControlEvent) + Send>);

    // Get methods to retrieve current state
    fn get_metadata(&self) -> Result<Option<MediaMetadata>, Box<dyn StdError>>;
    fn get_playback_info(&self) -> Result<Option<PlaybackInfo>, Box<dyn StdError>>;
    fn get_playback_status(&self) -> Result<PlaybackStatus, Box<dyn StdError>>;
    fn get_position(&self) -> Result<f64, Box<dyn StdError>>;
    fn is_enabled(&self) -> Result<bool, Box<dyn StdError>>;
}

pub fn create_media_controller() -> Box<dyn MediaController + Send> {
    #[cfg(target_os = "windows")]
    {
        Box::new(windows::WindowsMediaController::new())
    }
    #[cfg(target_os = "macos")]
    {
        Box::new(macos::MacOSMediaController::new())
    }
    #[cfg(target_os = "linux")]
    {
        Box::new(linux::LinuxMediaController::new())
    }
}
