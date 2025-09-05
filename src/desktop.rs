use serde::de::DeserializeOwned;
use std::sync::{Arc, Mutex};
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;
use crate::platform;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Media<R>> {
    let controller = platform::create_media_controller();
    Ok(Media {
        _app_handle: app.clone(),
        controller: Arc::new(Mutex::new(controller)),
    })
}

/// Access to the media APIs.
pub struct Media<R: Runtime> {
    _app_handle: AppHandle<R>,
    controller: Arc<Mutex<Box<dyn platform::MediaController + Send>>>,
}

impl<R: Runtime> Media<R> {
    pub fn initialize_session(&self, request: InitializeMediaSessionRequest) -> crate::Result<()> {
        let mut controller = self.controller.lock().unwrap();
        controller
            .initialize_session(request.app_id, request.app_name)
            .map_err(|e| crate::Error::String(e.to_string()))
    }

    pub fn set_metadata(&self, metadata: MediaMetadata) -> crate::Result<()> {
        let mut controller = self.controller.lock().unwrap();
        controller
            .set_metadata(metadata)
            .map_err(|e| crate::Error::String(e.to_string()))
    }

    pub fn set_playback_info(&self, info: PlaybackInfo) -> crate::Result<()> {
        let mut controller = self.controller.lock().unwrap();
        controller
            .set_playback_info(info)
            .map_err(|e| crate::Error::String(e.to_string()))
    }

    pub fn set_playback_status(&self, status: PlaybackStatus) -> crate::Result<()> {
        let mut controller = self.controller.lock().unwrap();
        controller
            .set_playback_status(status)
            .map_err(|e| crate::Error::String(e.to_string()))
    }

    pub fn set_position(&self, position: f64) -> crate::Result<()> {
        let mut controller = self.controller.lock().unwrap();
        controller
            .set_position(position)
            .map_err(|e| crate::Error::String(e.to_string()))
    }

    pub fn clear_metadata(&self) -> crate::Result<()> {
        let mut controller = self.controller.lock().unwrap();
        controller
            .clear_metadata()
            .map_err(|e| crate::Error::String(e.to_string()))
    }

    pub fn set_event_handler<F>(&self, handler: F)
    where
        F: Fn(MediaControlEvent) + Send + 'static,
    {
        let mut controller = self.controller.lock().unwrap();
        controller.set_event_handler(Box::new(handler));
    }

    pub fn get_metadata(&self) -> crate::Result<Option<MediaMetadata>> {
        let controller = self.controller.lock().unwrap();
        controller
            .get_metadata()
            .map_err(|e| crate::Error::String(e.to_string()))
    }

    pub fn get_playback_info(&self) -> crate::Result<Option<PlaybackInfo>> {
        let controller = self.controller.lock().unwrap();
        controller
            .get_playback_info()
            .map_err(|e| crate::Error::String(e.to_string()))
    }

    pub fn get_playback_status(&self) -> crate::Result<PlaybackStatus> {
        let controller = self.controller.lock().unwrap();
        controller
            .get_playback_status()
            .map_err(|e| crate::Error::String(e.to_string()))
    }

    pub fn get_position(&self) -> crate::Result<f64> {
        let controller = self.controller.lock().unwrap();
        controller
            .get_position()
            .map_err(|e| crate::Error::String(e.to_string()))
    }

    pub fn is_enabled(&self) -> crate::Result<bool> {
        let controller = self.controller.lock().unwrap();
        controller
            .is_enabled()
            .map_err(|e| crate::Error::String(e.to_string()))
    }
}
