use crate::models::*;
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

#[cfg(target_os = "windows")]
use windows::{
    Foundation::TypedEventHandler,
    Media::{
        Control::{
            GlobalSystemMediaTransportControlsSession,
            GlobalSystemMediaTransportControlsSessionManager,
            GlobalSystemMediaTransportControlsSessionPlaybackStatus,
        },
        MediaPlaybackAutoRepeatMode, MediaPlaybackStatus, MediaPlaybackType,
        Playback::MediaPlayer,
        SystemMediaTransportControls, SystemMediaTransportControlsButton,
        SystemMediaTransportControlsButtonPressedEventArgs,
        SystemMediaTransportControlsTimelineProperties,
    },
    Storage::Streams::RandomAccessStreamReference,
};

type WindowsEventHandler = Option<Arc<Mutex<Box<dyn Fn(MediaControlEvent) + Send>>>>;

pub struct WindowsMediaController {
    #[cfg(target_os = "windows")]
    media_player: Option<MediaPlayer>,
    controls: Option<SystemMediaTransportControls>,
    event_handler: WindowsEventHandler,
    metadata: Option<MediaMetadata>,
    playback_info: Option<PlaybackInfo>,
}

impl WindowsMediaController {
    pub fn new() -> Self {
        WindowsMediaController {
            #[cfg(target_os = "windows")]
            media_player: None,
            controls: None,
            event_handler: None,
            metadata: None,
            playback_info: None,
        }
    }

    #[cfg(target_os = "windows")]
    fn get_global_session(
    ) -> Result<Option<GlobalSystemMediaTransportControlsSession>, Box<dyn StdError>> {
        // RequestAsync returns an IAsyncOperation, we need to get it synchronously
        let async_op = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?;

        // Wait for the async operation to complete synchronously
        let session_manager = async_op.get()?;

        // Get the current session (active media playing in the system)
        let current_session = session_manager.GetCurrentSession();

        match current_session {
            Ok(session) => Ok(Some(session)),
            Err(_) => Ok(None), // No active session
        }
    }

    #[cfg(target_os = "windows")]
    #[allow(dead_code)]
    fn get_all_sessions(
    ) -> Result<Vec<GlobalSystemMediaTransportControlsSession>, Box<dyn StdError>> {
        let async_op = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?;
        let session_manager = async_op.get()?;

        let sessions_list = session_manager.GetSessions()?;
        let mut sessions = Vec::new();

        for i in 0..sessions_list.Size()? {
            if let Ok(session) = sessions_list.GetAt(i) {
                sessions.push(session);
            }
        }

        Ok(sessions)
    }

    #[cfg(target_os = "windows")]
    fn get_controls(&mut self) -> Result<&SystemMediaTransportControls, Box<dyn StdError>> {
        if self.controls.is_none() {
            // Create a MediaPlayer instance first
            if self.media_player.is_none() {
                let media_player = MediaPlayer::new()?;
                self.media_player = Some(media_player);
            }

            // Get SystemMediaTransportControls from MediaPlayer
            if let Some(player) = &self.media_player {
                let controls = player.SystemMediaTransportControls()?;
                // Set CommandManager to enable controls
                let command_manager = player.CommandManager()?;
                command_manager.SetIsEnabled(true)?;
                self.controls = Some(controls);
            }
        }
        self.controls
            .as_ref()
            .ok_or("Failed to get controls".into())
    }

    #[cfg(target_os = "windows")]
    fn setup_button_handlers(&mut self) -> Result<(), Box<dyn StdError>> {
        // Clone event handler before getting controls to avoid borrow issues
        let handler = self.event_handler.clone();
        let controls = self.get_controls()?;

        if let Some(handler) = handler {
            // Play button
            let play_handler = handler.clone();
            controls.ButtonPressed(&TypedEventHandler::new(
                move |_, args: &Option<SystemMediaTransportControlsButtonPressedEventArgs>| {
                    if let Some(args) = args {
                        let button = args.Button()?;
                        let event = match button {
                            SystemMediaTransportControlsButton::Play => MediaControlEvent {
                                event_type: MediaControlEventType::Play,
                                timestamp: std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs(),
                            },
                            SystemMediaTransportControlsButton::Pause => MediaControlEvent {
                                event_type: MediaControlEventType::Pause,
                                timestamp: std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs(),
                            },
                            SystemMediaTransportControlsButton::Stop => MediaControlEvent {
                                event_type: MediaControlEventType::Stop,
                                timestamp: std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs(),
                            },
                            SystemMediaTransportControlsButton::Next => MediaControlEvent {
                                event_type: MediaControlEventType::Next,
                                timestamp: std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs(),
                            },
                            SystemMediaTransportControlsButton::Previous => MediaControlEvent {
                                event_type: MediaControlEventType::Previous,
                                timestamp: std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs(),
                            },
                            _ => return Ok(()),
                        };

                        if let Ok(handler) = play_handler.lock() {
                            (*handler)(event);
                        }
                    }
                    Ok(())
                },
            ))?;
        }

        Ok(())
    }
}

