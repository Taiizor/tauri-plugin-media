<script>
  import { 
    mediaControls, 
    PlaybackStatus, 
    RepeatMode,
    getMetadata,
    getPlaybackInfo,
    getPlaybackStatus,
    getPosition,
    isEnabled
  } from 'tauri-plugin-media-api'
  import { onMount } from 'svelte'

  let initialized = $state(false)
  let currentSong = $state({
    title: '',
    artist: '',
    album: '',
    duration: 0,
    artwork: '/tauri.svg'
  })
  
  let playbackState = $state({
    status: PlaybackStatus.Stopped,
    position: 0,
    shuffle: false,
    repeatMode: RepeatMode.None,
    playbackRate: 1.0
  })
  
  let currentTime = $state(0)
  let intervalId = null
  
  // State from system
  let systemState = $state({
    metadata: null,
    playbackInfo: null,
    status: null,
    position: 0,
    enabled: false
  })

  // Demo songs
  const songs = [
    {
      title: 'Demo Song 1',
      artist: 'Demo Artist',
      album: 'Demo Album',
      duration: 180,
      artwork: '/tauri.svg'
    },
    {
      title: 'Demo Song 2', 
      artist: 'Another Artist',
      album: 'Another Album',
      duration: 240,
      artwork: '/vite.svg'
    },
    {
      title: 'Demo Song 3',
      artist: 'Third Artist',
      album: 'Third Album',
      duration: 200,
      artwork: '/svelte.svg'
    }
  ]
  
  let currentSongIndex = $state(0)

  onMount(async () => {
    try {
      await mediaControls.initialize('tauri-media-demo', 'Tauri Media Demo')
      initialized = true
      console.log('Media controls initialized')
    } catch (error) {
      console.error('Failed to initialize media controls:', error)
    }
  })

  async function loadSong(index) {
    currentSongIndex = index
    currentSong = songs[index]
    currentTime = 0
    
    if (initialized) {
      await mediaControls.updateNowPlaying(
        {
          title: currentSong.title,
          artist: currentSong.artist,
          album: currentSong.album,
          duration: currentSong.duration,
          artworkUrl: currentSong.artwork
        },
        {
          status: PlaybackStatus.Stopped,
          position: 0,
          shuffle: playbackState.shuffle,
          repeatMode: playbackState.repeatMode,
          playbackRate: playbackState.playbackRate
        }
      )
    }
  }

  async function play() {
    if (!currentSong.title) {
      await loadSong(0)
    }
    
    playbackState.status = PlaybackStatus.Playing
    
    if (initialized) {
      await mediaControls.play()
    }
    
    // Simulate playback progress
    if (intervalId) clearInterval(intervalId)
    intervalId = setInterval(() => {
      if (currentTime < currentSong.duration) {
        currentTime += 0.1
        if (initialized && Math.floor(currentTime) % 5 === 0) {
          mediaControls.updatePosition(currentTime)
        }
      } else {
        next()
      }
    }, 100)
  }

  async function pause() {
    playbackState.status = PlaybackStatus.Paused
    
    if (initialized) {
      await mediaControls.pause()
    }
    
    if (intervalId) {
      clearInterval(intervalId)
      intervalId = null
    }
  }

  async function stop() {
    playbackState.status = PlaybackStatus.Stopped
    currentTime = 0
    
    if (initialized) {
      await mediaControls.stop()
    }
    
    if (intervalId) {
      clearInterval(intervalId)
      intervalId = null
    }
  }

  async function next() {
    let nextIndex = currentSongIndex + 1
    if (nextIndex >= songs.length) {
      if (playbackState.repeatMode === RepeatMode.List) {
        nextIndex = 0
      } else {
        await stop()
        return
      }
    }
    
    await loadSong(nextIndex)
    if (playbackState.status === PlaybackStatus.Playing) {
      await play()
    }
  }

  async function previous() {
    let prevIndex = currentSongIndex - 1
    if (prevIndex < 0) {
      if (playbackState.repeatMode === RepeatMode.List) {
        prevIndex = songs.length - 1
      } else {
        prevIndex = 0
      }
    }
    
    await loadSong(prevIndex)
    if (playbackState.status === PlaybackStatus.Playing) {
      await play()
    }
  }

  function toggleShuffle() {
    playbackState.shuffle = !playbackState.shuffle
    updatePlaybackInfo()
  }

  function cycleRepeatMode() {
    const modes = [RepeatMode.None, RepeatMode.Track, RepeatMode.List]
    const currentIndex = modes.indexOf(playbackState.repeatMode)
    playbackState.repeatMode = modes[(currentIndex + 1) % modes.length]
    updatePlaybackInfo()
  }

  async function updatePlaybackInfo() {
    if (initialized && currentSong.title) {
      await mediaControls.updateNowPlaying(
        {
          title: currentSong.title,
          artist: currentSong.artist,
          album: currentSong.album,
          duration: currentSong.duration,
          artworkUrl: currentSong.artwork
        },
        playbackState
      )
    }
  }

  function formatTime(seconds) {
    const mins = Math.floor(seconds / 60)
    const secs = Math.floor(seconds % 60)
    return `${mins}:${secs.toString().padStart(2, '0')}`
  }

  function seek(event) {
    const rect = event.currentTarget.getBoundingClientRect()
    const x = event.clientX - rect.left
    const percentage = x / rect.width
    currentTime = currentSong.duration * percentage
    
    if (initialized) {
      mediaControls.updatePosition(currentTime)
    }
  }
  
  async function fetchSystemState() {
    if (!initialized) return
    
    try {
      systemState.metadata = await getMetadata()
      systemState.playbackInfo = await getPlaybackInfo()
      systemState.status = await getPlaybackStatus()
      systemState.position = await getPosition()
      systemState.enabled = await isEnabled()
    } catch (error) {
      console.error('Failed to fetch system state:', error)
    }
  }
  
  // Fetch system state periodically
  onMount(() => {
    const stateInterval = setInterval(() => {
      fetchSystemState()
    }, 1000)
    
    return () => clearInterval(stateInterval)
  })
