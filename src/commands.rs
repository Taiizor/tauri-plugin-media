use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::MediaExt;
use crate::Result;

#[command]
pub(crate) async fn initialize_session<R: Runtime>(
    app: AppHandle<R>,
    request: InitializeMediaSessionRequest,
) -> Result<()> {
    app.media().initialize_session(request)
}

#[command]
pub(crate) async fn set_metadata<R: Runtime>(
    app: AppHandle<R>,
    metadata: MediaMetadata,
) -> Result<()> {
    app.media().set_metadata(metadata)
}

#[command]
pub(crate) async fn set_playback_info<R: Runtime>(
    app: AppHandle<R>,
    info: PlaybackInfo,
) -> Result<()> {
    app.media().set_playback_info(info)
}

#[command]
pub(crate) async fn set_playback_status<R: Runtime>(
    app: AppHandle<R>,
    status: PlaybackStatus,
) -> Result<()> {
    app.media().set_playback_status(status)
}

#[command]
pub(crate) async fn set_position<R: Runtime>(app: AppHandle<R>, position: f64) -> Result<()> {
    app.media().set_position(position)
}

#[command]
pub(crate) async fn clear_metadata<R: Runtime>(app: AppHandle<R>) -> Result<()> {
    app.media().clear_metadata()
}

#[command]
pub(crate) async fn get_metadata<R: Runtime>(app: AppHandle<R>) -> Result<Option<MediaMetadata>> {
    app.media().get_metadata()
}

#[command]
pub(crate) async fn get_playback_info<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Option<PlaybackInfo>> {
    app.media().get_playback_info()
}

#[command]
pub(crate) async fn get_playback_status<R: Runtime>(app: AppHandle<R>) -> Result<PlaybackStatus> {
    app.media().get_playback_status()
}

#[command]
pub(crate) async fn get_position<R: Runtime>(app: AppHandle<R>) -> Result<f64> {
    app.media().get_position()
}

#[command]
pub(crate) async fn is_enabled<R: Runtime>(app: AppHandle<R>) -> Result<bool> {
    app.media().is_enabled()
}
