<script>
  import { mediaControls, PlaybackStatus, RepeatMode } from 'tauri-plugin-media-api';
  import { mediaState, addNotification } from '../lib/stores.js';
  import { onMount, onDestroy } from 'svelte';
  
  let initialized = false;
  let currentTime = 0;
  let interval;
  
  // Sample tracks
  const tracks = [
    {
      id: 1,
      title: 'Summer Breeze',
      artist: 'Relaxing Sounds',
      album: 'Chill Vibes',
      duration: 245,
      artwork: 'https://picsum.photos/300/300?random=1'
    },
    {
      id: 2,
      title: 'Ocean Waves',
      artist: 'Nature Sounds',
      album: 'Peaceful Moments',
      duration: 312,
      artwork: 'https://picsum.photos/300/300?random=2'
    },
    {
      id: 3,
      title: 'City Lights',
      artist: 'Urban Jazz',
      album: 'Night Sessions',
      duration: 198,
      artwork: 'https://picsum.photos/300/300?random=3'
    }
  ];
  
  let currentTrackIndex = 0;
  let isPlaying = false;
  let shuffle = false;
  let repeatMode = RepeatMode.None;
  let volume = 70;
  
  $: currentTrack = tracks[currentTrackIndex];
  $: progress = currentTrack ? (currentTime / currentTrack.duration) * 100 : 0;
  
  onMount(async () => {
    try {
      await mediaControls.initialize('media-player-demo', 'Media Player Demo');
      initialized = true;
      addNotification('Media player initialized', 'success');
    } catch (error) {
      addNotification(`Failed to initialize: ${error}`, 'error');
    }
  });
  
  onDestroy(() => {
    if (interval) clearInterval(interval);
  });
  
  async function updateMediaSession() {
    if (!initialized || !currentTrack) return;
    
    try {
      await mediaControls.updateNowPlaying(
        {
          title: currentTrack.title,
          artist: currentTrack.artist,
          album: currentTrack.album,
          duration: currentTrack.duration,
          artworkUrl: currentTrack.artwork
        },
        {
          status: isPlaying ? PlaybackStatus.Playing : PlaybackStatus.Paused,
          position: currentTime,
          shuffle: shuffle,
          repeatMode: repeatMode,
          playbackRate: 1.0
        }
      );
    } catch (error) {
      console.error('Failed to update media session:', error);
    }
  }
  
  async function play() {
    if (!initialized) return;
    
    isPlaying = true;
    await mediaControls.play();
    await updateMediaSession();
    
    interval = setInterval(() => {
      if (currentTime < currentTrack.duration) {
        currentTime += 1;
        mediaControls.updatePosition(currentTime);
      } else {
        next();
      }
    }, 1000);
  }
  
  async function pause() {
    if (!initialized) return;
    
    isPlaying = false;
    await mediaControls.pause();
    await updateMediaSession();
    
    if (interval) {
      clearInterval(interval);
      interval = null;
    }
  }
  
  async function togglePlayPause() {
    if (isPlaying) {
      await pause();
    } else {
      await play();
    }
  }
  
  async function stop() {
    if (!initialized) return;
    
    isPlaying = false;
    currentTime = 0;
    await mediaControls.stop();
    await updateMediaSession();
    
    if (interval) {
      clearInterval(interval);
      interval = null;
    }
  }
  
  async function next() {
    if (shuffle) {
      currentTrackIndex = Math.floor(Math.random() * tracks.length);
    } else {
      currentTrackIndex = (currentTrackIndex + 1) % tracks.length;
    }
    currentTime = 0;
    
    await mediaControls.next();
    await updateMediaSession();
    
    if (isPlaying) {
      await play();
    }
  }
  
  async function previous() {
    if (currentTime > 3) {
      currentTime = 0;
    } else {
      currentTrackIndex = (currentTrackIndex - 1 + tracks.length) % tracks.length;
      currentTime = 0;
    }
    
    await mediaControls.previous();
    await updateMediaSession();
    
    if (isPlaying) {
      await play();
    }
  }
  
  async function seek(event) {
    const rect = event.currentTarget.getBoundingClientRect();
    const x = event.clientX - rect.left;
    const percentage = x / rect.width;
    currentTime = Math.floor(currentTrack.duration * percentage);
    
    if (initialized) {
      await mediaControls.setPosition(currentTime);
      await updateMediaSession();
    }
  }
  
  async function toggleShuffle() {
    shuffle = !shuffle;
    await updateMediaSession();
  }
  
  async function cycleRepeat() {
    const modes = [RepeatMode.None, RepeatMode.List, RepeatMode.Track];
    const currentIndex = modes.indexOf(repeatMode);
    repeatMode = modes[(currentIndex + 1) % modes.length];
    await updateMediaSession();
  }
  
  async function selectTrack(index) {
    currentTrackIndex = index;
    currentTrack = tracks[index];
    currentTime = 0;
    
    await updateMediaSession();
    if (isPlaying) {
      await play();
    }
  }
  
  function formatTime(seconds) {
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  }
</script>