</script>

<main class="container">
  <h1>Tauri Media Controls Demo</h1>

  <div class="player">
    <div class="album-art">
      <img src={currentSong.artwork} alt="Album Art" />
    </div>

    <div class="song-info">
      <h2>{currentSong.title || 'No Song Selected'}</h2>
      <p>{currentSong.artist || 'Unknown Artist'} - {currentSong.album || 'Unknown Album'}</p>
    </div>

    <div 
      class="progress-bar" 
      role="slider"
      tabindex="0"
      aria-label="Seek slider"
      aria-valuenow={currentTime}
      aria-valuemin="0"
      aria-valuemax={currentSong.duration}
      onclick={seek}
      onkeydown={(e) => {
        if (e.key === 'ArrowLeft') {
          currentTime = Math.max(0, currentTime - 5)
          if (initialized) mediaControls.updatePosition(currentTime)
        } else if (e.key === 'ArrowRight') {
          currentTime = Math.min(currentSong.duration, currentTime + 5)
          if (initialized) mediaControls.updatePosition(currentTime)
        }
      }}
    >
      <div class="progress" style="width: {(currentTime / currentSong.duration) * 100}%"></div>
      <div class="time-info">
        <span>{formatTime(currentTime)}</span>
        <span>{formatTime(currentSong.duration)}</span>
      </div>
    </div>

    <div class="controls">
      <button onclick={toggleShuffle} class:active={playbackState.shuffle}>
        🔀 Shuffle
      </button>
      <button onclick={previous}>⏮️ Previous</button>
      
      {#if playbackState.status === PlaybackStatus.Playing}
        <button onclick={pause} class="play-pause">⏸️ Pause</button>
      {:else}
        <button onclick={play} class="play-pause">▶️ Play</button>
      {/if}
      
      <button onclick={stop}>⏹️ Stop</button>
      <button onclick={next}>⏭️ Next</button>
      <button onclick={cycleRepeatMode} class:active={playbackState.repeatMode !== RepeatMode.None}>
        {#if playbackState.repeatMode === RepeatMode.None}
          🔁 Repeat Off
        {:else if playbackState.repeatMode === RepeatMode.Track}
          🔂 Repeat One
        {:else}
          🔁 Repeat All
        {/if}
      </button>
    </div>

    <div class="playlist">
      <h3>Playlist</h3>
      <ul>
        {#each songs as song, index}
          <li>
            <button 
              onclick={() => loadSong(index)} 
              class:active={currentSongIndex === index}
            >
              {song.title} - {song.artist}
            </button>
          </li>
        {/each}
      </ul>
    </div>

    <div class="status">
      <p>Media Controls: {initialized ? '✅ Initialized' : '❌ Not Initialized'}</p>
      <p>Status: {playbackState.status}</p>
    </div>
    
    <div class="system-state">
      <h3>System State (Get Methods)</h3>
      <div class="state-grid">
        <div class="state-item">
          <strong>Enabled:</strong> {systemState.enabled ? '✅ Yes' : '❌ No'}
        </div>
        <div class="state-item">
          <strong>Status:</strong> {systemState.status || 'N/A'}
        </div>
        <div class="state-item">
          <strong>Position:</strong> {formatTime(systemState.position)}
        </div>
        <div class="state-item">
          <strong>Metadata:</strong>
          {#if systemState.metadata}
            <div class="metadata-info">
              <small>Title: {systemState.metadata.title || 'N/A'}</small>
              <small>Artist: {systemState.metadata.artist || 'N/A'}</small>
              <small>Album: {systemState.metadata.album || 'N/A'}</small>
              {#if systemState.metadata.artworkData}
                <img 
                  src="data:image/jpeg;base64,{systemState.metadata.artworkData}" 
                  alt="Artwork" 
                  class="system-artwork"
                />
              {:else if systemState.metadata.artworkUrl}
                <img 
                  src={systemState.metadata.artworkUrl} 
                  alt="Artwork" 
                  class="system-artwork"
                />
              {/if}
            </div>
          {:else}
            <small>No metadata</small>
          {/if}
        </div>
        <div class="state-item">
          <strong>Playback Info:</strong>
          {#if systemState.playbackInfo}
            <div class="metadata-info">
              <small>Status: {systemState.playbackInfo.status || 'N/A'}</small>
              <small>Shuffle: {systemState.playbackInfo.shuffle ? 'On' : 'Off'}</small>
              <small>Repeat: {systemState.playbackInfo.repeatMode || 'None'}</small>
            </div>
          {:else}
            <small>No playback info</small>
          {/if}
        </div>
      </div>
      <button onclick={fetchSystemState} class="refresh-btn">
        🔄 Refresh System State
      </button>
    </div>
  </div>
</main>

<style>
  .player {
    max-width: 600px;
    margin: 2rem auto;
    padding: 2rem;
    background: rgba(255, 255, 255, 0.05);
    border-radius: 12px;
  }

  .album-art {
    text-align: center;
    margin-bottom: 1rem;
  }

  .album-art img {
    width: 200px;
    height: 200px;
    border-radius: 8px;
    object-fit: cover;
  }

  .song-info {
    text-align: center;
    margin-bottom: 1.5rem;
  }

  .song-info h2 {
    margin: 0.5rem 0;
    font-size: 1.5rem;
  }

  .song-info p {
    margin: 0.25rem 0;
    color: #888;
  }

  .progress-bar {
    position: relative;
    height: 6px;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 3px;
    margin: 1.5rem 0;
    cursor: pointer;
  }

  .progress {
    height: 100%;
    background: #1db954;
    border-radius: 3px;
    transition: width 0.1s;
  }

  .time-info {
    display: flex;
    justify-content: space-between;
    margin-top: 0.5rem;
    font-size: 0.875rem;
    color: #888;
  }

  .controls {
    display: flex;
    justify-content: center;
    gap: 0.5rem;
    flex-wrap: wrap;
    margin: 1.5rem 0;
  }

  .controls button {
    padding: 0.5rem 1rem;
    font-size: 1rem;
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 4px;
    color: white;
    cursor: pointer;
    transition: all 0.2s;
  }

  .controls button:hover {
    background: rgba(255, 255, 255, 0.2);
  }

  .controls button.active {
    background: #1db954;
    border-color: #1db954;
  }

  .controls button.play-pause {
    padding: 0.5rem 1.5rem;
    background: #1db954;
    border-color: #1db954;
  }

  .playlist {
    margin-top: 2rem;
    padding-top: 2rem;
    border-top: 1px solid rgba(255, 255, 255, 0.1);
  }

  .playlist h3 {
    margin-bottom: 1rem;
  }

  .playlist ul {
    list-style: none;
    padding: 0;
  }

  .playlist li button {
    width: 100%;
    padding: 0.75rem;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid transparent;
    color: white;
    text-align: left;
    cursor: pointer;
    transition: all 0.2s;
  }

  .playlist li button:hover {
    background: rgba(255, 255, 255, 0.1);
  }

  .playlist li button.active {
    background: rgba(29, 185, 84, 0.2);
    border-color: #1db954;
  }

  .status {
    margin-top: 2rem;
    padding: 1rem;
    background: rgba(0, 0, 0, 0.2);
    border-radius: 4px;
    text-align: center;
  }

  .status p {
    margin: 0.25rem 0;
    font-size: 0.875rem;
  }
  
  .system-state {
    margin-top: 2rem;
    padding: 1.5rem;
    background: rgba(0, 0, 0, 0.3);
    border-radius: 8px;
    border: 1px solid rgba(255, 255, 255, 0.1);
  }
  
  .system-state h3 {
    margin-bottom: 1rem;
    color: #1db954;
  }
  
  .state-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
    margin-bottom: 1rem;
  }
  
  .state-item {
    padding: 0.75rem;
    background: rgba(255, 255, 255, 0.05);
    border-radius: 4px;
    border: 1px solid rgba(255, 255, 255, 0.1);
  }
  
  .state-item strong {
    display: block;
    margin-bottom: 0.5rem;
    color: #1db954;
  }
  
  .metadata-info {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    margin-top: 0.25rem;
  }
  
  .metadata-info small {
    color: #aaa;
  }
  
  .refresh-btn {
    padding: 0.5rem 1rem;
    background: #1db954;
    border: none;
    border-radius: 4px;
    color: white;
    cursor: pointer;
    transition: background 0.2s;
  }
  
  .refresh-btn:hover {
    background: #1ed760;
  }
  
  .system-artwork {
    width: 60px;
    height: 60px;
    object-fit: cover;
    border-radius: 4px;
    margin-top: 0.5rem;
    border: 1px solid rgba(255, 255, 255, 0.2);
  }
</style>
