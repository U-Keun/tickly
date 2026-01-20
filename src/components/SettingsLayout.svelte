<script lang="ts">
  import type { Snippet } from 'svelte';
  import { i18n } from '$lib/i18n';

  interface Props {
    title: string;
    onBack: () => void;
    children: Snippet;
    footer?: Snippet;
    contentClass?: string;
  }

  let {
    title,
    onBack,
    children,
    footer,
    contentClass = '',
  }: Props = $props();
</script>

<div class="settings-container bg-paper">
  <!-- Header -->
  <header class="settings-header">
    <button class="back-btn" onclick={onBack} aria-label={i18n.t('back')}>
      <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
      </svg>
    </button>
    <h1 class="header-title">{title}</h1>
    <div class="w-6"></div>
  </header>

  <!-- Content -->
  <div class="settings-content {contentClass}">
    {@render children()}
  </div>

  <!-- Footer (optional) -->
  {#if footer}
    {@render footer()}
  {/if}
</div>

<style>
  .settings-container {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    display: flex;
    flex-direction: column;
    padding-top: env(safe-area-inset-top, 0);
    padding-bottom: env(safe-area-inset-bottom, 0);
    overflow: hidden;
  }

  .settings-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 16px;
    border-bottom: 1px solid var(--color-stroke);
    background: var(--color-paper);
    position: sticky;
    top: 0;
    z-index: 10;
  }

  .back-btn {
    padding: 8px;
    color: var(--color-ink);
    background: none;
    border: none;
    cursor: pointer;
    border-radius: 8px;
  }

  .back-btn:hover {
    background: var(--color-canvas);
  }

  .header-title {
    font-size: 17px;
    font-weight: 600;
    color: var(--color-ink);
  }

  .settings-content {
    flex: 1;
    overflow-y: auto;
    -webkit-overflow-scrolling: touch;
    padding: 16px;
    min-height: 0;
  }
</style>
