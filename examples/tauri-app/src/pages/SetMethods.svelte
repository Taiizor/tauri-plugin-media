<script>
  import { mediaControls, PlaybackStatus, RepeatMode } from 'tauri-plugin-media-api';
  import { addNotification } from '../lib/stores.js';
  
  let initialized = false;
  
  // Form data
  let metadata = {
    title: 'Test Song',
    artist: 'Test Artist',
    album: 'Test Album',
    albumArtist: 'Test Album Artist',
    duration: 180,
    artworkUrl: 'https://picsum.photos/300/300'
  };
  
  let playbackInfo = {
    status: PlaybackStatus.Playing,
    position: 0,
    shuffle: false,
    repeatMode: RepeatMode.None,
    playbackRate: 1.0
  };
  
  let position = 0;
  
  async function initialize() {
    try {
      await mediaControls.initialize('set-methods-test', 'Set Methods Test');
      initialized = true;
      addNotification('Initialized successfully', 'success');
    } catch (error) {
      addNotification(`Failed to initialize: ${error}`, 'error');
    }
  }
  
  async function testUpdateNowPlaying() {
    try {
      await mediaControls.updateNowPlaying(metadata, playbackInfo);
      addNotification('Updated now playing info', 'success');
    } catch (error) {
      addNotification(`Failed to update: ${error}`, 'error');
    }
  }
  
  async function testPlay() {
    try {
      await mediaControls.play();
      playbackInfo.status = PlaybackStatus.Playing;
      addNotification('Play command sent', 'success');
    } catch (error) {
      addNotification(`Failed to play: ${error}`, 'error');
    }
  }
  
  async function testPause() {
    try {
      await mediaControls.pause();
      playbackInfo.status = PlaybackStatus.Paused;
      addNotification('Pause command sent', 'success');
    } catch (error) {
      addNotification(`Failed to pause: ${error}`, 'error');
    }
  }
  
  async function testStop() {
    try {
      await mediaControls.stop();
      playbackInfo.status = PlaybackStatus.Stopped;
      addNotification('Stop command sent', 'success');
    } catch (error) {
      addNotification(`Failed to stop: ${error}`, 'error');
    }
  }
  
  async function testTogglePlayPause() {
    try {
      await mediaControls.togglePlayPause();
      addNotification('Toggle play/pause command sent', 'success');
    } catch (error) {
      addNotification(`Failed to toggle: ${error}`, 'error');
    }
  }
  
  async function testNext() {
    try {
      await mediaControls.next();
      addNotification('Next command sent', 'success');
    } catch (error) {
      addNotification(`Failed to skip next: ${error}`, 'error');
    }
  }
  
  async function testPrevious() {
    try {
      await mediaControls.previous();
      addNotification('Previous command sent', 'success');
    } catch (error) {
      addNotification(`Failed to skip previous: ${error}`, 'error');
    }
  }
  
  async function testSetPosition() {
    try {
      await mediaControls.setPosition(position);
      addNotification(`Position set to ${position}s`, 'success');
    } catch (error) {
      addNotification(`Failed to set position: ${error}`, 'error');
    }
  }
  
  async function testUpdatePosition() {
    try {
      await mediaControls.updatePosition(position);
      addNotification(`Position updated to ${position}s`, 'success');
    } catch (error) {
      addNotification(`Failed to update position: ${error}`, 'error');
    }
  }
  
  async function testUpdatePlaybackStatus() {
    try {
      await mediaControls.updatePlaybackStatus(playbackInfo.status);
      addNotification(`Status updated to ${playbackInfo.status}`, 'success');
    } catch (error) {
      addNotification(`Failed to update status: ${error}`, 'error');
    }
  }
  
  async function testClearNowPlaying() {
    try {
      await mediaControls.clearNowPlaying();
      addNotification('Cleared now playing info', 'success');
    } catch (error) {
      addNotification(`Failed to clear: ${error}`, 'error');
    }
  }
  
  async function loadImageAsBase64() {
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = 'image/*';
    async function handleArtworkUpload(event) {
      const input = event.target;
      const file = input.files?.[0];
      if (!file) return;
      
      const reader = new FileReader();
      reader.onload = (e) => {
        const result = e.target?.result;
        if (typeof result === 'string') {
          const base64 = result.split(',')[1];
          metadata.artworkData = base64;
        }
      };
      reader.readAsDataURL(file);
    }
    input.onchange = handleArtworkUpload;
    input.click();
  }
