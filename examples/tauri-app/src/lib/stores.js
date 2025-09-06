import { writable } from 'svelte/store';
import { mediaControls } from 'tauri-plugin-media-api';

// Navigation state
export const currentPage = writable('overview');

// Plugin initialization state
export const pluginInitialized = writable(false);
export const mediaControlsInstance = writable(mediaControls);

// Media state
export const mediaState = writable({
  initialized: false,
  isPlaying: false,
  currentTrack: null,
  position: 0,
  duration: 0
});

// System state
export const systemMediaState = writable({
  metadata: null,
  playbackInfo: null,
  status: null,
  position: 0,
  enabled: false
});

// Notifications
export const notifications = writable([]);

export function addNotification(message, type = 'info') {
  const notification = {
    id: Date.now(),
    message,
    type, // 'success', 'error', 'info', 'warning'
    timestamp: new Date()
  };
  
  notifications.update(n => {
    return [...n, notification];
  });
  
  // Auto-remove after 5 seconds
  setTimeout(() => {
    notifications.update(n => n.filter(notif => notif.id !== notification.id));
  }, 5000);
}