impl super::MediaController for WindowsMediaController {
    fn initialize_session(
        &mut self,
        _app_id: String,
        _app_name: String,
    ) -> Result<(), Box<dyn StdError>> {
        #[cfg(target_os = "windows")]
        {
            let controls = self.get_controls()?;
            controls.SetIsEnabled(true)?;
            controls.SetIsPlayEnabled(true)?;
            controls.SetIsPauseEnabled(true)?;
            controls.SetIsNextEnabled(true)?;
            controls.SetIsPreviousEnabled(true)?;
            controls.SetIsStopEnabled(true)?;

            self.setup_button_handlers()?;
        }
        Ok(())
    }

    fn set_metadata(&mut self, metadata: MediaMetadata) -> Result<(), Box<dyn StdError>> {
        #[cfg(target_os = "windows")]
        {
            let controls = self.get_controls()?;
            let updater = controls.DisplayUpdater()?;

            updater.SetType(MediaPlaybackType::Music)?;

            let props = updater.MusicProperties()?;
            props.SetTitle(&windows::core::HSTRING::from(&metadata.title))?;

            if let Some(artist) = &metadata.artist {
                props.SetArtist(&windows::core::HSTRING::from(artist))?;
            }

            if let Some(album) = &metadata.album {
                props.SetAlbumTitle(&windows::core::HSTRING::from(album))?;
            }

            // Set artwork from URL or raw data
            if let Some(artwork_url) = &metadata.artwork_url {
                if let Ok(uri) =
                    windows::Foundation::Uri::CreateUri(&windows::core::HSTRING::from(artwork_url))
                {
                    if let Ok(stream_ref) = RandomAccessStreamReference::CreateFromUri(&uri) {
                        updater.SetThumbnail(&stream_ref)?;
                    }
                }
            } else if let Some(artwork_data) = &metadata.artwork_data {
                // Create a stream from raw image data
                use windows::Storage::Streams::{DataWriter, InMemoryRandomAccessStream};

                let stream = InMemoryRandomAccessStream::new()?;
                let writer = DataWriter::CreateDataWriter(&stream)?;
                writer.WriteBytes(artwork_data)?;
                writer.StoreAsync()?.get()?;
                writer.FlushAsync()?.get()?;
                writer.DetachStream()?;

                stream.Seek(0)?;
                let stream_ref = RandomAccessStreamReference::CreateFromStream(&stream)?;
                updater.SetThumbnail(&stream_ref)?;
            }

            updater.Update()?;
            self.metadata = Some(metadata);
        }
        #[cfg(not(target_os = "windows"))]
        {
            self.metadata = Some(metadata);
        }
        Ok(())
    }

    fn set_playback_info(&mut self, info: PlaybackInfo) -> Result<(), Box<dyn StdError>> {
        #[cfg(target_os = "windows")]
        {
            // Clone metadata to avoid borrow issues
            let metadata = self.metadata.clone();
            let controls = self.get_controls()?;

            // Set playback status
            let status = match info.status {
                PlaybackStatus::Playing => MediaPlaybackStatus::Playing,
                PlaybackStatus::Paused => MediaPlaybackStatus::Paused,
                PlaybackStatus::Stopped => MediaPlaybackStatus::Stopped,
            };
            controls.SetPlaybackStatus(status)?;

            // Set repeat mode
            let repeat = match info.repeat_mode {
                RepeatMode::None => MediaPlaybackAutoRepeatMode::None,
                RepeatMode::Track => MediaPlaybackAutoRepeatMode::Track,
                RepeatMode::List => MediaPlaybackAutoRepeatMode::List,
            };
            controls.SetAutoRepeatMode(repeat)?;

            controls.SetShuffleEnabled(info.shuffle)?;
            controls.SetPlaybackRate(info.playback_rate)?;

            // Update timeline if we have duration
            if let Some(metadata) = metadata {
                if let Some(duration) = metadata.duration {
                    let timeline = SystemMediaTransportControlsTimelineProperties::new()?;
                    timeline.SetStartTime(windows::Foundation::TimeSpan { Duration: 0 })?;
                    timeline.SetEndTime(windows::Foundation::TimeSpan {
                        Duration: (duration * 10_000_000.0) as i64,
                    })?;
                    timeline.SetPosition(windows::Foundation::TimeSpan {
                        Duration: (info.position * 10_000_000.0) as i64,
                    })?;
                    controls.UpdateTimelineProperties(&timeline)?;
                }
            }

            self.playback_info = Some(info);
        }
        #[cfg(not(target_os = "windows"))]
        {
            self.playback_info = Some(info);
        }
        Ok(())
    }