<div class="player-container">
  <h1>🎵 Media Player</h1>
  <p class="subtitle">Full-featured media player demonstration</p>
  
  {#if !initialized}
    <div class="warning-card">
      ⚠️ Media controls not initialized. Please go to Overview page first.
    </div>
  {/if}
  
  <div class="player">
    <div class="player-main">
      <div class="artwork-container">
        <img 
          src={currentTrack.artwork} 
          alt="Album artwork"
          class="artwork"
          class:spinning={isPlaying}
        />
      </div>
      
      <div class="track-info">
        <h2>{currentTrack.title}</h2>
        <p class="artist">{currentTrack.artist}</p>
        <p class="album">{currentTrack.album}</p>
      </div>
      
      <div class="progress-container">
        <span class="time">{formatTime(currentTime)}</span>
        <div 
          class="progress-bar"
          on:click={seek}
          on:keydown={(e) => {
            if (e.key === 'ArrowLeft') {
              currentTime = Math.max(0, currentTime - 5);
              if (initialized) mediaControls.updatePosition(currentTime);
            } else if (e.key === 'ArrowRight') {
              currentTime = Math.min(currentTrack.duration, currentTime + 5);
              if (initialized) mediaControls.updatePosition(currentTime);
            }
          }}
          role="slider"
          tabindex="0"
          aria-label="Seek"
          aria-valuenow={currentTime}
          aria-valuemin="0"
          aria-valuemax={currentTrack.duration}
        >
          <div class="progress-fill" style="width: {progress}%"></div>
        </div>
        <span class="time">{formatTime(currentTrack.duration)}</span>
      </div>
      
      <div class="controls">
        <button 
          class="control-btn small"
          class:active={shuffle}
          on:click={toggleShuffle}
          title="Shuffle"
        >
          🔀
        </button>
        
        <button 
          class="control-btn"
          on:click={previous}
          title="Previous"
        >
          ⏮️
        </button>
        
        <button 
          class="control-btn large"
          on:click={togglePlayPause}
          title={isPlaying ? 'Pause' : 'Play'}
        >
          {isPlaying ? '⏸️' : '▶️'}
        </button>
        
        <button 
          class="control-btn"
          on:click={next}
          title="Next"
        >
          ⏭️
        </button>
        
        <button 
          class="control-btn small"
          class:active={repeatMode !== RepeatMode.None}
          on:click={cycleRepeat}
          title="Repeat"
        >
          {#if repeatMode === RepeatMode.Track}
            🔂
          {:else if repeatMode === RepeatMode.List}
            🔁
          {:else}
            🔁
          {/if}
        </button>
      </div>
      
      <div class="volume-control">
        <span>🔊</span>
        <input 
          type="range"
          min="0"
          max="100"
          bind:value={volume}
          class="volume-slider"
        />
        <span>{volume}%</span>
      </div>
    </div>
    
    <div class="playlist">
      <h3>Playlist</h3>
      <ul>
        {#each tracks as track, index}
          <li>
            <button
              class="playlist-item"
              class:active={index === currentTrackIndex}
              on:click={() => selectTrack(index)}
            >
              <img src={track.artwork} alt="Track artwork" />
              <div class="track-details">
                <span class="track-title">{track.title}</span>
                <span class="track-artist">{track.artist}</span>
              </div>
              <span class="track-duration">{formatTime(track.duration)}</span>
            </button>
          </li>
        {/each}
      </ul>
    </div>
  </div>
</div>

<style>
  .player-container {
    padding: 2rem;
    max-width: 1200px;
    margin: 0 auto;
  }
  
  h1 {
    margin-bottom: 0.5rem;
  }
  
  .subtitle {
    color: #a0a0a0;
    margin-bottom: 2rem;
  }
  
  .warning-card {
    background: #fef3c7;
    color: #92400e;
    padding: 1rem;
    border-radius: 8px;
    margin-bottom: 2rem;
  }
  
  .player {
    display: grid;
    grid-template-columns: 1fr 400px;
    gap: 2rem;
  }
  
  .player-main {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 16px;
    padding: 2rem;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
    max-width: 600px;
    margin: 0 auto 2rem;
  }
  
  .artwork-container {
    text-align: center;
    margin-bottom: 2rem;
  }
  
  .artwork {
    width: 300px;
    height: 300px;
    border-radius: 16px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
    transition: transform 0.3s ease;
  }
  
  .artwork.spinning {
    animation: spin 20s linear infinite;
  }
  
  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }
  
  .track-info {
    text-align: center;
    margin-bottom: 2rem;
  }
  
  .track-info h2 {
    color: #fff;
  }
  
  .track-info h2 {
    font-size: 1.8rem;
    margin-bottom: 0.5rem;
  }
  
  .artist {
    font-size: 1.1rem;
    color: #a0a0a0;
    margin-bottom: 0.25rem;
  }
  
  .album {
    font-size: 0.9rem;
    color: #808080;
  }
  
  .progress-container {
    display: flex;
    align-items: center;
    gap: 1rem;
    margin-bottom: 2rem;
  }
  
  .time {
    font-size: 0.9rem;
  }
  
  .progress-bar {
    background: rgba(255, 255, 255, 0.1);
    height: 6px;
    border-radius: 3px;
    overflow: hidden;
    cursor: pointer;
    margin: 1.5rem 0;
  }
  
  .progress-fill {
    height: 100%;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    border-radius: 3px;
    transition: width 0.3s ease;
  }
  
  .controls {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 1rem;
    margin-bottom: 2rem;
  }
  
  .control-btn {
    background: white;
    border: 2px solid #e5e7eb;
    border-radius: 50%;
    cursor: pointer;
    transition: all 0.3s ease;
    font-size: 1.5rem;
    width: 60px;
    height: 60px;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  
  .control-btn.large {
    width: 80px;
    height: 80px;
    font-size: 2rem;
  }
  
  .control-btn.small {
    width: 50px;
    height: 50px;
    font-size: 1.2rem;
  }
  
  .control-btn:hover {
    background: rgba(255, 255, 255, 0.08);
    transform: scale(1.05);
  }
  
  .control-btn.active {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    border-color: transparent;
  }
  
  .volume-control {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 1rem;
  }
  
  .volume-slider {
    width: 200px;
  }
  
  .playlist {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 16px;
    padding: 1.5rem;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
  }
  
  .playlist h3 {
    margin-bottom: 1rem;
  }
  
  .playlist ul {
    list-style: none;
    padding: 0;
    margin: 0;
  }
  
  .playlist-item {
    display: flex;
    align-items: center;
    padding: 0.75rem;
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid rgba(255, 255, 255, 0.05);
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.3s ease;
    text-align: left;
  }
  
  .playlist-item:hover {
    background: rgba(255, 255, 255, 0.08);
    transform: translateX(4px);
  }
  
  .playlist-item.active {
    background: linear-gradient(135deg, rgba(102, 126, 234, 0.15) 0%, rgba(118, 75, 162, 0.15) 100%);
    border-left: 3px solid #667eea;
  }
  
  .playlist-item img {
    width: 48px;
    height: 48px;
    border-radius: 4px;
  }
  
  .track-details {
    flex: 1;
    display: flex;
    flex-direction: column;
  }
  
  .track-title {
    font-weight: 500;
  }
  
  .track-artist {
    font-size: 0.9rem;
    color: #666;
  }
  
  .track-duration {
    color: #999;
    font-size: 0.9rem;
  }
</style>
