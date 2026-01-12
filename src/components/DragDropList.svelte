<script lang="ts">
  import { flip } from 'svelte/animate';
  import { dndzone } from 'svelte-dnd-action';
  import { invoke } from '@tauri-apps/api/core';
  import type { TodoItem } from '../types';

  interface Props {
    items: TodoItem[];
    onItemsReorder: (items: TodoItem[]) => void;
    children: any;
  }

  let { items = $bindable([]), onItemsReorder, children }: Props = $props();

  const flipDurationMs = 200;

  function handleDndConsider(e: CustomEvent<{ items: TodoItem[] }>) {
    // Update items during drag (optimistic update)
    items = e.detail.items;
  }

  async function handleDndFinalize(e: CustomEvent<{ items: TodoItem[] }>) {
    // Update local state
    items = e.detail.items;

    // Save to backend
    const itemIds = items.map(item => item.id);
    try {
      await invoke('reorder_items', { itemIds });
      onItemsReorder(items);
    } catch (error) {
      console.error('Failed to reorder items:', error);
    }
  }
</script>

<div
  use:dndzone={{
    items,
    flipDurationMs,
    type: 'items',
    dragDisabled: false,
    dropTargetStyle: {},
    dropTargetClasses: [],
    dragHandle: '.drag-handle-zone',
    touchStartDelay: 0,
    morphDisabled: false,
    centreDraggedOnCursor: false
  }}
  onconsider={handleDndConsider}
  onfinalize={handleDndFinalize}
  class="divide-y divide-gray-200"
>
  {#each items as item (item.id)}
    <div animate:flip={{ duration: flipDurationMs }}>
      {@render children(item)}
    </div>
  {/each}
</div>
