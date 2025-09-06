<script>
  import { mediaControls } from 'tauri-plugin-media-api';
  import { addNotification } from '../lib/stores.js';
  
  let initialized = false;
  
  async function initializePlugin() {
    try {
      await mediaControls.initialize('tauri-media-example', 'Tauri Media Example');
      initialized = true;
      addNotification('Media plugin initialized successfully!', 'success');
    } catch (error) {
      addNotification(`Failed to initialize: ${error}`, 'error');
    }
  }
</script>

<div class="overview-container">
  <header class="page-header">
    <h1>Welcome to Tauri Media Plugin</h1>
    <p>Cross-platform media control integration for your Tauri applications</p>
  </header>
  
  <div class="quick-start">
    <h2>🚀 Quick Start</h2>
    <div class="init-section">
      {#if !initialized}
        <button class="btn btn-primary btn-large" on:click={initializePlugin}>
          Initialize Media Plugin
        </button>
        <p class="hint">Initialize the plugin to start using media controls</p>
      {:else}
        <div class="success-badge">
          ✅ Plugin Initialized
        </div>
      {/if}
    </div>
  </div>
  
  <div class="features-grid">
    <div class="feature-card">
      <span class="feature-icon">🎵</span>
      <h3>Media Playback Control</h3>
      <p>Control media playback with play, pause, stop, next, and previous commands</p>
    </div>
    
    <div class="feature-card">
      <span class="feature-icon">📝</span>
      <h3>Rich Metadata</h3>
      <p>Set and retrieve detailed metadata including title, artist, album, and artwork</p>
    </div>
    
    <div class="feature-card">
      <span class="feature-icon">🖼️</span>
      <h3>Artwork Support</h3>
      <p>Display album artwork from URLs or binary data with automatic encoding</p>
    </div>
    
    <div class="feature-card">
      <span class="feature-icon">🎛️</span>
      <h3>System Integration</h3>
      <p>Integrate with Windows SMTC, macOS Now Playing, and Linux MPRIS</p>
    </div>
    
    <div class="feature-card">
      <span class="feature-icon">📊</span>
      <h3>System Monitoring</h3>
      <p>Read media state from other applications like Spotify, YouTube, and VLC</p>
    </div>
    
    <div class="feature-card">
      <span class="feature-icon">⚡</span>
      <h3>Media Events</h3>
      <p>Respond to system media control events and keyboard media keys</p>
    </div>
  </div>
  
  <div class="platform-support">
    <h2>🖥️ Platform Support</h2>
    <div class="platforms">
      <div class="platform">
        <span class="platform-icon">🪟</span>
        <h4>Windows</h4>
        <p>Windows 10/11 with SMTC</p>
      </div>
      <div class="platform">
        <span class="platform-icon">🍎</span>
        <h4>macOS</h4>
        <p>macOS 10.13+ with MPNowPlayingInfoCenter</p>
      </div>
      <div class="platform">
        <span class="platform-icon">🐧</span>
        <h4>Linux</h4>
        <p>MPRIS via DBus</p>
      </div>
    </div>
  </div>
  
  <div class="code-example">
    <h2>💻 Quick Example</h2>
    <pre><code>{`import { mediaControls, PlaybackStatus } from 'tauri-plugin-media-api';

// Initialize
await mediaControls.initialize('my-app', 'My App');

// Set metadata
await mediaControls.updateNowPlaying({
  title: 'Song Title',
  artist: 'Artist Name',
  album: 'Album Name',
  artworkUrl: 'https://example.com/cover.jpg'
}, {
  status: PlaybackStatus.Playing,
  position: 0,
  shuffle: false,
  repeatMode: RepeatMode.None,
  playbackRate: 1.0
});

// Control playback
await mediaControls.play();
await mediaControls.pause();

// Get system media info
const metadata = await mediaControls.getMetadata();
const status = await mediaControls.getPlaybackStatus();`}</code></pre>
  </div>
</div>

<style>
  .overview-container {
    padding: 2rem;
    max-width: 1200px;
    margin: 0 auto;
  }
  
  .page-header {
    text-align: center;
    margin-bottom: 3rem;
  }
  
  .page-header h1 {
    font-size: 2.5rem;
    margin-bottom: 1rem;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
  }
  
  .page-header p {
    font-size: 1.2rem;
    color: #666;
  }
  
  .quick-start {
    background: white;
    border-radius: 16px;
    padding: 2rem;
    margin-bottom: 3rem;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
  }
  
  .quick-start h2 {
    margin-bottom: 1.5rem;
  }
  
  .init-section {
    text-align: center;
  }
  
  .btn {
    padding: 0.75rem 1.5rem;
    border: none;
    border-radius: 8px;
    font-size: 1rem;
    cursor: pointer;
    transition: all 0.3s ease;
  }
  
  .btn-primary {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
  }
  
  .btn-large {
    padding: 1rem 2rem;
    font-size: 1.1rem;
  }
  
  .btn:hover {
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(102, 126, 234, 0.4);
  }
  
  .hint {
    margin-top: 1rem;
    color: #666;
  }
  
  .success-badge {
    display: inline-block;
    background: #10b981;
    color: white;
    padding: 1rem 2rem;
    border-radius: 8px;
    font-size: 1.1rem;
    font-weight: 500;
  }
  
  .features-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 1.5rem;
    margin-bottom: 3rem;
  }
  
  .feature-card {
    background: white;
    border-radius: 12px;
    padding: 1.5rem;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.05);
    transition: all 0.3s ease;
  }
  
  .feature-card:hover {
    transform: translateY(-5px);
    box-shadow: 0 8px 25px rgba(0, 0, 0, 0.1);
  }
  
  .feature-icon {
    font-size: 2rem;
    display: block;
    margin-bottom: 1rem;
  }
  
  .feature-card h3 {
    margin-bottom: 0.5rem;
    color: #333;
  }
  
  .feature-card p {
    color: #666;
    line-height: 1.5;
  }
  
  .platform-support {
    background: white;
    border-radius: 16px;
    padding: 2rem;
    margin-bottom: 3rem;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
  }
  
  .platforms {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 2rem;
    margin-top: 2rem;
  }
  
  .platform {
    text-align: center;
  }
  
  .platform-icon {
    font-size: 3rem;
    display: block;
    margin-bottom: 1rem;
  }
  
  .platform h4 {
    margin-bottom: 0.5rem;
  }
  
  .platform p {
    color: #666;
    font-size: 0.9rem;
  }
  
  .code-example {
    background: white;
    border-radius: 16px;
    padding: 2rem;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
  }
  
  .code-example h2 {
    margin-bottom: 1.5rem;
  }
  
  pre {
    background: #1e1e1e;
    color: #d4d4d4;
    padding: 1.5rem;
    border-radius: 8px;
    overflow-x: auto;
  }
  
  code {
    font-family: 'Fira Code', 'Consolas', monospace;
    font-size: 0.9rem;
    line-height: 1.5;
  }
</style>
