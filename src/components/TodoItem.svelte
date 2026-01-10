<script lang="ts">
  import type { TodoItem } from '../types';

  interface Props {
    item: TodoItem;
    onToggle: (id: number) => void;
    onDelete: (id: number) => void;
    onEdit: (id: number, text: string) => void;
  }

  let { item, onToggle, onDelete, onEdit }: Props = $props();
  let isEditing = $state(false);
  let editText = $state('');
  let inputElement: HTMLInputElement;

  function startEdit() {
    isEditing = true;
    editText = item.text;
    setTimeout(() => inputElement?.focus(), 0);
  }

  function saveEdit() {
    const trimmed = editText.trim();
    if (trimmed && trimmed !== item.text) {
      onEdit(item.id, trimmed);
    }
    isEditing = false;
  }

  function cancelEdit() {
    isEditing = false;
    editText = '';
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter') {
      e.preventDefault();
      saveEdit();
    } else if (e.key === 'Escape') {
      e.preventDefault();
      cancelEdit();
    }
  }
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

  <!-- Text or Input -->
  {#if isEditing}
    <input
      bind:this={inputElement}
      bind:value={editText}
      onkeydown={handleKeydown}
      onblur={saveEdit}
      class="flex-1 px-2 py-1 border border-blue-500 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
      type="text"
    />
  {:else}
    <div class="flex-1 {item.done ? 'line-through text-gray-400' : 'text-gray-800'}">
      {item.text}
    </div>
  {/if}

  <!-- Edit Button -->
  {#if !isEditing}
    <button
      onclick={startEdit}
      class="flex-shrink-0 w-8 h-8 text-blue-500 hover:bg-blue-50 rounded flex items-center justify-center"
      aria-label="Edit item"
    >
      <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
      </svg>
    </button>
  {/if}

  <!-- Delete Button -->
  {#if !isEditing}
    <button
      onclick={() => onDelete(item.id)}
      class="flex-shrink-0 w-8 h-8 text-red-500 hover:bg-red-50 rounded flex items-center justify-center"
      aria-label="Delete item"
    >
      <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
      </svg>
    </button>
  {/if}
</div>