    fn set_playback_status(&mut self, status: PlaybackStatus) -> Result<(), Box<dyn StdError>> {
        #[cfg(target_os = "windows")]
        {
            let controls = self.get_controls()?;
            let media_status = match status {
                PlaybackStatus::Playing => MediaPlaybackStatus::Playing,
                PlaybackStatus::Paused => MediaPlaybackStatus::Paused,
                PlaybackStatus::Stopped => MediaPlaybackStatus::Stopped,
            };
            controls.SetPlaybackStatus(media_status)?;

            let mut info = self.playback_info.clone().unwrap_or(PlaybackInfo {
                status,
                position: 0.0,
                shuffle: false,
                repeat_mode: RepeatMode::None,
                playback_rate: 1.0,
            });
            info.status = status;
            self.playback_info = Some(info);
        }
        #[cfg(not(target_os = "windows"))]
        {
            let mut info = self.playback_info.clone().unwrap_or(PlaybackInfo {
                status,
                position: 0.0,
                shuffle: false,
                repeat_mode: RepeatMode::None,
                playback_rate: 1.0,
            });
            info.status = status;
            self.playback_info = Some(info);
        }
        Ok(())
    }

    fn set_position(&mut self, position: f64) -> Result<(), Box<dyn StdError>> {
        #[cfg(target_os = "windows")]
        {
            if let Some(metadata) = &self.metadata {
                if let Some(duration) = metadata.duration {
                    let controls = self.get_controls()?;
                    let timeline = SystemMediaTransportControlsTimelineProperties::new()?;
                    timeline.SetStartTime(windows::Foundation::TimeSpan { Duration: 0 })?;
                    timeline.SetEndTime(windows::Foundation::TimeSpan {
                        Duration: (duration * 10_000_000.0) as i64,
                    })?;
                    timeline.SetPosition(windows::Foundation::TimeSpan {
                        Duration: (position * 10_000_000.0) as i64,
                    })?;
                    controls.UpdateTimelineProperties(&timeline)?;
                }
            }

            let mut info = self.playback_info.clone().unwrap_or(PlaybackInfo {
                status: PlaybackStatus::Stopped,
                position,
                shuffle: false,
                repeat_mode: RepeatMode::None,
                playback_rate: 1.0,
            });
            info.position = position;
            self.playback_info = Some(info);
        }
        #[cfg(not(target_os = "windows"))]
        {
            let mut info = self.playback_info.clone().unwrap_or(PlaybackInfo {
                status: PlaybackStatus::Stopped,
                position,
                shuffle: false,
                repeat_mode: RepeatMode::None,
                playback_rate: 1.0,
            });
            info.position = position;
            self.playback_info = Some(info);
        }
        Ok(())
    }

    fn clear_metadata(&mut self) -> Result<(), Box<dyn StdError>> {
        #[cfg(target_os = "windows")]
        {
            let controls = self.get_controls()?;
            let updater = controls.DisplayUpdater()?;
            updater.ClearAll()?;
            updater.Update()?;
            self.metadata = None;
            self.playback_info = None;
        }
        #[cfg(not(target_os = "windows"))]
        {
            self.metadata = None;
            self.playback_info = None;
        }
        Ok(())
    }

    fn set_event_handler(&mut self, handler: Box<dyn Fn(MediaControlEvent) + Send>) {
        self.event_handler = Some(Arc::new(Mutex::new(handler)));
        #[cfg(target_os = "windows")]
        {
            let _ = self.setup_button_handlers();
        }
    }

