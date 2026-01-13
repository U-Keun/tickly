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

<div class="flex items-center gap-3 p-4 bg-paper sketch-border">
  <!-- Checkbox -->
  <button
    onclick={() => onToggle(item.id)}
    class="flex-shrink-0 w-7 h-7 rounded-full text-ink flex items-center justify-center"
    aria-label={item.done ? 'Mark as incomplete' : 'Mark as complete'}
  >
    <svg class="w-7 h-7" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="1.8"
        d="M12 3.6c4.5-.3 8.4 3.1 8.2 7.8-.2 4.4-3.7 8.2-8.3 8.1-4.6-.1-8.2-3.8-8-8.5.2-4.2 3.5-7.7 8.1-7.4"
      />
      {#if item.done}
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2.2"
          d="M7.2 12.6l3.1 3 6.7-7.1"
        />
      {/if}
    </svg>
  </button>

  <!-- Text or Input -->
  {#if isEditing}
    <input
      bind:this={inputElement}
      bind:value={editText}
      onkeydown={handleKeydown}
      onblur={saveEdit}
      class="flex-1 px-2 py-1 border border-accent-sky-strong rounded text-ink focus:outline-none focus:ring-2 focus:ring-accent-sky"
      type="text"
    />
  {:else}
    <div class="flex-1 {item.done ? 'line-through text-ink-muted' : 'text-ink'}">
      {item.text}
    </div>
  {/if}

  <!-- Edit Button -->
  {#if !isEditing}
    <button
      onclick={startEdit}
      class="flex-shrink-0 w-8 h-8 text-accent-sky-strong hover:bg-accent-sky rounded flex items-center justify-center"
      aria-label="Edit item"
    >
      <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="1.8"
          d="M4.5 18.8l4.6-.9 8.3-8.6-3.8-3.7-8.5 8.7-.6 4.5z"
        />
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="1.3"
          d="M12.1 6.1l3.8 3.7"
        />
      </svg>
    </button>
  {/if}
</div>
