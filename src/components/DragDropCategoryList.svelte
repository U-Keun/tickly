<script lang="ts">
  import { flip } from 'svelte/animate';
  import { dndzone } from 'svelte-dnd-action';
  import { invoke } from '@tauri-apps/api/core';
  import type { Category } from '../types';

  interface Props {
    categories: Category[];
    onCategoriesReorder: (categories: Category[]) => void;
    children: any;
  }

  let { categories = $bindable([]), onCategoriesReorder, children }: Props = $props();

  const flipDurationMs = 200;

  function handleDndConsider(e: CustomEvent<{ items: Category[] }>) {
    categories = e.detail.items;
  }

  async function handleDndFinalize(e: CustomEvent<{ items: Category[] }>) {
    categories = e.detail.items;

    const categoryIds = categories.map(cat => cat.id);
    try {
      await invoke('reorder_categories', { categoryIds });
      onCategoriesReorder(categories);
    } catch (error) {
      console.error('Failed to reorder categories:', error);
    }
  }
</script>

<div
  use:dndzone={{
    items: categories,
    flipDurationMs,
    type: 'categories',
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
  {#each categories as category (category.id)}
    <div animate:flip={{ duration: flipDurationMs }}>
      {@render children(category)}
    </div>
  {/each}
</div>
