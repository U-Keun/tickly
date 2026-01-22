<script lang="ts">
  import { goto } from '$app/navigation';
  import { i18n } from '$lib/i18n';

  let {
    onReorder,
    onHome,
  }: {
    onReorder: () => void;
    onHome: () => void;
  } = $props();
</script>

<nav class="fixed bottom-0 left-0 right-0 z-20" style="background: var(--color-paper); border-top: 1px solid var(--color-border); padding-bottom: env(safe-area-inset-bottom, 0);">
  <div class="flex justify-around items-center h-14">
    <!-- Reorder Button -->
    <button
      class="flex flex-col items-center justify-center flex-1 h-full"
      style="color: var(--color-ink-muted);"
      title={i18n.t('reorder')}
      onclick={onReorder}
    >
      <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
      </svg>
    </button>

    <!-- Home Button -->
    <button
      class="flex flex-col items-center justify-center flex-1 h-full"
      style="color: var(--color-ink-muted);"
      title={i18n.t('home')}
      onclick={onHome}
    >
      <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
      </svg>
    </button>

    <!-- Settings Button -->
    <button
      class="flex flex-col items-center justify-center flex-1 h-full"
      style="color: var(--color-ink-muted);"
      title={i18n.t('settings')}
      onclick={() => goto('/settings')}
    >
      <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
      </svg>
    </button>
  </div>
</nav>
