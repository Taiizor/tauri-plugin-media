use crate::models::*;
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

#[cfg(target_os = "linux")]
use dbus::blocking::Connection;
#[cfg(target_os = "linux")]
use dbus_crossroads::{Crossroads, IfaceBuilder, IfaceToken};
#[cfg(target_os = "linux")]
use std::collections::HashMap;

pub struct LinuxMediaController {
    #[cfg(target_os = "linux")]
    connection: Option<Connection>,
    #[cfg(target_os = "linux")]
    crossroads: Option<Arc<Mutex<Crossroads>>>,
    event_handler: Option<Box<dyn Fn(MediaControlEvent) + Send>>,
    metadata: Option<MediaMetadata>,
    playback_info: Option<PlaybackInfo>,
    app_id: String,
    app_name: String,
}

impl LinuxMediaController {
    pub fn new() -> Self {
        LinuxMediaController {
            #[cfg(target_os = "linux")]
            connection: None,
            #[cfg(target_os = "linux")]
            crossroads: None,
            event_handler: None,
            metadata: None,
            playback_info: None,
            app_id: String::new(),
            app_name: String::new(),
        }
    }

    #[cfg(target_os = "linux")]
    fn setup_mpris(&mut self) -> Result<(), Box<dyn StdError>> {
        let conn = Connection::new_session()?;
        let name = format!("org.mpris.MediaPlayer2.{}", self.app_id);
        conn.request_name(&name, false, true, false)?;

        let mut cr = Crossroads::new();

        // MediaPlayer2 interface
        let iface_token = cr.register("org.mpris.MediaPlayer2", |b: &mut IfaceBuilder<()>| {
            b.property("CanQuit").get(|_, _| Ok(false));
            b.property("CanRaise").get(|_, _| Ok(false));
            b.property("HasTrackList").get(|_, _| Ok(false));
            b.property("Identity").get({
                let app_name = self.app_name.clone();
                move |_, _| Ok(app_name.clone())
            });
            b.property("SupportedUriSchemes")
                .get(|_, _| Ok(vec!["file", "http", "https"]));
            b.property("SupportedMimeTypes")
                .get(|_, _| Ok(vec!["audio/mpeg", "audio/mp4", "audio/ogg"]));
        });

        // MediaPlayer2.Player interface
        let player_iface_token = cr.register(
            "org.mpris.MediaPlayer2.Player",
            |b: &mut IfaceBuilder<()>| {
                // Methods
                b.method("Play", (), (), |_, _, _: ()| {
                    // Handle play event
                    Ok(())
                });

                b.method("Pause", (), (), |_, _, _: ()| {
                    // Handle pause event
                    Ok(())
                });

                b.method("PlayPause", (), (), |_, _, _: ()| {
                    // Handle play/pause toggle
                    Ok(())
                });

                b.method("Stop", (), (), |_, _, _: ()| {
                    // Handle stop event
                    Ok(())
                });

                b.method("Next", (), (), |_, _, _: ()| {
                    // Handle next track
                    Ok(())
                });

                b.method("Previous", (), (), |_, _, _: ()| {
                    // Handle previous track
                    Ok(())
                });

                b.method("Seek", ("offset",), (), |_, _, (offset,): (i64,)| {
                    // Handle seek
                    Ok(())
                });

                b.method(
                    "SetPosition",
                    ("track_id", "position"),
                    (),
                    |_, _, (_track_id, _position): (String, i64)| {
                        // Handle position change
                        Ok(())
                    },
                );

                // Properties
                b.property("PlaybackStatus").get({
                    let status = self
                        .playback_info
                        .as_ref()
                        .map(|info| match info.status {
                            PlaybackStatus::Playing => "Playing",
                            PlaybackStatus::Paused => "Paused",
                            PlaybackStatus::Stopped => "Stopped",
                        })
                        .unwrap_or("Stopped");
                    move |_, _| Ok(status.to_string())
                });

                b.property("LoopStatus")
                    .get({
                        let repeat = self
                            .playback_info
                            .as_ref()
                            .map(|info| match info.repeat_mode {
                                RepeatMode::None => "None",
                                RepeatMode::Track => "Track",
                                RepeatMode::List => "Playlist",
                            })
                            .unwrap_or("None");
                        move |_, _| Ok(repeat.to_string())
                    })
                    .set(|_, _, value: String| {
                        // Handle loop status change
                        Ok(Some(value))
                    });

                b.property("Rate")
                    .get({
                        let rate = self
                            .playback_info
                            .as_ref()
                            .map(|info| info.playback_rate)
                            .unwrap_or(1.0);
                        move |_, _| Ok(rate)
                    })
                    .set(|_, _, value: f64| {
                        // Handle rate change
                        Ok(Some(value))
                    });

                b.property("Shuffle")
                    .get({
                        let shuffle = self
                            .playback_info
                            .as_ref()
                            .map(|info| info.shuffle)
                            .unwrap_or(false);
                        move |_, _| Ok(shuffle)
                    })
                    .set(|_, _, value: bool| {
                        // Handle shuffle change
                        Ok(Some(value))
                    });

                b.property("Metadata").get({
                    let metadata = self.create_metadata_dict();
                    move |_, _| Ok(metadata.clone())
                });

                b.property("Volume")
                    .get(|_, _| Ok(1.0_f64))
                    .set(|_, _, value: f64| {
                        // Handle volume change
                        Ok(Some(value))
                    });

                b.property("Position").get({
                    let position = self
                        .playback_info
                        .as_ref()
                        .map(|info| (info.position * 1_000_000.0) as i64)
                        .unwrap_or(0);
                    move |_, _| Ok(position)
                });

                b.property("MinimumRate").get(|_, _| Ok(1.0_f64));

                b.property("MaximumRate").get(|_, _| Ok(1.0_f64));

                b.property("CanGoNext").get(|_, _| Ok(true));

                b.property("CanGoPrevious").get(|_, _| Ok(true));

                b.property("CanPlay").get(|_, _| Ok(true));

                b.property("CanPause").get(|_, _| Ok(true));

                b.property("CanSeek").get(|_, _| Ok(true));

                b.property("CanControl").get(|_, _| Ok(true));
            },
        );

        cr.insert(
            "/org/mpris/MediaPlayer2",
            &[iface_token, player_iface_token],
            (),
        );

        self.connection = Some(conn);
        self.crossroads = Some(Arc::new(Mutex::new(cr)));

        Ok(())
    }

