<script>
  import { currentPage } from '../lib/stores.js';
  
  const pages = [
    { id: 'overview', name: 'Overview', icon: '🏠', description: 'Plugin overview and capabilities' },
    { id: 'player', name: 'Media Player', icon: '🎵', description: 'Full-featured media player example' },
    { id: 'set-methods', name: 'Set Methods', icon: '📝', description: 'Test all set methods' },
    { id: 'get-methods', name: 'Get Methods', icon: '📖', description: 'Test all get methods' },
    { id: 'system-monitor', name: 'System Monitor', icon: '📊', description: 'Monitor system-wide media' },
    { id: 'events', name: 'Events', icon: '⚡', description: 'Media control events' }
  ];
  
  function selectPage(pageId) {
    currentPage.set(pageId);
  }
</script>

<nav class="navigation">
  <div class="nav-header">
    <h2>🎮 Tauri Media Plugin</h2>
    <span class="version">v0.1.0</span>
  </div>
  
  <ul class="nav-list">
    {#each pages as page}
      <li>
        <button 
          class="nav-item"
          class:active={$currentPage === page.id}
          on:click={() => selectPage(page.id)}
        >
          <span class="nav-icon">{page.icon}</span>
          <div class="nav-content">
            <span class="nav-name">{page.name}</span>
            <span class="nav-description">{page.description}</span>
          </div>
        </button>
      </li>
    {/each}
  </ul>
</nav>

<style>
  .navigation {
    width: 280px;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    height: 100vh;
    overflow-y: auto;
    box-shadow: 2px 0 10px rgba(0, 0, 0, 0.1);
  }
  
  .nav-header {
    padding: 2rem;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  
  .nav-header h2 {
    margin: 0;
    font-size: 1.5rem;
    font-weight: 600;
  }
  
  .version {
    background: rgba(255, 255, 255, 0.2);
    padding: 0.25rem 0.5rem;
    border-radius: 12px;
    font-size: 0.75rem;
  }
  
  .nav-list {
    list-style: none;
    padding: 1rem;
    margin: 0;
  }
  
  .nav-item {
    width: 100%;
    background: transparent;
    border: none;
    color: white;
    padding: 1rem;
    margin-bottom: 0.5rem;
    border-radius: 12px;
    cursor: pointer;
    transition: all 0.3s ease;
    display: flex;
    align-items: center;
    gap: 1rem;
    text-align: left;
  }
  
  .nav-item:hover {
    background: rgba(255, 255, 255, 0.1);
    transform: translateX(5px);
  }
  
  .nav-item.active {
    background: rgba(255, 255, 255, 0.2);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }
  
  .nav-icon {
    font-size: 1.5rem;
    width: 40px;
    text-align: center;
  }
  
  .nav-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }
  
  .nav-name {
    font-weight: 500;
    font-size: 1rem;
  }
  
  .nav-description {
    font-size: 0.75rem;
    opacity: 0.8;
  }
</style>
