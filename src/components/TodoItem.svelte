<script lang="ts">
  import type { TodoItem } from '../types';

  interface Props {
    item: TodoItem;
    onToggle: (id: number) => void;
    onDelete: (id: number) => void;
  }

  let { item, onToggle, onDelete }: Props = $props();
</script>

<div class="flex items-center gap-3 p-4 bg-white border-b border-gray-200">
  <!-- Checkbox -->
  <button
    onclick={() => onToggle(item.id)}
    class="flex-shrink-0 w-6 h-6 rounded border-2 border-gray-300 flex items-center justify-center {item.done ? 'bg-blue-500 border-blue-500' : 'bg-white'}"
    aria-label={item.done ? 'Mark as incomplete' : 'Mark as complete'}
  >
    {#if item.done}
      <svg class="w-4 h-4 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M5 13l4 4L19 7" />
      </svg>
    {/if}
  </button>

  <!-- Text -->
  <div class="flex-1 {item.done ? 'line-through text-gray-400' : 'text-gray-800'}">
    {item.text}
  </div>

  <!-- Delete Button -->
  <button
    onclick={() => onDelete(item.id)}
    class="flex-shrink-0 w-8 h-8 text-red-500 hover:bg-red-50 rounded flex items-center justify-center"
    aria-label="Delete item"
  >
    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
    </svg>
  </button>
</div>
