<script>
  import { mediaControls, MediaControlEventType } from 'tauri-plugin-media-api';
  import { addNotification, pluginInitialized } from '../lib/stores.js';
  import { onMount, onDestroy } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  
  let events = [];
  let listening = false;
  let eventHandler = null;
  let maxEvents = 50;
  let autoScroll = true;
  let initialized = false;
  let unlistenFn = null;
  
  pluginInitialized.subscribe(value => {
    initialized = value;
  });
  
  onMount(async () => {
    if (initialized) {
      addNotification('Ready to listen for events', 'info');
    }
  });
  
  onDestroy(() => {
    if (listening) {
      stopListening();
    }
  });
  
  async function startListening() {
    if (!initialized) {
      addNotification('Please initialize the plugin first', 'warning');
      return;
    }
    
    if (unlistenFn) return;
    
    try {
      // Listen for media control events from the Rust backend
      unlistenFn = await listen('media-control-event', (event) => {
        const eventEntry = {
          id: Date.now(),
          type: event.payload?.type || 'unknown',
          timestamp: new Date(),
          data: event.payload,
          isReal: true
        };
        
        events = [eventEntry, ...events].slice(0, maxEvents);
        addNotification(`Real Event: ${eventEntry.type}`, 'success');
      });
      
      listening = true;
      addNotification('Started listening for real media events', 'success');
    } catch (error) {
      addNotification(`Failed to start listening: ${error}`, 'error');
    }
  }
  
  function stopListening() {
    if (unlistenFn) {
      unlistenFn();
      unlistenFn = null;
    }
    
    listening = false;
    addNotification('Stopped listening for events', 'info');
  }
  
  function clearEvents() {
    events = [];
    addNotification('Event log cleared', 'info');
  }
  
  function simulateEvent(type) {
    const eventEntry = {
      id: Date.now(),
      type,
      timestamp: new Date(),
      data: { simulated: true },
      isReal: false
    };
    
    events = [eventEntry, ...events].slice(0, maxEvents);
    addNotification(`Simulated: ${type}`, 'warning');
  }
  
  function formatTimestamp(date) {
    return date.toLocaleTimeString('en-US', { 
      hour12: false,
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
      fractionalSecondDigits: 3
    });
  }
  
  function getEventIcon(type) {
    const icons = {
      play: '▶️',
      pause: '⏸️',
      stop: '⏹️',
      next: '⏭️',
      previous: '⏮️',
      seek: '📍',
      position: '📊',
      shuffle: '🔀',
      repeat: '🔁'
    };
    return icons[type] || '📨';
  }
  
  function getEventColor(type) {
    const colors = {
      play: '#10b981',
      pause: '#f59e0b',
      stop: '#ef4444',
      next: '#3b82f6',
      previous: '#3b82f6',
      seek: '#8b5cf6',
      position: '#6366f1',
      shuffle: '#ec4899',
      repeat: '#14b8a6'
    };
    return colors[type] || '#6b7280';
  }
</script>

