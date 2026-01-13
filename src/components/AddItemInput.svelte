<script lang="ts">
  import { iosFocusFix } from '$lib/iosFocusFix';

  interface Props {
    onAdd: (text: string) => void;
  }

  let { onAdd }: Props = $props();
  let inputText = $state('');

  function handleSubmit(e: Event) {
    e.preventDefault();
    const text = inputText.trim();
    if (text) {
      onAdd(text);
      inputText = '';
    }
  }
</script>

<form onsubmit={handleSubmit} class="p-4 bg-white sketch-border">
  <div class="flex gap-2">
    <input
      use:iosFocusFix
      type="text"
      bind:value={inputText}
      placeholder="항목을 입력하세요..."
      class="flex-1 px-4 py-3 text-base border border-stroke rounded-lg text-ink placeholder:text-ink-muted focus:outline-none focus:ring-2 focus:ring-accent-sky focus:border-transparent"
    />
    <button
      type="submit"
      class="px-6 py-3 bg-accent-sky text-ink rounded-lg font-medium hover:bg-accent-sky-strong active:bg-accent-sky-strong transition-colors"
    >
      추가
    </button>
  </div>
</form>
