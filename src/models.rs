use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaMetadata {
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub duration: Option<f64>, // Duration in seconds
    pub artwork_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", with = "base64_serde")]
    pub artwork_data: Option<Vec<u8>>, // Raw image data
}

mod base64_serde {
    use super::*;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(data: &Option<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match data {
            Some(bytes) => {
                let encoded = BASE64.encode(bytes);
                serializer.serialize_str(&encoded)
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        match s {
            Some(encoded) => BASE64
                .decode(encoded)
                .map(Some)
                .map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum PlaybackStatus {
    Playing,
    Paused,
    Stopped,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaybackInfo {
    pub status: PlaybackStatus,
    pub position: f64, // Current position in seconds
    pub shuffle: bool,
    pub repeat_mode: RepeatMode,
    pub playback_rate: f64,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum RepeatMode {
    None,
    Track,
    List,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaControlEvent {
    pub event_type: MediaControlEventType,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum MediaControlEventType {
    Play,
    Pause,
    PlayPause,
    Stop,
    Next,
    Previous,
    FastForward,
    Rewind,
    SeekTo(f64),
    SetPosition(f64),
    SetPlaybackRate(f64),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeMediaSessionRequest {
    pub app_id: String,
    pub app_name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_metadata_serialization() {
        let metadata = MediaMetadata {
            title: "Test Song".to_string(),
            artist: Some("Test Artist".to_string()),
            album: Some("Test Album".to_string()),
            album_artist: Some("Test Album Artist".to_string()),
            duration: Some(180.5),
            artwork_url: Some("https://example.com/cover.jpg".to_string()),
            artwork_data: Some(vec![1, 2, 3, 4, 5]),
        };

        let json = serde_json::to_string(&metadata).unwrap();
        assert!(json.contains("\"title\":\"Test Song\""));
        assert!(json.contains("\"artist\":\"Test Artist\""));
        assert!(json.contains("\"artworkData\":\"AQIDBAU=\""));

        let deserialized: MediaMetadata = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.title, "Test Song");
        assert_eq!(deserialized.artist.as_deref(), Some("Test Artist"));
        assert_eq!(deserialized.artwork_data, Some(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_playback_info_serialization() {
        let info = PlaybackInfo {
            status: PlaybackStatus::Playing,
            position: 45.2,
            shuffle: true,
            repeat_mode: RepeatMode::Track,
            playback_rate: 1.0,
        };

        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("\"status\":\"playing\""));
        assert!(json.contains("\"repeatMode\":\"track\""));
        assert!(json.contains("\"shuffle\":true"));

        let deserialized: PlaybackInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.status, PlaybackStatus::Playing);
        assert_eq!(deserialized.repeat_mode, RepeatMode::Track);
        assert!(deserialized.shuffle);
    }

    #[test]
    fn test_media_control_event_serialization() {
        let event = MediaControlEvent {
            event_type: MediaControlEventType::Play,
            timestamp: 1700000000,
        };

        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"eventType\":\"play\""));
        assert!(json.contains("\"timestamp\":1700000000"));

        let next_event = MediaControlEvent {
            event_type: MediaControlEventType::Next,
            timestamp: 1700000001,
        };
        let next_json = serde_json::to_string(&next_event).unwrap();
        assert!(next_json.contains("\"eventType\":\"next\""));

        let prev_event = MediaControlEvent {
            event_type: MediaControlEventType::Previous,
            timestamp: 1700000002,
        };
        let prev_json = serde_json::to_string(&prev_event).unwrap();
        assert!(prev_json.contains("\"eventType\":\"previous\""));

        let seek_event = MediaControlEvent {
            event_type: MediaControlEventType::SeekTo(42.5),
            timestamp: 1700000003,
        };
        let seek_json = serde_json::to_string(&seek_event).unwrap();
        assert!(seek_json.contains("\"seekTo\":42.5"));
    }
}
