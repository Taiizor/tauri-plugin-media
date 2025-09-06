<script>
  import { notifications } from '../lib/stores.js';
  import { fade, fly } from 'svelte/transition';
  
  function getIcon(type) {
    switch(type) {
      case 'success': return '✅';
      case 'error': return '❌';
      case 'warning': return '⚠️';
      default: return 'ℹ️';
    }
  }
  
  function getColor(type) {
    switch(type) {
      case 'success': return '#10b981';
      case 'error': return '#ef4444';
      case 'warning': return '#f59e0b';
      default: return '#3b82f6';
    }
  }
</script>

<div class="notifications-container">
  {#each $notifications as notification (notification.id)}
    <div 
      class="notification"
      style="border-left-color: {getColor(notification.type)}"
      transition:fly="{{ y: -20, duration: 300 }}"
    >
      <span class="notification-icon">{getIcon(notification.type)}</span>
      <span class="notification-message">{notification.message}</span>
    </div>
  {/each}
</div>

<style>
  .notifications-container {
    position: fixed;
    top: 20px;
    right: 20px;
    z-index: 1000;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    max-width: 400px;
  }
  
  .notification {
    background: white;
    padding: 1rem 1.25rem;
    border-radius: 8px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
    display: flex;
    align-items: center;
    gap: 0.75rem;
    border-left: 3px solid;
    animation: slideIn 0.3s ease;
  }
  
  @keyframes slideIn {
    from {
      transform: translateX(100%);
      opacity: 0;
    }
    to {
      transform: translateX(0);
      opacity: 1;
    }
  }
  
  .notification-icon {
    font-size: 1.25rem;
  }
  
  .notification-message {
    flex: 1;
    font-size: 0.95rem;
    color: #333;
  }
</style>
