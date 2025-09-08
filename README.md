# Tauri Plugin Media

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/Taiizor/tauri-plugin-media/blob/develop/LICENSE-MIT)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-yellow.svg)](https://github.com/Taiizor/tauri-plugin-media/blob/develop/LICENSE-APACHE)

An advanced system media control integration plugin for Tauri applications. This plugin provides comprehensive media playback controls, rich metadata support, and seamless integration with native operating system media interfaces. It enables your Tauri application to interact with system media controls, display now-playing information, respond to media keys, and monitor system-wide media playback from other applications. Working seamlessly on Windows, macOS, and Linux platforms, it brings professional-grade media capabilities to your Tauri applications.

## Features

- **Type Safety**: Full TypeScript typings with comprehensive interfaces
- **Cross-Platform Support**: Works on Windows, macOS, and Linux
- **System Integration**: Native OS media control integration
- **Rich Metadata**: Support for title, artist, album, artwork, and more
- **Playback Control**: Play, pause, stop, next, previous controls
- **Media Key Support**: Responds to keyboard media keys
- **Position Tracking**: Real-time playback position updates
- **Shuffle & Repeat**: Full playback mode support
- **System Media Monitoring**: Read media state from other applications
- **Event Handling**: Media control event callbacks
- **Artwork Support**: Display album art in system controls
- **Playback Rate Control**: Variable speed playback support
- **Multi-App Detection**: Monitor media from Spotify, YouTube, VLC, etc.

## Installation

### Using Tauri CLI (Recommended)

The easiest way to install this plugin is using the Tauri CLI, which automatically adds both Rust and JavaScript dependencies to your project:

```bash
# Using npm
npm run tauri add media

# Using pnpm
pnpm tauri add media

# Using yarn
yarn tauri add media
```

This will:
- Add the `tauri-plugin-media` crate to your `Cargo.toml`
- Install the `tauri-plugin-media-api` npm package
- Set up the necessary configurations

### Manual Installation

If you prefer to manually install the plugin, you can follow these steps:

#### Rust Dependencies

Add this plugin to your project using one of these methods:

```bash
# Using cargo add
cargo add tauri-plugin-media
```

Or manually add to your `Cargo.toml` file:

```toml
[dependencies]
tauri = { version = "2.8.2" }
tauri-plugin-media = "0.1.1"
```

#### JavaScript/TypeScript API

Add the plugin API package to your project:

```bash
pnpm install tauri-plugin-media-api
# or
npm install tauri-plugin-media-api
# or
yarn add tauri-plugin-media-api
```

## Setup

Register the plugin in your Tauri application:

```rust
// Basic setup
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_media::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Or using the MediaExt trait for advanced usage
use tauri_plugin_media::MediaExt;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_media::init())
        .setup(|app| {
            // Access media controls through the extension
            let media = app.media();
            // Additional setup if needed
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## Permissions

By default all plugin commands are blocked and cannot be accessed. You must modify the permissions in your `capabilities` configuration to enable these.

See the [Capabilities Overview](https://v2.tauri.app/security/capabilities) for more information.

### Example Capability Configuration

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "media-access",
  "description": "Capability to access the media control functionality",
  "windows": ["main"],
  "permissions": [
    "media:default"
  ]
}
```

Then enable this capability in your `tauri.conf.json`:

```json
{
  "app": {
    "security": {
      "capabilities": ["media-access"]
    }
  }
}
```

### Default Permission

The `media:default` permission set configures which media control features are exposed by default.

#### Granted Permissions
This enables all media control operations including metadata management, playback control, and system monitoring.

#### This default permission set includes the following:
- `media:allow-initialize-session`
- `media:allow-set-metadata`
- `media:allow-set-playback-info`
- `media:allow-set-playback-status`
- `media:allow-set-position`
- `media:allow-clear-metadata`
- `media:allow-get-metadata`
- `media:allow-get-playback-info`
- `media:allow-get-playback-status`
- `media:allow-get-position`
- `media:allow-is-enabled`
- `media:allow-next`
- `media:allow-previous`

### Permission Table

| Permission | Description |
|------------|-------------|
| media:allow-initialize-session | Allows initializing the media session |
| media:deny-initialize-session | Denies initializing the media session |
| media:allow-set-metadata | Allows setting media metadata |
| media:deny-set-metadata | Denies setting media metadata |
| media:allow-set-playback-info | Allows setting playback information |
| media:deny-set-playback-info | Denies setting playback information |
| media:allow-set-playback-status | Allows setting playback status |
| media:deny-set-playback-status | Denies setting playback status |
| media:allow-set-position | Allows setting playback position |
| media:deny-set-position | Denies setting playback position |
| media:allow-clear-metadata | Allows clearing media metadata |
| media:deny-clear-metadata | Denies clearing media metadata |
| media:allow-get-metadata | Allows retrieving media metadata |
| media:deny-get-metadata | Denies retrieving media metadata |
| media:allow-get-playback-info | Allows retrieving playback information |
| media:deny-get-playback-info | Denies retrieving playback information |
| media:allow-get-playback-status | Allows retrieving playback status |
| media:deny-get-playback-status | Denies retrieving playback status |
| media:allow-get-position | Allows retrieving playback position |
| media:deny-get-position | Denies retrieving playback position |
| media:allow-is-enabled | Allows checking if media controls are enabled |
| media:deny-is-enabled | Denies checking if media controls are enabled |
| media:allow-next | Allows skipping to the next track |
| media:deny-next | Denies skipping to the next track |
| media:allow-previous | Allows skipping to the previous track |
| media:deny-previous | Denies skipping to the previous track |

## Usage

### JavaScript/TypeScript Example

```typescript
import { mediaControls, PlaybackStatus, RepeatMode } from 'tauri-plugin-media-api';

// Initialize media session
await mediaControls.initialize('my-app', 'My Music App');

// Update now playing metadata
await mediaControls.updateNowPlaying(
  {
    title: 'Bohemian Rhapsody',
    artist: 'Queen',
    album: 'A Night at the Opera',
    albumArtist: 'Queen',
    duration: 354, // seconds
    artworkUrl: 'https://example.com/artwork.jpg',
    // Or use base64 encoded image
    artworkData: 'data:image/jpeg;base64,/9j/4AAQ...'
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
await mediaControls.togglePlayPause();

// Navigate tracks
await mediaControls.next();
await mediaControls.previous();

// Update position
await mediaControls.setPosition(120); // Jump to 2 minutes
await mediaControls.updatePosition(121); // Update current position

// Get system media information
const metadata = await mediaControls.getMetadata();
if (metadata) {
  console.log(`Now playing: ${metadata.title} by ${metadata.artist}`);
}

const playbackInfo = await mediaControls.getPlaybackInfo();
if (playbackInfo) {
  console.log(`Status: ${playbackInfo.status}`);
  console.log(`Position: ${playbackInfo.position}s`);
  console.log(`Shuffle: ${playbackInfo.shuffle}`);
  console.log(`Repeat: ${playbackInfo.repeatMode}`);
}

// Check if media controls are available
const isEnabled = await mediaControls.isEnabled();
if (isEnabled) {
  console.log('Media controls are available');
}

// Clear all metadata
await mediaControls.clearNowPlaying();
```

### Rust Example

```rust
use tauri::Manager;
use tauri_plugin_media::MediaExt;

// In a command or elsewhere with access to the app handle
#[tauri::command]
async fn control_media(app_handle: tauri::AppHandle) -> Result<String, String> {
    // Access the media controls
    let media = app_handle.media();
    
    // Example: Update metadata from Rust side
    // Note: Most operations are typically done from the frontend
    // This is just to demonstrate the Rust API availability
    
    Ok("Media control operations completed".to_string())
}

// Example of setting up media controls in the app setup
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_media::init())
        .setup(|app| {
            // Initialize media controls on app startup if needed
            let media = app.media();
            // Perform any initial setup
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![control_media])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## API

### JavaScript/TypeScript API

#### Types and Enums

```typescript
// Media metadata interface
interface MediaMetadata {
  title: string;
  artist?: string;
  album?: string;
  albumArtist?: string;
  duration?: number; // Duration in seconds
  artworkUrl?: string;
  artworkData?: string; // Base64 encoded image data
}

// Playback status enum
enum PlaybackStatus {
  Playing = 'playing',
  Paused = 'paused',
  Stopped = 'stopped',
}

// Repeat mode enum
enum RepeatMode {
  None = 'none',
  Track = 'track',
  List = 'list',
}

// Playback info interface
interface PlaybackInfo {
  status: PlaybackStatus;
  position: number; // Current position in seconds
  shuffle: boolean;
  repeatMode: RepeatMode;
  playbackRate: number;
}
```

#### Methods

##### Session Management

- `initialize(appId: string, appName: string): Promise<void>` - Initialize the media session

##### Metadata Control

- `updateNowPlaying(metadata: MediaMetadata, info?: Partial<PlaybackInfo>): Promise<void>` - Update currently playing media information
- `setMetadata(metadata: MediaMetadata): Promise<void>` - Set media metadata
- `clearNowPlaying(): Promise<void>` - Clear all media metadata

##### Playback Control

- `play(): Promise<void>` - Start or resume playback
- `pause(): Promise<void>` - Pause playback
- `stop(): Promise<void>` - Stop playback
- `togglePlayPause(): Promise<void>` - Toggle between play and pause states
- `next(): Promise<void>` - Skip to the next track
- `previous(): Promise<void>` - Skip to the previous track

##### Position Control

- `setPosition(position: number): Promise<void>` - Set playback position in seconds
- `updatePosition(position: number): Promise<void>` - Update current playback position

##### Status Management

- `setPlaybackInfo(info: PlaybackInfo): Promise<void>` - Set complete playback information
- `setPlaybackStatus(status: PlaybackStatus): Promise<void>` - Set playback status
- `updatePlaybackStatus(status: PlaybackStatus): Promise<void>` - Update playback status

##### System Monitoring

- `getMetadata(): Promise<MediaMetadata | null>` - Get current media metadata from system
- `getPlaybackInfo(): Promise<PlaybackInfo | null>` - Get complete playback information
- `getPlaybackStatus(): Promise<PlaybackStatus>` - Get current playback status
- `getPosition(): Promise<number>` - Get current playback position
- `isEnabled(): Promise<boolean>` - Check if media controls are available

## Platform Implementation

### Windows
- **Technology**: Windows System Media Transport Controls (SMTC)
- **Features**: 
  - Full integration with Windows 10/11 media overlay
  - Timeline scrubbing support
  - Playback rate control
  - GlobalSystemMediaTransportControlsSessionManager for system-wide monitoring
  - Detects media from Spotify, Chrome, Edge, VLC, and other apps
- **Requirements**: Windows 10 version 1703 or later

### macOS
- **Technology**: MPNowPlayingInfoCenter and MPRemoteCommandCenter
- **Features**:
  - Full integration with macOS media controls
  - Touch Bar support
  - Now Playing widget in Control Center
  - Lock screen controls
- **Requirements**: macOS 10.13 or later

### Linux
- **Technology**: MPRIS (Media Player Remote Interfacing Specification) via DBus
- **Features**:
  - Compatible with GNOME, KDE, and other desktop environments
  - Works with media control applets and extensions
  - Full MPRIS2 specification support
- **Requirements**: DBus-compatible desktop environment

## Example Application

A comprehensive example Tauri application is included in the `examples/tauri-app` directory, featuring a modern multi-page interface that demonstrates all plugin capabilities.

### Example Features

The example application includes:

- **Overview Page**: Introduction and quick start guide
- **Media Player**: Full-featured media player with playlist support
- **Set Methods**: Interactive testing of all setter methods
- **Get Methods**: Real-time display of system media state
- **System Monitor**: Monitor media from all system applications
- **Events**: Media control event handling and simulation

### Running the Example

```bash
# Navigate to the example directory
cd examples/tauri-app

# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## Development Status

| Feature | Status | Description |
|---------|--------|-------------|
| Core Platform Abstractions | ✅ Complete | Platform-agnostic interface layer |
| Windows SMTC Integration | ✅ Complete | Full Windows media control support |
| macOS Integration | ✅ Complete | MPNowPlayingInfoCenter support |
| Linux MPRIS Integration | ✅ Complete | DBus MPRIS2 implementation |
| TypeScript API | ✅ Complete | Full TypeScript bindings |
| Example Application | ✅ Complete | Comprehensive demo app |
| System Media Reading | ✅ Complete | Get media from other apps |
| Event Callbacks | ⚠️ Partial | OS to app event handling |
| Advanced Features | ⏳ Planned | Playlists, queue management |

## Requirements

- **Tauri**: v2.0 or later
- **Rust**: 1.77.2 or later
- **Node.js**: 16.0 or later

### Platform-Specific Requirements

| Platform | Minimum Version | Additional Requirements |
|----------|-----------------|------------------------|
| Windows | Windows 10 v1703 | Windows SDK |
| macOS | macOS 10.13 | Xcode Command Line Tools |
| Linux | Any modern distro | DBus-compatible DE |

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.