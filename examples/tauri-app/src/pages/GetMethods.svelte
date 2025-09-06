<script>
  import { mediaControls } from 'tauri-plugin-media-api';
  import { addNotification } from '../lib/stores.js';
  import { onMount, onDestroy } from 'svelte';
  
  let results = {
    metadata: null,
    playbackInfo: null,
    playbackStatus: null,
    position: null,
    enabled: null
  };
  
  let autoRefresh = false;
  let refreshInterval = null;
  
  onMount(() => {
    testAll();
  });
  
  onDestroy(() => {
    if (refreshInterval) {
      clearInterval(refreshInterval);
    }
  });
  
  $: if (autoRefresh && !refreshInterval) {
    refreshInterval = setInterval(() => {
      testAll();
    }, 1000);
  } else if (!autoRefresh && refreshInterval) {
    clearInterval(refreshInterval);
    refreshInterval = null;
  }
  
  async function testGetMetadata() {
    try {
      results.metadata = await mediaControls.getMetadata();
      addNotification('Got metadata successfully', 'success');
    } catch (error) {
      addNotification(`Failed to get metadata: ${error}`, 'error');
      results.metadata = null;
    }
  }
  
  async function testGetPlaybackInfo() {
    try {
      results.playbackInfo = await mediaControls.getPlaybackInfo();
      addNotification('Got playback info successfully', 'success');
    } catch (error) {
      addNotification(`Failed to get playback info: ${error}`, 'error');
      results.playbackInfo = null;
    }
  }
  
  async function testGetPlaybackStatus() {
    try {
      results.playbackStatus = await mediaControls.getPlaybackStatus();
      addNotification('Got playback status successfully', 'success');
    } catch (error) {
      addNotification(`Failed to get playback status: ${error}`, 'error');
      results.playbackStatus = null;
    }
  }
  
  async function testGetPosition() {
    try {
      results.position = await mediaControls.getPosition();
      addNotification('Got position successfully', 'success');
    } catch (error) {
      addNotification(`Failed to get position: ${error}`, 'error');
      results.position = null;
    }
  }
  
  async function testIsEnabled() {
    try {
      results.enabled = await mediaControls.isEnabled();
      addNotification('Got enabled status successfully', 'success');
    } catch (error) {
      addNotification(`Failed to get enabled status: ${error}`, 'error');
      results.enabled = null;
    }
  }
  
  async function testAll() {
    await Promise.all([
      testGetMetadata(),
      testGetPlaybackInfo(),
      testGetPlaybackStatus(),
      testGetPosition(),
      testIsEnabled()
    ]);
  }
  
  function formatTime(seconds) {
    if (seconds == null) return 'N/A';
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  }
</script>

