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

<form onsubmit={handleSubmit} class="p-4 bg-gray-50 border-b border-gray-200">
  <div class="flex gap-2">
    <input
      use:iosFocusFix
      type="text"
      bind:value={inputText}
      placeholder="항목을 입력하세요..."
      class="flex-1 px-4 py-3 text-base border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
    />
    <button
      type="submit"
      class="px-6 py-3 bg-blue-500 text-white rounded-lg font-medium hover:bg-blue-600 active:bg-blue-700 transition-colors"
    >
      추가
    </button>
  </div>
</form>
