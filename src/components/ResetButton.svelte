<script lang="ts">
  interface Props {
    onReset: () => void;
  }

  let { onReset }: Props = $props();
</script>

<!-- Floating Reset Button -->
<button
  onclick={onReset}
  class="fixed bottom-6 right-6 w-14 h-14 bg-accent-peach-strong hover:bg-accent-peach text-ink rounded-full shadow-lg flex items-center justify-center transition-all hover:scale-110"
  title="체크 초기화"
>
  <svg class="w-7 h-7" fill="none" stroke="currentColor" viewBox="0 0 24 24">
    <path
      stroke-linecap="round"
      stroke-linejoin="round"
      stroke-width="1.7"
      d="M4 4.5v4.6h.7m14.7 2a7.7 7.7 0 0 0-15-1.8m0 0h4.1m11.2 10.1v-4.4h-.7m.7 0a7.8 7.8 0 0 1-14.6-2.2m14.6 2.2h-4.2"
    />
    <path
      stroke-linecap="round"
      stroke-linejoin="round"
      stroke-width="1.1"
      d="M3.8 8.8l2.2-2.3"
    />
  </svg>
</button>
