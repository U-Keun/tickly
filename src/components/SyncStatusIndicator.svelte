<script lang="ts">
  import { syncStore, authStore } from '$lib/stores';

  let { size = 'small' }: { size?: 'small' | 'medium' } = $props();

  $effect(() => {
    if (authStore.isLoggedIn) {
      syncStore.loadStatus();
    }
  });

  const iconSize = size === 'small' ? 'w-4 h-4' : 'w-5 h-5';
</script>

{#if authStore.isLoggedIn && syncStore.isEnabled}
  <div class="sync-indicator" class:syncing={syncStore.isSyncing}>
    {#if syncStore.isSyncing}
      <svg class="{iconSize} spin" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
      </svg>
    {:else if syncStore.pendingCount > 0}
      <div class="pending-badge">
        <svg class={iconSize} fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 15a4 4 0 004 4h9a5 5 0 10-.1-9.999 5.002 5.002 0 10-9.78 2.096A4.001 4.001 0 003 15z" />
        </svg>
        <span class="badge-count">{syncStore.pendingCount}</span>
      </div>
    {:else}
      <svg class="{iconSize} synced" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 15a4 4 0 004 4h9a5 5 0 10-.1-9.999 5.002 5.002 0 10-9.78 2.096A4.001 4.001 0 003 15z" />
      </svg>
    {/if}
  </div>
{/if}

<style>
  .sync-indicator {
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--color-ink-muted);
  }

  .sync-indicator.syncing {
    color: var(--color-accent-sky-strong);
  }

  .synced {
    color: var(--color-accent-mint-strong);
  }

  .spin {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .pending-badge {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--color-accent-peach-strong);
  }

  .badge-count {
    position: absolute;
    top: -6px;
    right: -8px;
    min-width: 14px;
    height: 14px;
    padding: 0 4px;
    background: var(--color-accent-peach-strong);
    color: var(--color-white);
    font-size: 10px;
    font-weight: 600;
    border-radius: 7px;
    display: flex;
    align-items: center;
    justify-content: center;
  }
</style>
