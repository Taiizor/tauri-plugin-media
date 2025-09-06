<script>
  import { mediaControls } from 'tauri-plugin-media-api';
  import { systemMediaState, addNotification } from '../lib/stores.js';
  import { onMount, onDestroy } from 'svelte';
  
  let monitoring = false;
  let refreshInterval = null;
  let refreshRate = 1000;
  let history = [];
  let maxHistory = 10;
  
  onMount(() => {
    fetchSystemState();
  });
  
  onDestroy(() => {
    stopMonitoring();
  });
  
  async function fetchSystemState() {
    try {
      const [metadata, playbackInfo, status, position, enabled] = await Promise.all([
        mediaControls.getMetadata(),
        mediaControls.getPlaybackInfo(),
        mediaControls.getPlaybackStatus(),
        mediaControls.getPosition(),
        mediaControls.isEnabled()
      ]);
      
      systemMediaState.set({
        metadata,
        playbackInfo,
        status,
        position,
        enabled
      });
      
      // Add to history
      if (metadata || playbackInfo) {
        const entry = {
          timestamp: new Date(),
          title: metadata?.title || 'Unknown',
          artist: metadata?.artist || 'Unknown',
          status: status || 'stopped',
          position: position || 0
        };
        
        history = [entry, ...history].slice(0, maxHistory);
      }
      
      return true;
    } catch (error) {
      console.error('Failed to fetch system state:', error);
      addNotification('Failed to fetch system state', 'error');
      return false;
    }
  }
  
  function startMonitoring() {
    monitoring = true;
    refreshInterval = setInterval(fetchSystemState, refreshRate);
    addNotification('Started monitoring system media', 'success');
  }
  
  function stopMonitoring() {
    monitoring = false;
    if (refreshInterval) {
      clearInterval(refreshInterval);
      refreshInterval = null;
    }
    addNotification('Stopped monitoring', 'info');
  }
  
  function toggleMonitoring() {
    if (monitoring) {
      stopMonitoring();
    } else {
      startMonitoring();
    }
  }
  
  function clearHistory() {
    history = [];
    addNotification('History cleared', 'info');
  }
  
  function formatTime(seconds) {
    if (seconds == null) return '0:00';
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  }
  
  function formatTimestamp(date) {
    return date.toLocaleTimeString();
  }
</script>