<div class="events-container">
  <h1>⚡ Media Control Events</h1>
  <p class="subtitle">Listen and respond to system media control events</p>
  
  <div class="control-panel">
    <div class="controls">
      <button 
        class="btn {listening ? 'btn-danger' : 'btn-success'}"
        on:click={() => listening ? stopListening() : startListening()}
      >
        {listening ? '⏹️ Stop Listening' : '▶️ Start Listening'}
      </button>
      
      <button class="btn btn-secondary" on:click={clearEvents}>
        🗑️ Clear Events
      </button>
      
      <label class="auto-scroll">
        <input type="checkbox" bind:checked={autoScroll} />
        Auto-scroll
      </label>
    </div>
    
    <div class="status">
      {#if listening}
        <span class="listening-indicator"></span>
        <span>Listening for events...</span>
      {:else}
        <span class="idle-indicator"></span>
        <span>Not listening</span>
      {/if}
    </div>
  </div>
  
  <div class="events-section">
    <div class="simulator">
      <h3>Event Simulator</h3>
      <p>Simulate media control events for testing:</p>
      
      <div class="simulator-grid">
        <button class="sim-btn" on:click={() => simulateEvent('play')}>
          ▶️ Play
        </button>
        <button class="sim-btn" on:click={() => simulateEvent('pause')}>
          ⏸️ Pause
        </button>
        <button class="sim-btn" on:click={() => simulateEvent('stop')}>
          ⏹️ Stop
        </button>
        <button class="sim-btn" on:click={() => simulateEvent('next')}>
          ⏭️ Next
        </button>
        <button class="sim-btn" on:click={() => simulateEvent('previous')}>
          ⏮️ Previous
        </button>
        <button class="sim-btn" on:click={() => simulateEvent('seek')}>
          📍 Seek
        </button>
        <button class="sim-btn" on:click={() => simulateEvent('shuffle')}>
          🔀 Shuffle
        </button>
        <button class="sim-btn" on:click={() => simulateEvent('repeat')}>
          🔁 Repeat
        </button>
      </div>
    </div>
    
    <div class="event-log">
      <div class="log-header">
        <h3>Event Log</h3>
        <span class="event-count">{events.length} events</span>
      </div>
      
      {#if events.length > 0}
        <div class="events-list" class:auto-scroll={autoScroll}>
          {#each events as event}
            <div 
              class="event-item"
              style="border-left-color: {getEventColor(event.type)}"
            >
              <span class="event-icon">{getEventIcon(event.type)}</span>
              <div class="event-details">
                <div class="event-type" style="color: {getEventColor(event.type)}">
                  {event.type.toUpperCase()}
                </div>
                <div class="event-time">{formatTimestamp(event.timestamp)}</div>
                {#if event.data.simulated}
                  <span class="simulated-badge">Simulated</span>
                {/if}
              </div>
              <div class="event-data">
                <pre>{JSON.stringify(event.data, null, 2)}</pre>
              </div>
            </div>
          {/each}
        </div>
      {:else}
        <div class="no-events">
          <span class="no-events-icon">📭</span>
          <p>No events yet. Start listening or simulate some events.</p>
        </div>
      {/if}
    </div>
  </div>
  
  <div class="info-cards">
    <div class="info-card">
      <h3>📱 System Events</h3>
      <p>
        When listening is active, the plugin will receive events from system media controls
        like keyboard media keys, lock screen controls, and notification controls.
      </p>
    </div>
    
    <div class="info-card">
      <h3>🎮 Event Types</h3>
      <ul>
        <li><strong>Play/Pause/Stop</strong> - Playback control events</li>
        <li><strong>Next/Previous</strong> - Track navigation events</li>
        <li><strong>Seek</strong> - Position change events</li>
        <li><strong>Shuffle/Repeat</strong> - Playback mode events</li>
      </ul>
    </div>
  </div>
</div>

<style>
  .events-container {
    padding: 2rem;
    max-width: 1200px;
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
  
  .controls {
    display: flex;
    gap: 1rem;
    align-items: center;
  }
  
  .auto-scroll {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-weight: 500;
  }
  
  .status {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-weight: 500;
  }
  
  .listening-indicator {
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
  
  .idle-indicator {
    width: 12px;
    height: 12px;
    background: #6b7280;
    border-radius: 50%;
  }
  
  .events-section {
    display: grid;
    grid-template-columns: 350px 1fr;
    gap: 2rem;
    margin-bottom: 2rem;
  }
  
  .simulator {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 12px;
    padding: 1.5rem;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.3);
    height: fit-content;
  }
  
  .simulator h3 {
    margin-bottom: 0.5rem;
  }
  
  .simulator p {
    color: #a0a0a0;
    margin-bottom: 1rem;
    font-size: 0.9rem;
  }
  
  .simulator-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 0.75rem;
  }
  
  .sim-btn {
    padding: 0.75rem;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.3s ease;
    font-size: 1rem;
    color: #fff;
  }
  
  .sim-btn:hover {
    background: linear-gradient(135deg, rgba(102, 126, 234, 0.15) 0%, rgba(118, 75, 162, 0.15) 100%);
    transform: translateY(-2px);
  }
  
  .event-log {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 12px;
    padding: 1.5rem;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.3);
    max-height: 600px;
    display: flex;
    flex-direction: column;
  }
  
  .log-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }
  
  .event-count {
    background: rgba(255, 255, 255, 0.1);
    padding: 0.25rem 0.75rem;
    border-radius: 12px;
    font-size: 0.875rem;
    color: #a0a0a0;
  }
  
  .events-list {
    flex: 1;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }
  
  .events-list.auto-scroll {
    scroll-behavior: smooth;
  }
  
  .event-item {
    display: grid;
    grid-template-columns: 40px 1fr auto;
    gap: 1rem;
    padding: 1rem;
    background: rgba(255, 255, 255, 0.03);
    border-radius: 8px;
    border-left: 3px solid;
    transition: all 0.3s ease;
  }
  
  .event-item:hover {
    background: rgba(255, 255, 255, 0.08);
    transform: translateX(2px);
  }
  
  .event-icon {
    font-size: 1.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  
  .event-details {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }
  
  .event-type {
    font-weight: 600;
    font-size: 0.9rem;
  }
  
  .event-time {
    font-size: 0.8rem;
    color: #a0a0a0;
  }
  
  .simulated-badge {
    background: #fbbf24;
    color: #78350f;
    padding: 0.125rem 0.5rem;
    border-radius: 4px;
    font-size: 0.75rem;
    font-weight: 500;
    width: fit-content;
  }
  
  .event-data {
    grid-column: 2 / -1;
    margin-top: 0.5rem;
  }
  
  .event-data pre {
    background: rgba(0, 0, 0, 0.3);
    padding: 0.5rem;
    border-radius: 4px;
    font-size: 0.75rem;
    overflow-x: auto;
    margin: 0;
    color: #fff;
  }
  
  .no-events {
    text-align: center;
    padding: 3rem;
    color: #666;
  }
  
  .no-events-icon {
    font-size: 3rem;
    display: block;
    margin-bottom: 1rem;
  }
  
  .info-cards {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 1.5rem;
  }
  
  .info-card {
    background: rgba(59, 130, 246, 0.1);
    border: 1px solid rgba(59, 130, 246, 0.3);
    border-radius: 12px;
    padding: 1.5rem;
  }
  
  .info-card h3 {
    color: #60a5fa;
    margin-bottom: 0.75rem;
  }
  
  .info-card p,
  .info-card ul {
    color: #93c5fd;
    line-height: 1.6;
  }
  
  .info-card ul {
    margin: 0;
    padding-left: 1.5rem;
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
  
  .btn-secondary {
    background: #6b7280;
    color: white;
  }
  
  .btn:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  }
</style>
