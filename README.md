# Tauri Plugin Media

A Tauri v2 plugin for integrating system media controls (Now Playing / MPRIS / SMTC) across Windows, macOS, and Linux.

## Features

- **Cross-platform media controls** - Windows (SMTC), macOS (MPNowPlayingInfoCenter), Linux (MPRIS via DBus)
- **Full playback control** - Play, pause, stop, next, previous
- **Rich metadata support** - Title, artist, album, artwork
- **Playback state sync** - Position, shuffle, repeat modes
- **System integration** - Responds to media keys and system controls
- **Get system media info** - Read current media state from other apps (Spotify, YouTube, VLC, etc.)
- **Real-time monitoring** - Query system-wide media playback status

## Installation

Add the plugin to your Tauri project:

```toml
# src-tauri/Cargo.toml
[dependencies]
tauri-plugin-media = { path = "../path-to-plugin" }
```

```json
// package.json
{
  "dependencies": {
    "tauri-plugin-media-api": "file:../path-to-plugin"
  }
}
```

## Usage

### Rust Backend

Initialize the plugin in your Tauri app:

```rust
// src-tauri/src/lib.rs
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_media::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### JavaScript/TypeScript Frontend

```typescript
import { mediaControls, PlaybackStatus, RepeatMode } from 'tauri-plugin-media-api';

// Initialize media session
await mediaControls.initialize('app-id', 'App Name');

// Update now playing metadata
await mediaControls.updateNowPlaying(
  {
    title: 'Song Title',
    artist: 'Artist Name',
    album: 'Album Name',
    duration: 180, // seconds
    artworkUrl: '/path/to/artwork.jpg'
  },
  {
    status: PlaybackStatus.Playing,
    position: 0,
    shuffle: false,
    repeatMode: RepeatMode.None,
    playbackRate: 1.0
  }
);

// Control playback
await mediaControls.play();
await mediaControls.pause();
await mediaControls.stop();

// Update position
await mediaControls.updatePosition(30); // seconds

// Get system media info (NEW)
const metadata = await mediaControls.getMetadata();
const playbackInfo = await mediaControls.getPlaybackInfo();
const status = await mediaControls.getPlaybackStatus();
const position = await mediaControls.getPosition();
const isEnabled = await mediaControls.isEnabled();

// metadata returns:
// {
//   title: string,
//   artist?: string,
//   album?: string,
//   albumArtist?: string,
//   artworkUrl?: string,
//   duration?: number
// }

// playbackInfo returns:
// {
//   status: PlaybackStatus,
//   position: number,
//   shuffle: boolean,
//   repeatMode: RepeatMode,
//   playbackRate: number
// }
```

## Platform-Specific Implementation Details

### Windows
- Uses Windows System Media Transport Controls (SMTC)
- Full integration with Windows 10/11 media overlay
- Supports timeline scrubbing and playback rate control
- **NEW:** Uses GlobalSystemMediaTransportControlsSessionManager to read system-wide media state
- Can detect and retrieve media info from any playing app (Spotify, Chrome, Edge, VLC, etc.)

### macOS
- Uses MPNowPlayingInfoCenter and MPRemoteCommandCenter
- Full integration with macOS media controls and Touch Bar
- Supports Now Playing widget in Control Center

### Linux
- Uses MPRIS (Media Player Remote Interfacing Specification) via DBus
- Compatible with GNOME, KDE, and other desktop environments
- Works with media control applets and extensions

## Example App

A comprehensive example Tauri app is included in the `examples/tauri-app` directory with a modern multi-page interface demonstrating all plugin features.

### Example App Pages:
- **Overview** - Plugin introduction and quick start guide
- **Media Player** - Full-featured media player with playlist support
- **Set Methods** - Interactive testing of all set methods
- **Get Methods** - Real-time display of system media state
- **System Monitor** - Monitor media from all system applications
- **Events** - Media control event handling and simulation

To run the example:

```bash
cd examples/tauri-app
npm install
npm run tauri dev
```

## API Reference

### Set Methods
- `initialize(appId: string, appName: string)` - Initialize the media controller
- `updateNowPlaying(metadata: MediaMetadata, playbackInfo: PlaybackInfo)` - Update current media info
- `play()` - Start playback
- `pause()` - Pause playback
- `stop()` - Stop playback
- `togglePlayPause()` - Toggle between play and pause
- `next()` - Skip to next track
- `previous()` - Skip to previous track
- `setPosition(position: number)` - Set playback position in seconds
- `updatePosition(position: number)` - Update current position
- `updatePlaybackStatus(status: PlaybackStatus)` - Update playback state
- `clearNowPlaying()` - Clear all media info

### Get Methods (NEW)
- `getMetadata()` - Get current media metadata from system
- `getPlaybackInfo()` - Get playback state (status, position, shuffle, repeat)
- `getPlaybackStatus()` - Get current playback status
- `getPosition()` - Get current playback position
- `isEnabled()` - Check if media controls are available

## Development Status

- ✅ Core platform abstractions implemented
- ✅ Windows SMTC integration with system-wide media reading
- ✅ macOS MPNowPlayingInfoCenter integration  
- ✅ Linux MPRIS integration
- ✅ TypeScript/JavaScript API
- ✅ Example application
- ✅ Get methods for reading system media state
- ⚠️ Event callbacks from OS to app (partial implementation)
- ⏳ Advanced features (playlists, queue management)

## Requirements

- Tauri v2.0+
- Rust 1.77.2+
- Platform-specific requirements:
  - Windows: Windows 10 version 1703+
  - macOS: macOS 10.13+
  - Linux: DBus-compatible desktop environment

## License

MIT OR Apache-2.0