</script>

<div class="set-methods-container">
  <h1>📝 Set Methods</h1>
  <p class="subtitle">Test all available set methods for media control</p>
  
  {#if !initialized}
    <div class="init-card">
      <button class="btn btn-primary" on:click={initialize}>
        Initialize Media Controls
      </button>
    </div>
  {:else}
    <div class="success-badge">✅ Media Controls Initialized</div>
  {/if}
  
  <div class="method-sections">
    <section class="method-card">
      <h2>Metadata Configuration</h2>
      <div class="form-grid">
        <div class="form-field">
          <label for="title">Title</label>
          <input id="title" type="text" bind:value={metadata.title} />
        </div>
        
        <div class="form-field">
          <label for="artist">Artist</label>
          <input id="artist" type="text" bind:value={metadata.artist} />
        </div>
        
        <div class="form-field">
          <label for="album">Album</label>
          <input id="album" type="text" bind:value={metadata.album} />
        </div>
        
        <div class="form-field">
          <label for="albumArtist">Album Artist</label>
          <input id="albumArtist" type="text" bind:value={metadata.albumArtist} />
        </div>
        
        <div class="form-field">
          <label for="duration">Duration (seconds)</label>
          <input id="duration" type="number" bind:value={metadata.duration} />
        </div>
        
        <div class="form-field">
          <label for="artworkUrl">Artwork URL</label>
          <input id="artworkUrl" type="text" bind:value={metadata.artworkUrl} />
        </div>
      </div>
      
      <div class="button-group">
        <button class="btn btn-secondary" on:click={loadImageAsBase64}>
          📷 Load Image as Base64
        </button>
      </div>
    </section>
    
    <section class="method-card">
      <h2>Playback Configuration</h2>
      <div class="form-grid">
        <div class="form-field">
          <label for="status">Status</label>
          <select id="status" bind:value={playbackInfo.status}>
            <option value={PlaybackStatus.Playing}>Playing</option>
            <option value={PlaybackStatus.Paused}>Paused</option>
            <option value={PlaybackStatus.Stopped}>Stopped</option>
          </select>
        </div>
        
        <div class="form-field">
          <label for="playbackPosition">Position (seconds)</label>
          <input id="playbackPosition" type="number" bind:value={playbackInfo.position} />
        </div>
        
        <div class="form-field">
          <label for="shuffle">
            <input id="shuffle" type="checkbox" bind:checked={playbackInfo.shuffle} />
            Shuffle
          </label>
        </div>
        
        <div class="form-field">
          <label for="repeatMode">Repeat Mode</label>
          <select id="repeatMode" bind:value={playbackInfo.repeatMode}>
            <option value={RepeatMode.None}>None</option>
            <option value={RepeatMode.Track}>Track</option>
            <option value={RepeatMode.List}>List</option>
          </select>
        </div>
        
        <div class="form-field">
          <label for="playbackRate">Playback Rate</label>
          <input id="playbackRate" type="number" step="0.1" bind:value={playbackInfo.playbackRate} />
        </div>
      </div>
    </section>
    
    <section class="method-card">
      <h2>Update Methods</h2>
      <div class="method-grid">
        <button class="btn btn-primary" on:click={testUpdateNowPlaying} disabled={!initialized}>
          updateNowPlaying()
        </button>
        <button class="btn btn-primary" on:click={testUpdatePlaybackStatus} disabled={!initialized}>
          updatePlaybackStatus()
        </button>
        <button class="btn btn-primary" on:click={testClearNowPlaying} disabled={!initialized}>
          clearNowPlaying()
        </button>
      </div>
    </section>
    
    <section class="method-card">
      <h2>Playback Control</h2>
      <div class="method-grid">
        <button class="btn btn-success" on:click={testPlay} disabled={!initialized}>
          ▶️ play()
        </button>
        <button class="btn btn-warning" on:click={testPause} disabled={!initialized}>
          ⏸️ pause()
        </button>
        <button class="btn btn-danger" on:click={testStop} disabled={!initialized}>
          ⏹️ stop()
        </button>
        <button class="btn btn-primary" on:click={testTogglePlayPause} disabled={!initialized}>
          ⏯️ togglePlayPause()
        </button>
        <button class="btn btn-primary" on:click={testNext} disabled={!initialized}>
          ⏭️ next()
        </button>
        <button class="btn btn-primary" on:click={testPrevious} disabled={!initialized}>
          ⏮️ previous()
        </button>
      </div>
    </section>
    
    <section class="method-card">
      <h2>Position Control</h2>
      <div class="form-field">
        <label for="position">Position (seconds)</label>
        <input id="position" type="number" bind:value={position} />
      </div>
      <div class="button-group">
        <button class="btn btn-primary" on:click={testSetPosition} disabled={!initialized}>
          setPosition({position})
        </button>
        <button class="btn btn-primary" on:click={testUpdatePosition} disabled={!initialized}>
          updatePosition({position})
        </button>
      </div>
    </section>
  </div>
</div>

<style>
  .set-methods-container {
    padding: 2rem;
    max-width: 1200px;
    margin: 0 auto;
  }
  
  .subtitle {
    color: #a0a0a0;
    margin-bottom: 2rem;
  }
  
  .success-badge {
    background: #10b981;
    color: white;
    padding: 1rem;
    border-radius: 8px;
    text-align: center;
    margin-bottom: 2rem;
    font-weight: 500;
  }
  
  .form-field label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 500;
    color: #a0a0a0;
  }
  
  .method-sections {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }
  
  
  .method-card {
    background: rgba(30, 30, 40, 0.8) !important;
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 12px;
    padding: 2rem;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.5);
  }
  
  .init-card {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    padding: 2rem;
    border-radius: 12px;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.3);
    text-align: center;
    margin-bottom: 2rem;
    color: #fff;
  }
  
  .method-card h2 {
    margin-bottom: 1.5rem;
    color: #fff;
  }
  
  
  .form-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1.5rem;
    margin-bottom: 1.5rem;
  }
  
  .form-field {
    display: flex;
    flex-direction: column;
  }
  
  .form-field input {
    padding: 0.75rem;
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 6px;
    font-size: 1rem;
    background: rgba(0, 0, 0, 0.3);
    color: #fff;
  }
  
  .form-field input:focus {
    outline: none;
    border-color: #667eea;
  }
  
  .form-field select {
    padding: 0.75rem;
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 6px;
    font-size: 1rem;
    background: rgba(0, 0, 0, 0.3);
    color: #fff;
  }
  
  .form-field select option {
    background: #2a2a3a;
    color: #fff;
  }
  
  .form-field select:focus {
    outline: none;
    border-color: #667eea;
  }
  
  .form-field input[type="checkbox"] {
    width: auto;
    margin-right: 0.5rem;
    accent-color: #667eea;
  }
  
  .method-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
  }
  
  .button-group {
    display: flex;
    gap: 1rem;
    flex-wrap: wrap;
  }
  
  .btn {
    padding: 0.75rem 1.5rem;
    border: none;
    border-radius: 6px;
    font-size: 1rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.3s ease;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
  }
  
  .btn:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  }
  
  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  
  .btn-primary {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
  }
  
  .btn-secondary {
    background: #6b7280;
    color: white;
  }
  
  .btn-success {
    background: #10b981;
    color: white;
  }
  
  .btn-warning {
    background: #f59e0b;
    color: white;
  }
  
  .btn-danger {
    background: #ef4444;
    color: white;
  }
</style>