    #[cfg(target_os = "linux")]
    fn create_metadata_dict(
        &self,
    ) -> HashMap<String, dbus::arg::Variant<Box<dyn dbus::arg::RefArg>>> {
        use dbus::arg::Variant;

        let mut metadata = HashMap::new();

        if let Some(meta) = &self.metadata {
            metadata.insert(
                "mpris:trackid".to_string(),
                Variant(Box::new(format!("/org/mpris/MediaPlayer2/Track/{}", 1))
                    as Box<dyn dbus::arg::RefArg>),
            );

            metadata.insert(
                "xesam:title".to_string(),
                Variant(Box::new(meta.title.clone()) as Box<dyn dbus::arg::RefArg>),
            );

            if let Some(artist) = &meta.artist {
                metadata.insert(
                    "xesam:artist".to_string(),
                    Variant(Box::new(vec![artist.clone()]) as Box<dyn dbus::arg::RefArg>),
                );
            }

            if let Some(album) = &meta.album {
                metadata.insert(
                    "xesam:album".to_string(),
                    Variant(Box::new(album.clone()) as Box<dyn dbus::arg::RefArg>),
                );
            }

            if let Some(album_artist) = &meta.album_artist {
                metadata.insert(
                    "xesam:albumArtist".to_string(),
                    Variant(Box::new(vec![album_artist.clone()]) as Box<dyn dbus::arg::RefArg>),
                );
            }

            if let Some(duration) = meta.duration {
                metadata.insert(
                    "mpris:length".to_string(),
                    Variant(Box::new((duration * 1_000_000.0) as i64) as Box<dyn dbus::arg::RefArg>)
                );
            }

            // Handle artwork - MPRIS primarily uses URLs
            if let Some(artwork_url) = &meta.artwork_url {
                metadata.insert(
                    "mpris:artUrl".to_string(),
                    Variant(Box::new(artwork_url.clone()) as Box<dyn dbus::arg::RefArg>),
                );
            } else if let Some(artwork_data) = &meta.artwork_data {
                // For raw image data, we need to save it temporarily and provide a file:// URL
                // This is a simplified approach - in production you might want to use a proper temp file
                use std::fs;
                use std::path::PathBuf;

                let temp_dir = std::env::temp_dir();
                let artwork_path = temp_dir.join(format!("mpris_artwork_{}.jpg", self.app_id));

                // Decode base64 if needed (artwork_data is already Vec<u8>)
                if let Ok(_) = fs::write(&artwork_path, artwork_data) {
                    let file_url = format!("file://{}", artwork_path.display());
                    metadata.insert(
                        "mpris:artUrl".to_string(),
                        Variant(Box::new(file_url) as Box<dyn dbus::arg::RefArg>),
                    );
                }
            }
        }

        metadata
    }
}

