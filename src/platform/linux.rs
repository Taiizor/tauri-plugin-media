use crate::models::*;
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

#[cfg(target_os = "linux")]
use dbus::blocking::Connection;
#[cfg(target_os = "linux")]
use dbus::channel::Sender;
#[cfg(target_os = "linux")]
use dbus_crossroads::{Crossroads, IfaceBuilder};
#[cfg(target_os = "linux")]
use std::collections::HashMap;

pub struct LinuxMediaController {
    #[cfg(target_os = "linux")]
    connection: Option<Connection>,
    #[cfg(target_os = "linux")]
    crossroads: Option<Arc<Mutex<Crossroads>>>,
    event_handler: Arc<Mutex<Option<Box<dyn Fn(MediaControlEvent) + Send>>>>,
    metadata: Arc<Mutex<Option<MediaMetadata>>>,
    playback_info: Arc<Mutex<Option<PlaybackInfo>>>,
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
            event_handler: Arc::new(Mutex::new(None)),
            metadata: Arc::new(Mutex::new(None)),
            playback_info: Arc::new(Mutex::new(None)),
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
            b.property("SupportedUriSchemes").get(|_, _| {
                Ok(vec![
                    "file".to_string(),
                    "http".to_string(),
                    "https".to_string(),
                ])
            });
            b.property("SupportedMimeTypes").get(|_, _| {
                Ok(vec![
                    "audio/mpeg".to_string(),
                    "audio/mp4".to_string(),
                    "audio/ogg".to_string(),
                ])
            });
        });

        // MediaPlayer2.Player interface
        let handler_arc = Arc::clone(&self.event_handler);
        let metadata_arc = Arc::clone(&self.metadata);
        let playback_arc = Arc::clone(&self.playback_info);
        let app_id_clone = self.app_id.clone();

        let player_iface_token = cr.register(
            "org.mpris.MediaPlayer2.Player",
            move |b: &mut IfaceBuilder<()>| {
                // Methods
                let h_play = Arc::clone(&handler_arc);
                b.method("Play", (), (), move |_, _, _: ()| {
                    if let Some(handler) = h_play.lock().unwrap().as_ref() {
                        handler(MediaControlEvent {
                            event_type: MediaControlEventType::Play,
                            timestamp: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_secs(),
                        });
                    }
                    Ok(())
                });

                let h_pause = Arc::clone(&handler_arc);
                b.method("Pause", (), (), move |_, _, _: ()| {
                    if let Some(handler) = h_pause.lock().unwrap().as_ref() {
                        handler(MediaControlEvent {
                            event_type: MediaControlEventType::Pause,
                            timestamp: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_secs(),
                        });
                    }
                    Ok(())
                });

                let h_toggle = Arc::clone(&handler_arc);
                b.method("PlayPause", (), (), move |_, _, _: ()| {
                    if let Some(handler) = h_toggle.lock().unwrap().as_ref() {
                        handler(MediaControlEvent {
                            event_type: MediaControlEventType::PlayPause,
                            timestamp: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_secs(),
                        });
                    }
                    Ok(())
                });

                let h_stop = Arc::clone(&handler_arc);
                b.method("Stop", (), (), move |_, _, _: ()| {
                    if let Some(handler) = h_stop.lock().unwrap().as_ref() {
                        handler(MediaControlEvent {
                            event_type: MediaControlEventType::Stop,
                            timestamp: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_secs(),
                        });
                    }
                    Ok(())
                });

                let h_next = Arc::clone(&handler_arc);
                b.method("Next", (), (), move |_, _, _: ()| {
                    if let Some(handler) = h_next.lock().unwrap().as_ref() {
                        handler(MediaControlEvent {
                            event_type: MediaControlEventType::Next,
                            timestamp: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_secs(),
                        });
                    }
                    Ok(())
                });

                let h_prev = Arc::clone(&handler_arc);
                b.method("Previous", (), (), move |_, _, _: ()| {
                    if let Some(handler) = h_prev.lock().unwrap().as_ref() {
                        handler(MediaControlEvent {
                            event_type: MediaControlEventType::Previous,
                            timestamp: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_secs(),
                        });
                    }
                    Ok(())
                });

                let h_seek = Arc::clone(&handler_arc);
                b.method("Seek", ("offset",), (), move |_, _, (offset,): (i64,)| {
                    if let Some(handler) = h_seek.lock().unwrap().as_ref() {
                        handler(MediaControlEvent {
                            event_type: MediaControlEventType::SeekTo(offset as f64 / 1_000_000.0),
                            timestamp: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_secs(),
                        });
                    }
                    Ok(())
                });

                let h_setpos = Arc::clone(&handler_arc);
                b.method(
                    "SetPosition",
                    ("track_id", "position"),
                    (),
                    move |_, _, (_track_id, position): (dbus::Path, i64)| {
                        if let Some(handler) = h_setpos.lock().unwrap().as_ref() {
                            handler(MediaControlEvent {
                                event_type: MediaControlEventType::SetPosition(
                                    position as f64 / 1_000_000.0,
                                ),
                                timestamp: std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs(),
                            });
                        }
                        Ok(())
                    },
                );

                // Properties
                let pb_status = Arc::clone(&playback_arc);
                b.property("PlaybackStatus").get(move |_, _| {
                    let guard = pb_status.lock().unwrap();
                    let status = guard
                        .as_ref()
                        .map(|info| match info.status {
                            PlaybackStatus::Playing => "Playing",
                            PlaybackStatus::Paused => "Paused",
                            PlaybackStatus::Stopped => "Stopped",
                        })
                        .unwrap_or("Stopped");
                    Ok(status.to_string())
                });

                let pb_loop = Arc::clone(&playback_arc);
                b.property("LoopStatus")
                    .get(move |_, _| {
                        let guard = pb_loop.lock().unwrap();
                        let repeat = guard
                            .as_ref()
                            .map(|info| match info.repeat_mode {
                                RepeatMode::None => "None",
                                RepeatMode::Track => "Track",
                                RepeatMode::List => "Playlist",
                            })
                            .unwrap_or("None");
                        Ok(repeat.to_string())
                    })
                    .set(|_, _, value: String| Ok(Some(value)));

                let pb_rate = Arc::clone(&playback_arc);
                b.property("Rate")
                    .get(move |_, _| {
                        let guard = pb_rate.lock().unwrap();
                        let rate = guard.as_ref().map(|info| info.playback_rate).unwrap_or(1.0);
                        Ok(rate)
                    })
                    .set(|_, _, value: f64| Ok(Some(value)));

                let pb_shuffle = Arc::clone(&playback_arc);
                b.property("Shuffle")
                    .get(move |_, _| {
                        let guard = pb_shuffle.lock().unwrap();
                        let shuffle = guard.as_ref().map(|info| info.shuffle).unwrap_or(false);
                        Ok(shuffle)
                    })
                    .set(|_, _, value: bool| Ok(Some(value)));

                let meta_dict = Arc::clone(&metadata_arc);
                let app_id_meta = app_id_clone.clone();
                b.property("Metadata").get(move |_, _| {
                    let guard = meta_dict.lock().unwrap();
                    Ok(Self::create_metadata_dict_from(
                        &app_id_meta,
                        guard.as_ref(),
                    ))
                });

                b.property("Volume")
                    .get(|_, _| Ok(1.0_f64))
                    .set(|_, _, value: f64| Ok(Some(value)));

                let pb_pos = Arc::clone(&playback_arc);
                b.property("Position").get(move |_, _| {
                    let guard = pb_pos.lock().unwrap();
                    let position = guard
                        .as_ref()
                        .map(|info| (info.position * 1_000_000.0) as i64)
                        .unwrap_or(0);
                    Ok(position)
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
    fn create_metadata_dict_from(
        app_id: &str,
        metadata_opt: Option<&MediaMetadata>,
    ) -> HashMap<String, dbus::arg::Variant<Box<dyn dbus::arg::RefArg>>> {
        use dbus::arg::Variant;
        let mut metadata = HashMap::new();

        if let Some(meta) = metadata_opt {
            metadata.insert(
                "mpris:trackid".to_string(),
                Variant(Box::new(dbus::Path::from(format!(
                    "/org/mpris/MediaPlayer2/Track/{}",
                    app_id
                ))) as Box<dyn dbus::arg::RefArg>),
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
                    Variant(Box::new((duration * 1_000_000.0) as i64) as Box<dyn dbus::arg::RefArg>),
                );
            }

            if let Some(artwork_url) = &meta.artwork_url {
                metadata.insert(
                    "mpris:artUrl".to_string(),
                    Variant(Box::new(artwork_url.clone()) as Box<dyn dbus::arg::RefArg>),
                );
            } else if let Some(artwork_data) = &meta.artwork_data {
                use std::fs;
                let temp_dir = std::env::temp_dir();
                let artwork_path = temp_dir.join(format!("mpris_artwork_{}.jpg", app_id));
                if fs::write(&artwork_path, artwork_data).is_ok() {
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

    #[cfg(target_os = "linux")]
    fn get_other_player_names(&self, conn: &Connection) -> Vec<String> {
        let proxy = conn.with_proxy(
            "org.freedesktop.DBus",
            "/",
            std::time::Duration::from_millis(500),
        );

        let result: Result<(Vec<String>,), _> =
            proxy.method_call("org.freedesktop.DBus", "ListNames", ());

        match result {
            Ok((names,)) => names
                .into_iter()
                .filter(|name| {
                    name.starts_with("org.mpris.MediaPlayer2.") && !name.contains(&self.app_id)
                })
                .collect(),
            Err(_) => Vec::new(),
        }
    }

    #[cfg(target_os = "linux")]
    fn fetch_external_metadata(conn: &Connection, service_name: &str) -> Option<MediaMetadata> {
        use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;

        let proxy = conn.with_proxy(
            service_name,
            "/org/mpris/MediaPlayer2",
            std::time::Duration::from_millis(500),
        );

        let metadata_variant: dbus::arg::Variant<Box<dyn dbus::arg::RefArg>> = proxy
            .get("org.mpris.MediaPlayer2.Player", "Metadata")
            .ok()?;

        let entries = metadata_variant.0.as_iter()?;

        let mut title = String::from("Unknown");
        let mut artist = None;
        let mut album = None;
        let mut artwork_url = None;

        for entry in entries {
            if let Some(mut dict_iter) = entry.as_iter() {
                let key_ref = dict_iter.next()?;
                let value_ref = dict_iter.next()?;

                if let Some(key) = key_ref.as_str() {
                    match key {
                        "xesam:title" => {
                            if let Some(s) = value_ref.as_str() {
                                title = s.to_string();
                            }
                        }
                        "xesam:artist" => {
                            if let Some(iter) = value_ref.as_iter() {
                                for item in iter {
                                    if let Some(s) = item.as_str() {
                                        artist = Some(s.to_string());
                                        break;
                                    }
                                }
                            }
                        }
                        "xesam:album" => {
                            if let Some(s) = value_ref.as_str() {
                                album = Some(s.to_string());
                            }
                        }
                        "mpris:artUrl" => {
                            if let Some(s) = value_ref.as_str() {
                                artwork_url = Some(s.to_string());
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        Some(MediaMetadata {
            title,
            artist,
            album,
            album_artist: None,
            artwork_url,
            artwork_data: None,
            duration: None,
        })
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
        *self.metadata.lock().unwrap() = Some(metadata);

        #[cfg(target_os = "linux")]
        {
            // Send PropertiesChanged signal
            if let Some(conn) = &self.connection {
                let guard = self.metadata.lock().unwrap();
                let dict = Self::create_metadata_dict_from(&self.app_id, guard.as_ref());
                drop(guard);

                let mut changed = HashMap::new();
                changed.insert("Metadata", dbus::arg::Variant(dict));

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

    fn set_playback_info(&mut self, info: PlaybackInfo) -> Result<(), Box<dyn StdError>> {
        *self.playback_info.lock().unwrap() = Some(info.clone());

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
        let current_info = self.playback_info.lock().unwrap().clone();
        if let Some(mut info) = current_info {
            info.status = status;
            self.set_playback_info(info)?;
        }
        Ok(())
    }

    fn set_position(&mut self, position: f64) -> Result<(), Box<dyn StdError>> {
        let current_info = self.playback_info.lock().unwrap().clone();
        if let Some(mut info) = current_info {
            info.position = position;
            self.set_playback_info(info)?;
        }
        Ok(())
    }

    fn clear_metadata(&mut self) -> Result<(), Box<dyn StdError>> {
        *self.metadata.lock().unwrap() = None;

        #[cfg(target_os = "linux")]
        {
            // Send empty metadata
            if let Some(conn) = &self.connection {
                let mut changed = HashMap::new();
                changed.insert(
                    "Metadata",
                    dbus::arg::Variant(HashMap::<
                        String,
                        dbus::arg::Variant<Box<dyn dbus::arg::RefArg>>,
                    >::new()),
                );

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

    fn set_event_handler(&mut self, handler: Box<dyn Fn(MediaControlEvent) + Send>) {
        *self.event_handler.lock().unwrap() = Some(handler);
    }

    fn get_metadata(&self) -> Result<Option<MediaMetadata>, Box<dyn StdError>> {
        #[cfg(target_os = "linux")]
        {
            if let Some(conn) = &self.connection {
                let players = self.get_other_player_names(conn);
                for player in players {
                    if let Some(meta) = Self::fetch_external_metadata(conn, &player) {
                        return Ok(Some(meta));
                    }
                }
            }
        }

        // Fall back to our own metadata
        Ok(self.metadata.lock().unwrap().clone())
    }

    fn get_playback_info(&self) -> Result<Option<PlaybackInfo>, Box<dyn StdError>> {
        Ok(self.playback_info.lock().unwrap().clone())
    }

    fn get_playback_status(&self) -> Result<PlaybackStatus, Box<dyn StdError>> {
        Ok(self
            .playback_info
            .lock()
            .unwrap()
            .as_ref()
            .map(|info| info.status)
            .unwrap_or(PlaybackStatus::Stopped))
    }

    fn get_position(&self) -> Result<f64, Box<dyn StdError>> {
        Ok(self
            .playback_info
            .lock()
            .unwrap()
            .as_ref()
            .map(|info| info.position)
            .unwrap_or(0.0))
    }

    fn is_enabled(&self) -> Result<bool, Box<dyn StdError>> {
        #[cfg(target_os = "linux")]
        {
            Ok(self.connection.is_some())
        }
        #[cfg(not(target_os = "linux"))]
        {
            Ok(false)
        }
    }
}