    fn get_metadata(&self) -> Result<Option<MediaMetadata>, Box<dyn StdError>> {
        #[cfg(target_os = "windows")]
        {
            // If our own metadata is set, prefer it
            if self.metadata.is_some() {
                return Ok(self.metadata.clone());
            }

            // Fall back to system-wide media info
            if let Ok(Some(session)) = Self::get_global_session() {
                if let Ok(async_op) = session.TryGetMediaPropertiesAsync() {
                    if let Ok(media_properties) = async_op.get() {
                        let title = media_properties
                            .Title()
                            .map(|s| s.to_string())
                            .unwrap_or_default();
                        let artist = media_properties
                            .Artist()
                            .map(|s| s.to_string())
                            .unwrap_or_default();
                        let album_artist = media_properties
                            .AlbumArtist()
                            .map(|s| s.to_string())
                            .unwrap_or_default();
                        let album_title = media_properties
                            .AlbumTitle()
                            .map(|s| s.to_string())
                            .unwrap_or_default();

                        // Try to get artwork
                        let mut artwork_data = None;
                        if let Ok(thumbnail) = media_properties.Thumbnail() {
                            if let Ok(async_op) = thumbnail.OpenReadAsync() {
                                if let Ok(stream) = async_op.get() {
                                    use windows::Storage::Streams::DataReader;

                                    if let Ok(size) = stream.Size() {
                                        if size > 0 && size < 10_000_000 {
                                            // Limit to 10MB
                                            if let Ok(reader) = DataReader::CreateDataReader(&stream) {
                                                if let Ok(load_op) = reader.LoadAsync(size as u32) {
                                                    if load_op.get().is_ok() {
                                                        let mut buffer = vec![0u8; size as usize];
                                                        if reader.ReadBytes(&mut buffer).is_ok() {
                                                            artwork_data = Some(buffer);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        if !title.is_empty() || !artist.is_empty() {
                            return Ok(Some(MediaMetadata {
                                title: if title.is_empty() {
                                    "Unknown".to_string()
                                } else {
                                    title
                                },
                                artist: if artist.is_empty() {
                                    None
                                } else {
                                    Some(artist)
                                },
                                album: if album_title.is_empty() {
                                    None
                                } else {
                                    Some(album_title)
                                },
                                album_artist: if album_artist.is_empty() {
                                    None
                                } else {
                                    Some(album_artist)
                                },
                                artwork_url: None,
                                artwork_data,
                                duration: None,
                            }));
                        }
                    }
                }
            }
        }
        // Fall back to our own metadata
        Ok(self.metadata.clone())
    }

    fn get_playback_info(&self) -> Result<Option<PlaybackInfo>, Box<dyn StdError>> {
        #[cfg(target_os = "windows")]
        {
            // If our own playback info is set, prefer it
            if self.playback_info.is_some() {
                return Ok(self.playback_info.clone());
            }

            // Fall back to system-wide playback info
            if let Ok(Some(session)) = Self::get_global_session() {
                if let Ok(playback_info) = session.GetPlaybackInfo() {
                    let status = playback_info
                        .PlaybackStatus()
                        .map(|s| match s {
                            GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing => {
                                PlaybackStatus::Playing
                            }
                            GlobalSystemMediaTransportControlsSessionPlaybackStatus::Paused => {
                                PlaybackStatus::Paused
                            }
                            _ => PlaybackStatus::Stopped,
                        })
                        .unwrap_or(PlaybackStatus::Stopped);

                    let playback_rate = playback_info
                        .PlaybackRate()
                        .ok()
                        .and_then(|r| r.Value().ok())
                        .unwrap_or(1.0);

                    let is_shuffle = playback_info
                        .IsShuffleActive()
                        .ok()
                        .and_then(|s| s.Value().ok())
                        .unwrap_or(false);

                    let repeat_mode = playback_info
                        .AutoRepeatMode()
                        .ok()
                        .and_then(|mode| mode.Value().ok())
                        .map(|m| match m {
                            MediaPlaybackAutoRepeatMode::Track => RepeatMode::Track,
                            MediaPlaybackAutoRepeatMode::List => RepeatMode::List,
                            _ => RepeatMode::None,
                        })
                        .unwrap_or(RepeatMode::None);

                    let position = if let Ok(timeline) = session.GetTimelineProperties() {
                        timeline
                            .Position()
                            .map(|p| p.Duration as f64 / 10_000_000.0)
                            .unwrap_or(0.0)
                    } else {
                        0.0
                    };

                    return Ok(Some(PlaybackInfo {
                        status,
                        shuffle: is_shuffle,
                        repeat_mode,
                        playback_rate,
                        position,
                    }));
                }
            }
        }
        // Fall back to our own playback info
        Ok(self.playback_info.clone())
    }

    fn get_playback_status(&self) -> Result<PlaybackStatus, Box<dyn StdError>> {
        #[cfg(target_os = "windows")]
        {
            // If our own playback status is set, prefer it
            if let Some(ref info) = self.playback_info {
                return Ok(info.status);
            }

            // Fall back to system-wide playback status
            if let Ok(Some(session)) = Self::get_global_session() {
                if let Ok(playback_info) = session.GetPlaybackInfo() {
                    if let Ok(status) = playback_info.PlaybackStatus() {
                        return Ok(match status {
                            GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing => {
                                PlaybackStatus::Playing
                            }
                            GlobalSystemMediaTransportControlsSessionPlaybackStatus::Paused => {
                                PlaybackStatus::Paused
                            }
                            _ => PlaybackStatus::Stopped,
                        });
                    }
                }
            }
        }
        // Fall back to our own status
        Ok(self
            .playback_info
            .as_ref()
            .map(|info| info.status)
            .unwrap_or(PlaybackStatus::Stopped))
    }

    fn get_position(&self) -> Result<f64, Box<dyn StdError>> {
        #[cfg(target_os = "windows")]
        {
            // If our own position is set, prefer it
            if let Some(ref info) = self.playback_info {
                return Ok(info.position);
            }

            // Fall back to system-wide position
            if let Ok(Some(session)) = Self::get_global_session() {
                if let Ok(timeline) = session.GetTimelineProperties() {
                    if let Ok(position) = timeline.Position() {
                        return Ok(position.Duration as f64 / 10_000_000.0); // Convert from 100ns units to seconds
                    }
                }
            }
        }
        // Fall back to our own position
        Ok(self
            .playback_info
            .as_ref()
            .map(|info| info.position)
            .unwrap_or(0.0))
    }

    fn is_enabled(&self) -> Result<bool, Box<dyn StdError>> {
        #[cfg(target_os = "windows")]
        {
            // Check if there's any active media session in the system
            if let Ok(Some(_)) = Self::get_global_session() {
                return Ok(true);
            }
            // Or if our own controls are enabled
            if let Some(ref controls) = self.controls {
                return Ok(controls.IsEnabled().unwrap_or(false));
            }
        }
        Ok(false)
    }
}

#[cfg(all(test, target_os = "windows"))]
mod tests {
    use super::*;
    use crate::platform::MediaController;

    #[test]
    fn test_windows_media_controller() {
        let mut controller = WindowsMediaController::new();
        let init_res = controller.initialize_session("com.test.media".into(), "Test Media".into());
        println!("initialize_session result: {:?}", init_res);
        assert!(init_res.is_ok());

        let metadata = MediaMetadata {
            title: "Test Windows Track".to_string(),
            artist: Some("Test Artist".to_string()),
            album: Some("Test Album".to_string()),
            album_artist: None,
            duration: Some(210.0),
            artwork_url: None,
            artwork_data: None,
        };

        let meta_res = controller.set_metadata(metadata);
        println!("set_metadata result: {:?}", meta_res);
        assert!(meta_res.is_ok());

        let get_meta = controller.get_metadata().unwrap();
        println!("get_metadata result: {:?}", get_meta);
        assert!(get_meta.is_some());

        let status_res = controller.set_playback_status(PlaybackStatus::Playing);
        println!("set_playback_status result: {:?}", status_res);
        assert!(status_res.is_ok());

        let read_status = controller.get_playback_status().unwrap();
        println!("read_status: {:?}", read_status);
        assert_eq!(read_status, PlaybackStatus::Playing);

        let clear_res = controller.clear_metadata();
        println!("clear_metadata result: {:?}", clear_res);
        assert!(clear_res.is_ok());

        let fresh_controller = WindowsMediaController::new();
        let info_res = fresh_controller.get_playback_info();
        println!("fresh get_playback_info result: {:?}", info_res);
        assert!(info_res.is_ok());
    }
}
