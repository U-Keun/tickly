<script lang="ts">
  import { fly } from 'svelte/transition';
  import { cubicOut } from 'svelte/easing';
  import { i18n } from '$lib/i18n';

  let {
    show,
    onAdd,
    onReset,
    onReorder,
    onStreak,
    onSettings,
  }: {
    show: boolean;
    onAdd: () => void;
    onReset: () => void;
    onReorder: () => void;
    onStreak: () => void;
    onSettings: () => void;
  } = $props();

  let menuOpen = $state(false);

  function toggleMenu() {
    menuOpen = !menuOpen;
  }

  function handleReorder() {
    menuOpen = false;
    onReorder();
  }

  function handleStreak() {
    menuOpen = false;
    onStreak();
  }

  function handleSettings() {
    menuOpen = false;
    onSettings();
  }
</script>

{#if show}
<div
  class="fixed right-6 flex flex-col gap-3 items-end z-10"
  style="bottom: 14px;"
  in:fly={{ y: 100, duration: 400, easing: cubicOut }}
  out:fly={{ y: 100, duration: 300, easing: cubicOut }}
>
  <!-- Add Button -->
  <button
    onclick={onAdd}
    class="w-14 h-14 bg-accent-sky-strong hover:bg-accent-sky text-white rounded-full shadow-lg flex items-center justify-center transition-all hover:scale-110"
    title={i18n.t('addItem')}
  >
    <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
    </svg>
  </button>

  <!-- Reset Button -->
  <button
    onclick={onReset}
    class="w-14 h-14 bg-accent-peach-strong hover:bg-accent-peach text-ink rounded-full shadow-lg flex items-center justify-center transition-all hover:scale-110"
    title={i18n.t('resetCheck')}
  >
    <svg class="w-7 h-7" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
    </svg>
  </button>

  <!-- Menu Row -->
  <div class="flex items-center gap-2">
    <!-- Reorder Button (leftmost) -->
    {#if menuOpen}
      <button
        onclick={handleReorder}
        class="w-12 h-12 bg-paper text-ink-muted rounded-full shadow-lg flex items-center justify-center transition-all hover:scale-110"
        style="border: 1px solid var(--color-border);"
        title={i18n.t('reorder')}
        in:fly={{ x: 50, duration: 200, delay: 100, easing: cubicOut }}
        out:fly={{ x: 50, duration: 200, delay: 100, easing: cubicOut }}
      >
        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
        </svg>
      </button>
    {/if}

    <!-- Streak Button (middle) -->
    {#if menuOpen}
      <button
        onclick={handleStreak}
        class="w-12 h-12 bg-paper text-ink-muted rounded-full shadow-lg flex items-center justify-center transition-all hover:scale-110"
        style="border: 1px solid var(--color-border);"
        title={i18n.t('streak')}
        in:fly={{ x: 50, duration: 200, delay: 50, easing: cubicOut }}
        out:fly={{ x: 50, duration: 200, delay: 50, easing: cubicOut }}
      >
        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 18.657A8 8 0 016.343 7.343S7 9 9 10c0-2 .5-5 2.986-7C14 5 16.09 5.777 17.656 7.343A7.975 7.975 0 0120 13a7.975 7.975 0 01-2.343 5.657z" />
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.879 16.121A3 3 0 1012.015 11L11 14H9c0 .768.293 1.536.879 2.121z" />
        </svg>
      </button>
    {/if}

    <!-- Settings Button (rightmost) -->
    {#if menuOpen}
      <button
        onclick={handleSettings}
        class="w-12 h-12 bg-paper text-ink-muted rounded-full shadow-lg flex items-center justify-center transition-all hover:scale-110"
        style="border: 1px solid var(--color-border);"
        title={i18n.t('settings')}
        in:fly={{ x: 50, duration: 200, easing: cubicOut }}
        out:fly={{ x: 50, duration: 200, easing: cubicOut }}
      >
        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
        </svg>
      </button>
    {/if}

    <!-- Menu Toggle Button -->
    <button
      onclick={toggleMenu}
      class="w-14 h-14 bg-accent-mint-strong hover:bg-accent-mint text-ink rounded-full shadow-lg flex items-center justify-center transition-all hover:scale-110"
      title={i18n.t('menu')}
    >
      <svg class="w-7 h-7" fill="currentColor" viewBox="0 0 24 24">
        {#if menuOpen}
          <path fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        {:else}
          <circle cx="12" cy="5" r="2"/>
          <circle cx="12" cy="12" r="2"/>
          <circle cx="12" cy="19" r="2"/>
        {/if}
      </svg>
    </button>
  </div>
</div>
{/if}