impl super::MediaController for LinuxMediaController {
    fn initialize_session(
        &mut self,
        app_id: String,
        app_name: String,
    ) -> Result<(), Box<dyn StdError>> {
        self.app_id = app_id;
        self.app_name = app_name;

        #[cfg(target_os = "linux")]
        {
            self.setup_mpris()?;
        }

        Ok(())
    }

    fn set_metadata(&mut self, metadata: MediaMetadata) -> Result<(), Box<dyn StdError>> {
        self.metadata = Some(metadata);

        #[cfg(target_os = "linux")]
        {
            // Send PropertiesChanged signal
            if let Some(conn) = &self.connection {
                let msg = dbus::Message::signal(
                    &dbus::Path::from("/org/mpris/MediaPlayer2"),
                    &"org.freedesktop.DBus.Properties".into(),
                    &"PropertiesChanged".into(),
                )
                .append1("org.mpris.MediaPlayer2.Player")
                .append2(
                    vec![("Metadata", self.create_metadata_dict())]
                        .into_iter()
                        .collect::<HashMap<_, _>>(),
                    Vec::<String>::new(),
                );

                let _ = conn.send(msg);
            }
        }

        Ok(())
    }

    fn set_playback_info(&mut self, info: PlaybackInfo) -> Result<(), Box<dyn StdError>> {
        self.playback_info = Some(info);

        #[cfg(target_os = "linux")]
        {
            // Send PropertiesChanged signal for playback status
            if let Some(conn) = &self.connection {
                let status = match info.status {
                    PlaybackStatus::Playing => "Playing",
                    PlaybackStatus::Paused => "Paused",
                    PlaybackStatus::Stopped => "Stopped",
                };

                let mut changed = HashMap::new();
                changed.insert("PlaybackStatus", dbus::arg::Variant(status));
                changed.insert(
                    "Position",
                    dbus::arg::Variant((info.position * 1_000_000.0) as i64),
                );
                changed.insert("Rate", dbus::arg::Variant(info.playback_rate));
                changed.insert("Shuffle", dbus::arg::Variant(info.shuffle));

                let loop_status = match info.repeat_mode {
                    RepeatMode::None => "None",
                    RepeatMode::Track => "Track",
                    RepeatMode::List => "Playlist",
                };
                changed.insert("LoopStatus", dbus::arg::Variant(loop_status));

                let msg = dbus::Message::signal(
                    &dbus::Path::from("/org/mpris/MediaPlayer2"),
                    &"org.freedesktop.DBus.Properties".into(),
                    &"PropertiesChanged".into(),
                )
                .append1("org.mpris.MediaPlayer2.Player")
                .append2(changed, Vec::<String>::new());

                let _ = conn.send(msg);
            }
        }

        Ok(())
    }

    fn set_playback_status(&mut self, status: PlaybackStatus) -> Result<(), Box<dyn StdError>> {
        if let Some(mut info) = self.playback_info.clone() {
            info.status = status;
            self.set_playback_info(info)?;
        }
        Ok(())
    }

