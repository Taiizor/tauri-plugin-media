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

pub struct WindowsMediaController {
    #[cfg(target_os = "windows")]
    media_player: Option<MediaPlayer>,
    controls: Option<SystemMediaTransportControls>,
    event_handler: Option<Arc<Mutex<Box<dyn Fn(MediaControlEvent) + Send>>>>,
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
                use windows::Storage::Streams::{InMemoryRandomAccessStream, DataWriter};
                
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

            if let Some(mut info) = self.playback_info.clone() {
                info.status = status;
                self.playback_info = Some(info);
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            if let Some(mut info) = self.playback_info.clone() {
                info.status = status;
                self.playback_info = Some(info);
            }
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

            if let Some(mut info) = self.playback_info.clone() {
                info.position = position;
                self.playback_info = Some(info);
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            if let Some(mut info) = self.playback_info.clone() {
                info.position = position;
                self.playback_info = Some(info);
            }
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
        }
        #[cfg(not(target_os = "windows"))]
        {
            self.metadata = None;
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
            // Try to get system-wide media info first
            if let Ok(Some(session)) = Self::get_global_session() {
                if let Ok(media_properties) = session.TryGetMediaPropertiesAsync()?.get() {
                    let title = media_properties.Title()?.to_string();
                    let artist = media_properties.Artist()?.to_string();
                    let album_artist = media_properties.AlbumArtist()?.to_string();
                    let album_title = media_properties.AlbumTitle()?.to_string();
                    
                    // Try to get artwork
                    let mut artwork_data = None;
                    if let Ok(thumbnail) = media_properties.Thumbnail() {
                        if let Ok(stream) = thumbnail.OpenReadAsync()?.get() {
                            use windows::Storage::Streams::DataReader;
                            
                            let size = stream.Size()?;
                            if size > 0 && size < 10_000_000 { // Limit to 10MB
                                let reader = DataReader::CreateDataReader(&stream)?;
                                reader.LoadAsync(size as u32)?.get()?;
                                
                                let mut buffer = vec![0u8; size as usize];
                                reader.ReadBytes(&mut buffer)?;
                                artwork_data = Some(buffer);
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
        // Fall back to our own metadata
        Ok(self.metadata.clone())
    }

    fn get_playback_info(&self) -> Result<Option<PlaybackInfo>, Box<dyn StdError>> {
        #[cfg(target_os = "windows")]
        {
            // Try to get system-wide playback info
            if let Ok(Some(session)) = Self::get_global_session() {
                if let Ok(playback_info) = session.GetPlaybackInfo() {
                    let status = match playback_info.PlaybackStatus()? {
                        GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing => {
                            PlaybackStatus::Playing
                        }
                        GlobalSystemMediaTransportControlsSessionPlaybackStatus::Paused => {
                            PlaybackStatus::Paused
                        }
                        _ => PlaybackStatus::Stopped,
                    };

                    let playback_rate = playback_info.PlaybackRate()?.Value().unwrap_or(1.0);
                    let is_shuffle = playback_info.IsShuffleActive()?.Value().unwrap_or(false);
                    let repeat_mode = if let Ok(mode) = playback_info.AutoRepeatMode() {
                        match mode.Value().unwrap_or(MediaPlaybackAutoRepeatMode::None) {
                            MediaPlaybackAutoRepeatMode::Track => RepeatMode::Track,
                            MediaPlaybackAutoRepeatMode::List => RepeatMode::List,
                            _ => RepeatMode::None,
                        }
                    } else {
                        RepeatMode::None
                    };

                    return Ok(Some(PlaybackInfo {
                        status,
                        shuffle: is_shuffle,
                        repeat_mode: repeat_mode,
                        playback_rate,
                        position: 0.0, // Will be set by get_position
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
            // Try to get system-wide playback status
            if let Ok(Some(session)) = Self::get_global_session() {
                if let Ok(playback_info) = session.GetPlaybackInfo() {
                    return Ok(match playback_info.PlaybackStatus()? {
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
            // Try to get system-wide position
            if let Ok(Some(session)) = Self::get_global_session() {
                if let Ok(timeline) = session.GetTimelineProperties() {
                    let position_ms = timeline.Position()?.Duration;
                    return Ok(position_ms as f64 / 10_000_000.0); // Convert from 100ns units to seconds
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