<div class="monitor-container">
  <h1>📊 System Media Monitor</h1>
  <p class="subtitle">Monitor system-wide media playback from all applications</p>
  
  <div class="control-panel">
    <div class="monitor-controls">
      <button 
        class="btn {monitoring ? 'btn-danger' : 'btn-success'}"
        on:click={toggleMonitoring}
      >
        {monitoring ? '⏹️ Stop Monitoring' : '▶️ Start Monitoring'}
      </button>
      
      <button class="btn btn-primary" on:click={fetchSystemState}>
        🔄 Refresh Once
      </button>
      
      <div class="refresh-rate">
        <label for="refreshRate">Refresh Rate:</label>
        <select id="refreshRate" bind:value={refreshRate} disabled={monitoring}>
          <option value={500}>500ms</option>
          <option value={1000}>1s</option>
          <option value={2000}>2s</option>
          <option value={5000}>5s</option>
        </select>
      </div>
    </div>
    
    <div class="status-indicator">
      {#if monitoring}
        <span class="pulse"></span>
        <span>Monitoring Active</span>
      {:else}
        <span class="inactive"></span>
        <span>Monitoring Inactive</span>
      {/if}
    </div>
  </div>
  
  <div class="dashboard">
    <div class="current-state">
      <h2>Current System Media</h2>
      
      {#if $systemMediaState.enabled}
        <div class="media-card">
          {#if $systemMediaState.metadata}
            <div class="media-header">
              {#if $systemMediaState.metadata.artworkData || $systemMediaState.metadata.artworkUrl}
                <div class="artwork">
                  {#if $systemMediaState.metadata.artworkData}
                    <img 
                      src="data:image/jpeg;base64,{$systemMediaState.metadata.artworkData}" 
                      alt="Artwork"
                    />
                  {:else if $systemMediaState.metadata.artworkUrl}
                    <img 
                      src={$systemMediaState.metadata.artworkUrl} 
                      alt="Artwork"
                    />
                  {/if}
                </div>
              {/if}
              
              <div class="media-info">
                <h3>{$systemMediaState.metadata.title || 'Unknown Title'}</h3>
                <p class="artist">{$systemMediaState.metadata.artist || 'Unknown Artist'}</p>
                <p class="album">{$systemMediaState.metadata.album || 'Unknown Album'}</p>
              </div>
            </div>
            
            <div class="playback-bar">
              <span class="status-icon">
                {#if $systemMediaState.status === 'playing'}
                  ▶️
                {:else if $systemMediaState.status === 'paused'}
                  ⏸️
                {:else}
                  ⏹️
                {/if}
              </span>
              
              <div class="progress">
                <div 
                  class="progress-fill"
                  style="width: {$systemMediaState.metadata.duration ? 
                    ($systemMediaState.position / $systemMediaState.metadata.duration) * 100 : 0}%"
                ></div>
              </div>
              
              <span class="time">
                {formatTime($systemMediaState.position)}
                {#if $systemMediaState.metadata.duration}
                  / {formatTime($systemMediaState.metadata.duration)}
                {/if}
              </span>
            </div>
            
            {#if $systemMediaState.playbackInfo}
              <div class="playback-details">
                <span class="detail-item">
                  🔀 Shuffle: {$systemMediaState.playbackInfo.shuffle ? 'On' : 'Off'}
                </span>
                <span class="detail-item">
                  🔁 Repeat: {$systemMediaState.playbackInfo.repeatMode}
                </span>
                <span class="detail-item">
                  ⚡ Rate: {$systemMediaState.playbackInfo.playbackRate}x
                </span>
              </div>
            {/if}
          {:else}
            <div class="no-media">
              <span class="no-media-icon">🎵</span>
              <p>No media currently playing</p>
            </div>
          {/if}
        </div>
      {:else}
        <div class="disabled-state">
          <span class="disabled-icon">❌</span>
          <p>Media controls are disabled or no media player is active</p>
        </div>
      {/if}
    </div>
    
    <div class="history-panel">
      <div class="history-header">
        <h2>History</h2>
        <button class="btn btn-small" on:click={clearHistory}>Clear</button>
      </div>
      
      {#if history.length > 0}
        <div class="history-list">
          {#each history as entry}
            <div class="history-item">
              <span class="history-time">{formatTimestamp(entry.timestamp)}</span>
              <span class="history-status status-{entry.status}">
                {#if entry.status === 'playing'}
                  ▶️
                {:else if entry.status === 'paused'}
                  ⏸️
                {:else}
                  ⏹️
                {/if}
              </span>
              <div class="history-info">
                <span class="history-title">{entry.title}</span>
                <span class="history-artist">{entry.artist}</span>
              </div>
              <span class="history-position">{formatTime(entry.position)}</span>
            </div>
          {/each}
        </div>
      {:else}
        <div class="no-history">
          <p>No history yet. Start monitoring to see media history.</p>
        </div>
      {/if}
    </div>
  </div>
  
  <div class="info-card">
    <h3>💡 System Monitoring</h3>
    <p>
      This monitor reads media information from all running applications on your system.
      It can detect media from Spotify, YouTube, Chrome, Edge, VLC, and any other app
      that integrates with your system's media controls.
    </p>
  </div>
</div>

<style>
  .monitor-container {
    padding: 2rem;
    max-width: 1400px;
    margin: 0 auto;
  }
  
  .subtitle {
    color: #a0a0a0;
    margin-bottom: 2rem;
  }
  
  .control-panel {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 12px;
    padding: 1.5rem;
    margin-bottom: 2rem;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.3);
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  
  .monitor-controls {
    display: flex;
    gap: 1rem;
    align-items: center;
  }
  
  .refresh-rate {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
  
  .refresh-rate label {
    font-weight: 500;
  }
  
  .refresh-rate select {
    padding: 0.5rem;
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 6px;
    background: rgba(0, 0, 0, 0.3);
    color: #fff;
  }
  
  .status-indicator {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-weight: 500;
  }
  
  .pulse {
    width: 12px;
    height: 12px;
    background: #10b981;
    border-radius: 50%;
    animation: pulse 1.5s infinite;
  }
  
  @keyframes pulse {
    0% {
      box-shadow: 0 0 0 0 rgba(16, 185, 129, 0.7);
    }
    70% {
      box-shadow: 0 0 0 10px rgba(16, 185, 129, 0);
    }
    100% {
      box-shadow: 0 0 0 0 rgba(16, 185, 129, 0);
    }
  }
  
  .inactive {
    width: 12px;
    height: 12px;
    background: #6b7280;
    border-radius: 50%;
  }
  
  .dashboard {
    display: grid;
    grid-template-columns: 1fr 400px;
    gap: 2rem;
    margin-bottom: 2rem;
  }
  
  .current-state {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 12px;
    padding: 2rem;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.3);
  }
  
  .current-state h2 {
    margin-bottom: 1.5rem;
  }
  
  .media-card {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }
  
  .media-header {
    display: flex;
    gap: 1.5rem;
  }
  
  .artwork {
    flex-shrink: 0;
  }
  
  .artwork img {
    width: 120px;
    height: 120px;
    border-radius: 8px;
    object-fit: cover;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }
  
  .media-info {
    flex: 1;
  }
  
  .media-info h3 {
    font-size: 1.5rem;
    margin-bottom: 0.5rem;
  }
  
  .artist {
    font-size: 1.1rem;
    color: #a0a0a0;
    margin-bottom: 0.25rem;
  }
  
  .album {
    color: #808080;
  }
  
  .playback-bar {
    display: flex;
    align-items: center;
    gap: 1rem;
  }
  
  .status-icon {
    font-size: 1.5rem;
  }
  
  .progress {
    flex: 1;
    height: 6px;
    background: #e5e7eb;
    border-radius: 3px;
    overflow: hidden;
  }
  
  .progress-fill {
    height: 100%;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    transition: width 0.3s ease;
  }
  
  .time {
    color: #a0a0a0;
    font-size: 0.9rem;
    min-width: 80px;
    text-align: right;
  }
  
  .playback-details {
    display: flex;
    gap: 2rem;
    padding: 1rem;
    background: rgba(0, 0, 0, 0.2);
    border-radius: 8px;
  }
  
  .detail-item {
    font-size: 0.9rem;
    color: #a0a0a0;
  }
  
  .no-media,
  .disabled-state {
    text-align: center;
    padding: 3rem;
    color: #666;
  }
  
  .no-media-icon,
  .disabled-icon {
    font-size: 3rem;
    display: block;
    margin-bottom: 1rem;
  }
  
  .history-panel {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 12px;
    padding: 1.5rem;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.3);
    max-height: 600px;
    display: flex;
    flex-direction: column;
  }
  
  .history-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }
  
  .history-list {
    flex: 1;
    overflow-y: auto;
  }
  
  .history-item {
    display: grid;
    grid-template-columns: 60px 30px 1fr 60px;
    gap: 0.75rem;
    align-items: center;
    padding: 0.75rem;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  }
  
  .history-item:last-child {
    border-bottom: none;
  }
  
  .history-time {
    font-size: 0.8rem;
    color: #808080;
  }
  
  .history-status {
    font-size: 1.2rem;
  }
  
  .history-info {
    display: flex;
    flex-direction: column;
  }
  
  .history-title {
    font-weight: 500;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  
  .history-artist {
    font-size: 0.85rem;
    color: #a0a0a0;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  
  .history-position {
    font-size: 0.85rem;
    color: #808080;
    text-align: right;
  }
  
  .no-history {
    text-align: center;
    padding: 2rem;
    color: #666;
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
  
  .btn-success {
    background: #10b981;
    color: white;
  }
  
  .btn-danger {
    background: #ef4444;
    color: white;
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
