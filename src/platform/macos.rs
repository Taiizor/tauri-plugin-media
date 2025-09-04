use crate::models::*;
use std::error::Error as StdError;

#[cfg(target_os = "macos")]
use cocoa::base::{id, nil};
#[cfg(target_os = "macos")]
use cocoa::foundation::{NSArray, NSDictionary, NSNumber, NSString};
#[cfg(target_os = "macos")]
use core_graphics::image::CGImage;
#[cfg(target_os = "macos")]
use objc::{class, msg_send, runtime::Object, sel, sel_impl};

pub struct MacOSMediaController {
    event_handler: Option<Box<dyn Fn(MediaControlEvent) + Send>>,
    metadata: Option<MediaMetadata>,
    playback_info: Option<PlaybackInfo>,
}

impl MacOSMediaController {
    pub fn new() -> Self {
        MacOSMediaController {
            event_handler: None,
            metadata: None,
            playback_info: None,
        }
    }

    #[cfg(target_os = "macos")]
    fn get_info_center() -> id {
        unsafe {
            let cls = class!(MPNowPlayingInfoCenter);
            let center: id = msg_send![cls, defaultCenter];
            center
        }
    }

    #[cfg(target_os = "macos")]
    fn get_command_center() -> id {
        unsafe {
            let cls = class!(MPRemoteCommandCenter);
            let center: id = msg_send![cls, sharedCommandCenter];
            center
        }
    }

    #[cfg(target_os = "macos")]
    fn setup_command_handlers(&mut self) -> Result<(), Box<dyn StdError>> {
        unsafe {
            let command_center = Self::get_command_center();

            // Play command
            let play_command: id = msg_send![command_center, playCommand];
            let _: () = msg_send![play_command, setEnabled: true];

            // Pause command
            let pause_command: id = msg_send![command_center, pauseCommand];
            let _: () = msg_send![pause_command, setEnabled: true];

            // Stop command
            let stop_command: id = msg_send![command_center, stopCommand];
            let _: () = msg_send![stop_command, setEnabled: true];

            // Next track command
            let next_command: id = msg_send![command_center, nextTrackCommand];
            let _: () = msg_send![next_command, setEnabled: true];

            // Previous track command
            let previous_command: id = msg_send![command_center, previousTrackCommand];
            let _: () = msg_send![previous_command, setEnabled: true];

            // Toggle play/pause command
            let toggle_command: id = msg_send![command_center, togglePlayPauseCommand];
            let _: () = msg_send![toggle_command, setEnabled: true];

            // Change playback position command
            let position_command: id = msg_send![command_center, changePlaybackPositionCommand];
            let _: () = msg_send![position_command, setEnabled: true];
        }

        Ok(())
    }
}

impl super::MediaController for MacOSMediaController {
    fn initialize_session(
        &mut self,
        _app_id: String,
        _app_name: String,
    ) -> Result<(), Box<dyn StdError>> {
        #[cfg(target_os = "macos")]
        {
            self.setup_command_handlers()?;
        }
        Ok(())
    }

    fn set_metadata(&mut self, metadata: MediaMetadata) -> Result<(), Box<dyn StdError>> {
        #[cfg(target_os = "macos")]
        {
            unsafe {
                let info_center = Self::get_info_center();
                let mut info = Vec::new();

                // Title
                let title_key = NSString::alloc(nil).init_str("MPMediaItemPropertyTitle");
                let title_value = NSString::alloc(nil).init_str(&metadata.title);
                info.push((title_key, title_value));

                // Artist
                if let Some(artist) = &metadata.artist {
                    let artist_key = NSString::alloc(nil).init_str("MPMediaItemPropertyArtist");
                    let artist_value = NSString::alloc(nil).init_str(artist);
                    info.push((artist_key, artist_value));
                }

                // Album
                if let Some(album) = &metadata.album {
                    let album_key = NSString::alloc(nil).init_str("MPMediaItemPropertyAlbumTitle");
                    let album_value = NSString::alloc(nil).init_str(album);
                    info.push((album_key, album_value));
                }

                // Album Artist
                if let Some(album_artist) = &metadata.album_artist {
                    let album_artist_key =
                        NSString::alloc(nil).init_str("MPMediaItemPropertyAlbumArtist");
                    let album_artist_value = NSString::alloc(nil).init_str(album_artist);
                    info.push((album_artist_key, album_artist_value));
                }

                // Duration
                if let Some(duration) = metadata.duration {
                    let duration_key =
                        NSString::alloc(nil).init_str("MPMediaItemPropertyPlaybackDuration");
                    let duration_value = NSNumber::alloc(nil).init_f64(duration);
                    info.push((duration_key, duration_value));
                }

                // Create dictionary
                let keys: Vec<id> = info.iter().map(|(k, _)| *k).collect();
                let values: Vec<id> = info.iter().map(|(_, v)| *v).collect();
                let dict = NSDictionary::dictionaryWithObjects_forKeys_(
                    nil,
                    NSArray::arrayWithObjects(nil, &values),
                    NSArray::arrayWithObjects(nil, &keys),
                );

                let _: () = msg_send![info_center, setNowPlayingInfo: dict];
            }

            self.metadata = Some(metadata);
        }
        #[cfg(not(target_os = "macos"))]
        {
            self.metadata = Some(metadata);
        }
        Ok(())
    }

