import { invoke } from '@tauri-apps/api/core'

// Media metadata interface
export interface MediaMetadata {
  title: string;
  artist?: string;
  album?: string;
  albumArtist?: string;
  duration?: number; // Duration in seconds
  artworkUrl?: string;
  artworkData?: string; // Base64 encoded image data
}

// Playback status enum
export enum PlaybackStatus {
  Playing = 'playing',
  Paused = 'paused',
  Stopped = 'stopped',
}

// Repeat mode enum
export enum RepeatMode {
  None = 'none',
  Track = 'track',
  List = 'list',
}

// Playback info interface
export interface PlaybackInfo {
  status: PlaybackStatus;
  position: number; // Current position in seconds
  shuffle: boolean;
  repeatMode: RepeatMode;
  playbackRate: number;
}

// Media control event types
export enum MediaControlEventType {
  Play = 'play',
  Pause = 'pause',
  PlayPause = 'playPause',
  Stop = 'stop',
  Next = 'next',
  Previous = 'previous',
  FastForward = 'fastForward',
  Rewind = 'rewind',
  SeekTo = 'seekTo',
  SetPosition = 'setPosition',
  SetPlaybackRate = 'setPlaybackRate',
}

// Media control event interface
export interface MediaControlEvent {
  eventType: MediaControlEventType;
  timestamp: number;
  data?: any;
}

// Initialize the media session
export async function initializeSession(appId: string, appName: string): Promise<void> {
  await invoke('plugin:media|initialize_session', {
    request: {
      appId,
      appName,
    },
  });
}

// Set media metadata
export async function setMetadata(metadata: MediaMetadata): Promise<void> {
  await invoke('plugin:media|set_metadata', {
    metadata,
  });
}

// Set playback information
export async function setPlaybackInfo(info: PlaybackInfo): Promise<void> {
  await invoke('plugin:media|set_playback_info', {
    info,
  });
}

// Set playback status
export async function setPlaybackStatus(status: PlaybackStatus): Promise<void> {
  await invoke('plugin:media|set_playback_status', {
    status,
  });
}

// Set current playback position
export async function setPosition(position: number): Promise<void> {
  await invoke('plugin:media|set_position', {
    position,
  });
}

// Clear all metadata
export async function clearMetadata(): Promise<void> {
  await invoke('plugin:media|clear_metadata');
}

// Get current metadata
export async function getMetadata(): Promise<MediaMetadata | null> {
  return await invoke('plugin:media|get_metadata');
}

// Get current playback info
export async function getPlaybackInfo(): Promise<PlaybackInfo | null> {
  return await invoke('plugin:media|get_playback_info');
}

// Get current playback status
export async function getPlaybackStatus(): Promise<PlaybackStatus> {
  return await invoke('plugin:media|get_playback_status');
}

// Get current position
export async function getPosition(): Promise<number> {
  return await invoke('plugin:media|get_position');
}

// Check if media controls are enabled
export async function isEnabled(): Promise<boolean> {
  return await invoke('plugin:media|is_enabled');
}

// Media controls class for easier usage
export class MediaControls {
  private initialized = false;

  async initialize(appId: string, appName: string): Promise<void> {
    if (this.initialized) return;
    
    await initializeSession(appId, appName);
    this.initialized = true;
  }

  async updateNowPlaying(metadata: MediaMetadata, info?: Partial<PlaybackInfo>): Promise<void> {
    await setMetadata(metadata);
    
    if (info) {
      const fullInfo: PlaybackInfo = {
        status: info.status || PlaybackStatus.Stopped,
        position: info.position || 0,
        shuffle: info.shuffle || false,
        repeatMode: info.repeatMode || RepeatMode.None,
        playbackRate: info.playbackRate || 1.0,
      };
      await setPlaybackInfo(fullInfo);
    }
  }

  async play(): Promise<void> {
    await setPlaybackStatus(PlaybackStatus.Playing);
  }

  async pause(): Promise<void> {
    await setPlaybackStatus(PlaybackStatus.Paused);
  }

  async stop(): Promise<void> {
    await setPlaybackStatus(PlaybackStatus.Stopped);
  }

  async togglePlayPause(): Promise<void> {
    const status = await getPlaybackStatus();
    if (status === PlaybackStatus.Playing) {
      await this.pause();
    } else {
      await this.play();
    }
  }

  async next(): Promise<void> {
    await invoke('plugin:media|next');
  }

  async previous(): Promise<void> {
    await invoke('plugin:media|previous');
  }

  async setPosition(position: number): Promise<void> {
    await setPosition(position);
  }

  async updatePosition(position: number): Promise<void> {
    await setPosition(position);
  }

  async updatePlaybackStatus(status: PlaybackStatus): Promise<void> {
    await setPlaybackStatus(status);
  }

  async clearNowPlaying(): Promise<void> {
    await clearMetadata();
  }

  async getMetadata(): Promise<MediaMetadata | null> {
    return await invoke('plugin:media|get_metadata');
  }

  async getPlaybackInfo(): Promise<PlaybackInfo | null> {
    return await invoke('plugin:media|get_playback_info');
  }

  async getPlaybackStatus(): Promise<PlaybackStatus> {
    return await invoke('plugin:media|get_playback_status');
  }

  async getPosition(): Promise<number> {
    return await invoke('plugin:media|get_position');
  }

  async isEnabled(): Promise<boolean> {
    return await invoke('plugin:media|is_enabled');
  }

  setEventHandler(handler: ((event: MediaControlEvent) => void) | null): void {
    // This would need actual event listener implementation
    console.warn('Event handler not yet implemented');
  }

  async clear(): Promise<void> {
    await clearMetadata();
  }
}

// Export a default instance
export const mediaControls = new MediaControls();