    fn set_position(&mut self, position: f64) -> Result<(), Box<dyn StdError>> {
        if let Some(mut info) = self.playback_info.clone() {
            info.position = position;
            self.set_playback_info(info)?;
        }
        Ok(())
    }

    fn clear_metadata(&mut self) -> Result<(), Box<dyn StdError>> {
        self.metadata = None;

        #[cfg(target_os = "linux")]
        {
            // Send empty metadata
            if let Some(conn) = &self.connection {
                let msg = dbus::Message::signal(
                    &dbus::Path::from("/org/mpris/MediaPlayer2"),
                    &"org.freedesktop.DBus.Properties".into(),
                    &"PropertiesChanged".into(),
                )
                .append1("org.mpris.MediaPlayer2.Player")
                .append2(
                    vec![("Metadata", HashMap::new())]
                        .into_iter()
                        .collect::<HashMap<_, _>>(),
                    Vec::<String>::new(),
                );

                let _ = conn.send(msg);
            }
        }

        Ok(())
    }

    fn set_event_handler(&mut self, handler: Box<dyn Fn(MediaControlEvent) + Send>) {
        self.event_handler = Some(handler);
        // Note: Actual DBus signal handling would be set up in setup_mpris
    }

    fn get_metadata(&self) -> Result<Option<MediaMetadata>, Box<dyn StdError>> {
        // Linux'ta DBus üzerinden diğer media player'lardan bilgi almak için
        // org.mpris.MediaPlayer2.* servislerini sorgulamamız gerekiyor
        #[cfg(target_os = "linux")]
        {
            if let Some(conn) = &self.connection {
                // List all MPRIS players
                let proxy = conn.with_proxy(
                    "org.freedesktop.DBus",
                    "/",
                    std::time::Duration::from_millis(500),
                );
                use dbus::blocking::stdintf::org_freedesktop_dbus::Peer;

                if let Ok(names) = proxy.list_names() {
                    for name in names {
                        if name.starts_with("org.mpris.MediaPlayer2.")
                            && !name.contains(&self.app_id)
                        {
                            // Found another media player, try to get its metadata
                            let player_proxy = conn.with_proxy(
                                &name,
                                "/org/mpris/MediaPlayer2",
                                std::time::Duration::from_millis(500),
                            );

                            use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
                            if let Ok(metadata_variant) =
                                player_proxy.get("org.mpris.MediaPlayer2.Player", "Metadata")
                            {
                                if let Ok(metadata) = metadata_variant.0.as_iter() {
                                    let mut title = None;
                                    let mut artist = None;
                                    let mut album = None;
                                    let mut artwork_url = None;

                                    for (key, value) in metadata {
                                        if let Some(key_str) = key.as_str() {
                                            match key_str {
                                                "xesam:title" => {
                                                    if let Some(v) = value.as_str() {
                                                        title = Some(v.to_string());
                                                    }
                                                }
                                                "xesam:artist" => {
                                                    if let Some(arr) = value.as_iter() {
                                                        if let Some(first) = arr.next() {
                                                            if let Some(v) = first.1.as_str() {
                                                                artist = Some(v.to_string());
                                                            }
                                                        }
                                                    }
                                                }
                                                "xesam:album" => {
                                                    if let Some(v) = value.as_str() {
                                                        album = Some(v.to_string());
                                                    }
                                                }
                                                "mpris:artUrl" => {
                                                    if let Some(v) = value.as_str() {
                                                        artwork_url = Some(v.to_string());
                                                    }
                                                }
                                                _ => {}
                                            }
                                        }
                                    }

                                    if title.is_some() || artist.is_some() {
                                        return Ok(Some(MediaMetadata {
                                            title: title.unwrap_or_else(|| "Unknown".to_string()),
                                            artist,
                                            album,
                                            album_artist: None,
                                            artwork_url,
                                            artwork_data: None, // MPRIS doesn't provide raw data
                                            duration: None,
                                        }));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Fall back to our own metadata
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
        Ok(self.connection.is_some())
    }
}