    fn set_playback_info(&mut self, info: PlaybackInfo) -> Result<(), Box<dyn StdError>> {
        #[cfg(target_os = "macos")]
        {
            unsafe {
                let info_center = Self::get_info_center();
                let current_info: id = msg_send![info_center, nowPlayingInfo];

                if current_info != nil {
                    let mut_dict: id = msg_send![current_info, mutableCopy];

                    // Position
                    let position_key = NSString::alloc(nil)
                        .init_str("MPNowPlayingInfoPropertyElapsedPlaybackTime");
                    let position_value = NSNumber::alloc(nil).init_f64(info.position);
                    let _: () = msg_send![mut_dict, setObject:position_value forKey:position_key];

                    // Playback rate
                    let rate_key =
                        NSString::alloc(nil).init_str("MPNowPlayingInfoPropertyPlaybackRate");
                    let rate_value =
                        NSNumber::alloc(nil).init_f64(if info.status == PlaybackStatus::Playing {
                            info.playback_rate
                        } else {
                            0.0
                        });
                    let _: () = msg_send![mut_dict, setObject:rate_value forKey:rate_key];

                    let _: () = msg_send![info_center, setNowPlayingInfo: mut_dict];
                }
            }

            self.playback_info = Some(info);
        }
        #[cfg(not(target_os = "macos"))]
        {
            self.playback_info = Some(info);
        }
        Ok(())
    }

    fn set_playback_status(&mut self, status: PlaybackStatus) -> Result<(), Box<dyn StdError>> {
        #[cfg(target_os = "macos")]
        {
            unsafe {
                let info_center = Self::get_info_center();
                let current_info: id = msg_send![info_center, nowPlayingInfo];

                if current_info != nil {
                    let mut_dict: id = msg_send![current_info, mutableCopy];

                    let rate_key =
                        NSString::alloc(nil).init_str("MPNowPlayingInfoPropertyPlaybackRate");
                    let rate = if status == PlaybackStatus::Playing {
                        1.0
                    } else {
                        0.0
                    };
                    let rate_value = NSNumber::alloc(nil).init_f64(rate);
                    let _: () = msg_send![mut_dict, setObject:rate_value forKey:rate_key];

                    let _: () = msg_send![info_center, setNowPlayingInfo: mut_dict];
                }
            }

            if let Some(mut info) = self.playback_info.clone() {
                info.status = status;
                self.playback_info = Some(info);
            }
        }
        #[cfg(not(target_os = "macos"))]
        {
            if let Some(mut info) = self.playback_info.clone() {
                info.status = status;
                self.playback_info = Some(info);
            }
        }
        Ok(())
    }

    fn set_position(&mut self, position: f64) -> Result<(), Box<dyn StdError>> {
        #[cfg(target_os = "macos")]
        {
            unsafe {
                let info_center = Self::get_info_center();
                let current_info: id = msg_send![info_center, nowPlayingInfo];

                if current_info != nil {
                    let mut_dict: id = msg_send![current_info, mutableCopy];

                    let position_key = NSString::alloc(nil)
                        .init_str("MPNowPlayingInfoPropertyElapsedPlaybackTime");
                    let position_value = NSNumber::alloc(nil).init_f64(position);
                    let _: () = msg_send![mut_dict, setObject:position_value forKey:position_key];

                    let _: () = msg_send![info_center, setNowPlayingInfo: mut_dict];
                }
            }

            if let Some(mut info) = self.playback_info.clone() {
                info.position = position;
                self.playback_info = Some(info);
            }
        }
        #[cfg(not(target_os = "macos"))]
        {
            if let Some(mut info) = self.playback_info.clone() {
                info.position = position;
                self.playback_info = Some(info);
            }
        }
        Ok(())
    }

    fn clear_metadata(&mut self) -> Result<(), Box<dyn StdError>> {
        #[cfg(target_os = "macos")]
        {
            unsafe {
                let info_center = Self::get_info_center();
                let _: () = msg_send![info_center, setNowPlayingInfo: nil];
            }

            self.metadata = None;
        }
        #[cfg(not(target_os = "macos"))]
        {
            self.metadata = None;
        }
        Ok(())
    }

    fn set_event_handler(&mut self, handler: Box<dyn Fn(MediaControlEvent) + Send>) {
        self.event_handler = Some(handler);
        // Note: Actual event handler setup would require more complex Objective-C runtime manipulation
        // This is a simplified version. In production, you'd need to create proper target-action pairs
    }

    fn get_metadata(&self) -> Result<Option<MediaMetadata>, Box<dyn StdError>> {
        Ok(self.metadata.clone())
    }

    fn get_playback_info(&self) -> Result<Option<PlaybackInfo>, Box<dyn StdError>> {
        Ok(self.playback_info.clone())
    }

    fn get_playback_status(&self) -> Result<PlaybackStatus, Box<dyn StdError>> {
        Ok(self
            .playback_info
            .as_ref()
            .map(|info| info.status)
            .unwrap_or(PlaybackStatus::Stopped))
    }

    fn get_position(&self) -> Result<f64, Box<dyn StdError>> {
        Ok(self
            .playback_info
            .as_ref()
            .map(|info| info.position)
            .unwrap_or(0.0))
    }

    fn is_enabled(&self) -> Result<bool, Box<dyn StdError>> {
        // On macOS, media controls are always available once initialized
        Ok(self.initialized)
    }
}