<div class="get-methods-container">
  <h1>📖 Get Methods</h1>
  <p class="subtitle">Test all available get methods to read media state</p>
  
  <div class="controls-bar">
    <button class="btn btn-primary" on:click={testAll}>
      🔄 Refresh All
    </button>
    
    <label class="auto-refresh">
      <input type="checkbox" bind:checked={autoRefresh} />
      Auto-refresh (1s)
    </label>
  </div>
  
  <div class="methods-grid">
    <div class="method-result">
      <div class="method-header">
        <h3>getMetadata()</h3>
        <button class="btn btn-small" on:click={testGetMetadata}>Test</button>
      </div>
      <div class="result-content">
        {#if results.metadata}
          <div class="metadata-display">
            {#if results.metadata.artworkData || results.metadata.artworkUrl}
              <div class="artwork-preview">
                {#if results.metadata.artworkData}
                  <img 
                    src="data:image/jpeg;base64,{results.metadata.artworkData}" 
                    alt="Artwork"
                  />
                {:else if results.metadata.artworkUrl}
                  <img 
                    src={results.metadata.artworkUrl} 
                    alt="Artwork"
                  />
                {/if}
              </div>
            {/if}
            <div class="metadata-details">
              <div class="field">
                <span class="label">Title:</span>
                <span class="value">{results.metadata.title || 'N/A'}</span>
              </div>
              <div class="field">
                <span class="label">Artist:</span>
                <span class="value">{results.metadata.artist || 'N/A'}</span>
              </div>
              <div class="field">
                <span class="label">Album:</span>
                <span class="value">{results.metadata.album || 'N/A'}</span>
              </div>
              <div class="field">
                <span class="label">Album Artist:</span>
                <span class="value">{results.metadata.albumArtist || 'N/A'}</span>
              </div>
              <div class="field">
                <span class="label">Duration:</span>
                <span class="value">{formatTime(results.metadata.duration)}</span>
              </div>
              <div class="field">
                <span class="label">Has Artwork:</span>
                <span class="value">
                  {results.metadata.artworkData || results.metadata.artworkUrl ? '✅ Yes' : '❌ No'}
                </span>
              </div>
            </div>
          </div>
        {:else}
          <div class="no-data">No metadata available</div>
        {/if}
      </div>
    </div>
    
    <div class="method-result">
      <div class="method-header">
        <h3>getPlaybackInfo()</h3>
        <button class="btn btn-small" on:click={testGetPlaybackInfo}>Test</button>
      </div>
      <div class="result-content">
        {#if results.playbackInfo}
          <div class="field">
            <span class="label">Status:</span>
            <span class="value status-badge status-{results.playbackInfo.status}">
              {results.playbackInfo.status}
            </span>
          </div>
          <div class="field">
            <span class="label">Position:</span>
            <span class="value">{formatTime(results.playbackInfo.position)}</span>
          </div>
          <div class="field">
            <span class="label">Shuffle:</span>
            <span class="value">{results.playbackInfo.shuffle ? '✅ On' : '❌ Off'}</span>
          </div>
          <div class="field">
            <span class="label">Repeat Mode:</span>
            <span class="value">{results.playbackInfo.repeatMode}</span>
          </div>
          <div class="field">
            <span class="label">Playback Rate:</span>
            <span class="value">{results.playbackInfo.playbackRate}x</span>
          </div>
        {:else}
          <div class="no-data">No playback info available</div>
        {/if}
      </div>
    </div>
    
    <div class="method-result">
      <div class="method-header">
        <h3>getPlaybackStatus()</h3>
        <button class="btn btn-small" on:click={testGetPlaybackStatus}>Test</button>
      </div>
      <div class="result-content">
        {#if results.playbackStatus !== null}
          <div class="big-status status-{results.playbackStatus}">
            {#if results.playbackStatus === 'playing'}
              ▶️ Playing
            {:else if results.playbackStatus === 'paused'}
              ⏸️ Paused
            {:else}
              ⏹️ Stopped
            {/if}
          </div>
        {:else}
          <div class="no-data">No status available</div>
        {/if}
      </div>
    </div>
    
    <div class="method-result">
      <div class="method-header">
        <h3>getPosition()</h3>
        <button class="btn btn-small" on:click={testGetPosition}>Test</button>
      </div>
      <div class="result-content">
        {#if results.position !== null}
          <div class="big-value">
            {formatTime(results.position)}
          </div>
        {:else}
          <div class="no-data">No position available</div>
        {/if}
      </div>
    </div>
    
    <div class="method-result">
      <div class="method-header">
        <h3>isEnabled()</h3>
        <button class="btn btn-small" on:click={testIsEnabled}>Test</button>
      </div>
      <div class="result-content">
        {#if results.enabled !== null}
          <div class="big-status">
            {results.enabled ? '✅ Enabled' : '❌ Disabled'}
          </div>
        {:else}
          <div class="no-data">No enabled status available</div>
        {/if}
      </div>
    </div>
  </div>
  
  <div class="info-card">
    <h3>ℹ️ About Get Methods</h3>
    <p>
      These methods retrieve the current media state from the system. On Windows, this uses
      GlobalSystemMediaTransportControlsSessionManager to read from any playing media app
      (Spotify, YouTube, VLC, etc.). On macOS and Linux, it reads from MPNowPlayingInfoCenter
      and MPRIS respectively.
    </p>
  </div>
</div>

<style>
  .get-methods-container {
    padding: 2rem;
    max-width: 1200px;
    margin: 0 auto;
  }
  
  .subtitle {
    color: #666;
    margin-bottom: 2rem;
  }
  
  .controls-bar {
    display: flex;
    align-items: center;
    gap: 2rem;
    margin-bottom: 2rem;
    padding: 1rem;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 12px;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.3);
  }
  
  .auto-refresh {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-weight: 500;
  }
  
  .methods-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
    gap: 1.5rem;
    margin-bottom: 2rem;
  }
  
  .method-result {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 12px;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.3);
    overflow: hidden;
  }
  
  .method-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 1.5rem;
    background: linear-gradient(135deg, rgba(102, 126, 234, 0.1) 0%, rgba(118, 75, 162, 0.1) 100%);
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  }
  
  .method-header h3 {
    margin: 0;
    font-family: 'Fira Code', monospace;
    font-size: 1.1rem;
    color: #fff;
  }
  
  .result-content {
    padding: 1.5rem;
    min-height: 150px;
  }
  
  .metadata-display {
    display: flex;
    gap: 1.5rem;
  }
  
  .artwork-preview {
    flex-shrink: 0;
  }
  
  .artwork-preview img {
    width: 100px;
    height: 100px;
    border-radius: 8px;
    object-fit: cover;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }
  
  .metadata-details {
    flex: 1;
  }
  
  .field {
    display: flex;
    margin-bottom: 0.75rem;
  }
  
  .label {
    font-weight: 500;
    color: #a0a0a0;
    margin-right: 0.75rem;
    min-width: 120px;
  }
  
  .value {
    color: #fff;
    word-break: break-word;
  }
  
  .status-badge {
    padding: 0.25rem 0.75rem;
    border-radius: 4px;
    font-weight: 500;
  }
  
  .status-playing {
    background: #10b981;
    color: white;
  }
  
  .status-paused {
    background: #f59e0b;
    color: white;
  }
  
  .status-stopped {
    background: #ef4444;
    color: white;
  }
  
  .big-status,
  .big-value {
    font-size: 2rem;
    font-weight: 600;
    text-align: center;
    padding: 2rem;
  }
  
  .no-data {
    color: #666;
    text-align: center;
    padding: 2rem;
    font-style: italic;
  }
  
  .btn {
    padding: 0.75rem 1.5rem;
    border: none;
    border-radius: 6px;
    font-size: 1rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.3s ease;
  }
  
  .btn-primary {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
  }
  
  .btn-small {
    padding: 0.5rem 1rem;
    font-size: 0.875rem;
    background: #6b7280;
    color: white;
  }
  
  .btn:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  }
  
  .info-card {
    background: rgba(59, 130, 246, 0.1);
    border: 1px solid rgba(59, 130, 246, 0.3);
    border-radius: 12px;
    padding: 1.5rem;
  }
  
  .info-card h3 {
    margin-bottom: 0.75rem;
    color: #60a5fa;
  }
  
  .info-card p {
    color: #93c5fd;
    line-height: 1.6;
  }
</style>
