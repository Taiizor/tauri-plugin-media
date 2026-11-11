use serde::de::DeserializeOwned;
use std::sync::{Arc, Mutex};
use tauri::{plugin::PluginApi, AppHandle, Emitter, Runtime};

use crate::models::*;
use crate::platform;

pub type CustomEventHandler = Box<dyn Fn(MediaControlEvent) + Send + Sync + 'static>;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Media<R>> {
    let mut controller = platform::create_media_controller();
    let app_handle = app.clone();
    let custom_handler: Arc<Mutex<Option<CustomEventHandler>>> = Arc::new(Mutex::new(None));

    let handler_clone = custom_handler.clone();
    let app_clone = app_handle.clone();
    controller.set_event_handler(Box::new(move |event: MediaControlEvent| {
        // Emit events to frontend listeners
        let _ = app_clone.emit("media-control-event", &event);
        let _ = app_clone.emit("plugin:media:event", &event);

        // Call user-registered custom Rust handler if present
        if let Ok(guard) = handler_clone.lock() {
            if let Some(ref handler) = *guard {
                handler(event);
            }
        }
    }));

    Ok(Media {
        app_handle,
        controller: Arc::new(Mutex::new(controller)),
        custom_handler,
    })
}

/// Access to the media APIs.
pub struct Media<R: Runtime> {
    app_handle: AppHandle<R>,
    controller: Arc<Mutex<Box<dyn platform::MediaController + Send>>>,
    custom_handler: Arc<Mutex<Option<CustomEventHandler>>>,
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
        F: Fn(MediaControlEvent) + Send + Sync + 'static,
    {
        *self.custom_handler.lock().unwrap() = Some(Box::new(handler));
    }

    pub fn dispatch_event(&self, event: MediaControlEvent) {
        let _ = self.app_handle.emit("media-control-event", &event);
        let _ = self.app_handle.emit("plugin:media:event", &event);
        if let Ok(guard) = self.custom_handler.lock() {
            if let Some(ref handler) = *guard {
                handler(event);
            }
        }
    }

    pub fn next(&self) -> crate::Result<()> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let event = MediaControlEvent {
            event_type: MediaControlEventType::Next,
            timestamp,
        };
        self.dispatch_event(event);
        Ok(())
    }

    pub fn previous(&self) -> crate::Result<()> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let event = MediaControlEvent {
            event_type: MediaControlEventType::Previous,
            timestamp,
        };
        self.dispatch_event(event);
        Ok(())
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
