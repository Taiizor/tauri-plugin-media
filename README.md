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

## Platform-Specific Implementation Details

### Windows
- Uses Windows System Media Transport Controls (SMTC)
- Full integration with Windows 10/11 media overlay
- Supports timeline scrubbing and playback rate control

### macOS
- Uses MPNowPlayingInfoCenter and MPRemoteCommandCenter
- Full integration with macOS media controls and Touch Bar
- Supports Now Playing widget in Control Center

### Linux
- Uses MPRIS (Media Player Remote Interfacing Specification) via DBus
- Compatible with GNOME, KDE, and other desktop environments
- Works with media control applets and extensions

## Example App

An example Tauri app is included in the `examples/tauri-app` directory demonstrating full media control integration with a functional music player UI.

To run the example:

```bash
cd examples/tauri-app
npm install
npm run tauri dev
```

## Development Status

- Core platform abstractions implemented
- Windows SMTC integration
- macOS MPNowPlayingInfoCenter integration  
- Linux MPRIS integration
- TypeScript/JavaScript API
- Example application
- Event callbacks from OS to app (partial implementation)
- Advanced features (playlists, queue management)

## Requirements

- Tauri v2.0+
- Rust 1.77.2+
- Platform-specific requirements:
  - Windows: Windows 10 version 1703+
  - macOS: macOS 10.13+
  - Linux: DBus-compatible desktop environment

## License

MIT OR Apache-2.0