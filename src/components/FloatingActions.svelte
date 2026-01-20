<script lang="ts">
  import { fly } from 'svelte/transition';
  import { cubicOut } from 'svelte/easing';
  import { i18n } from '$lib/i18n';

  let {
    show,
    safeAreaBottom = 0,
    delay = 300,
    onAdd,
    onReset,
  }: {
    show: boolean;
    safeAreaBottom?: number;
    delay?: number;
    onAdd: () => void;
    onReset: () => void;
  } = $props();
</script>

{#if show}
<div
  class="fixed bottom-12 right-6 flex flex-col gap-3 items-center z-10 transition-[margin] duration-300"
  style="margin-bottom: {safeAreaBottom}px;"
  in:fly={{ y: 100, duration: 400, delay, easing: cubicOut }}
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
</div>
{/if}
