<script>
  import { currentPage } from './lib/stores.js';
  import Navigation from './components/Navigation.svelte';
  import Notifications from './components/Notifications.svelte';
  
  // Import pages
  import Overview from './pages/Overview.svelte';
  import MediaPlayer from './pages/MediaPlayer.svelte';
  import SetMethods from './pages/SetMethods.svelte';
  import GetMethods from './pages/GetMethods.svelte';
  import SystemMonitor from './pages/SystemMonitor.svelte';
  import Events from './pages/Events.svelte';

  // Page components mapping
  const pages = {
    'overview': Overview,
    'player': MediaPlayer,
    'set-methods': SetMethods,
    'get-methods': GetMethods,
    'system-monitor': SystemMonitor,
    'events': Events
  };
  
  $: PageComponent = pages[$currentPage] || Overview;
</script>

<div class="app">
  <Navigation />
  <main class="main-content">
    <svelte:component this={PageComponent} />
  </main>
  <Notifications />
</div>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
    background: linear-gradient(135deg, #1e1e2e 0%, #2d2d44 100%);
    color: #fff;
    min-height: 100vh;
  }
  
  :global(*) {
    box-sizing: border-box;
  }
  
  .app {
    display: flex;
    height: 100vh;
    overflow: hidden;
  }
  
  .main-content {
    flex: 1;
    overflow-y: auto;
    background: linear-gradient(135deg, #1e1e2e 0%, #2d2d44 100%);
  }
</style>
