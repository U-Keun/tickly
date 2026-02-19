<script lang="ts">
  import { flip } from 'svelte/animate';
  import type { Snippet } from 'svelte';
  import { dndzone } from 'svelte-dnd-action';
  import { reorderCategories } from '../lib/api/categoryApi';
  import type { Category } from '../types';

  interface Props {
    categories: Category[];
    onCategoriesReorder: (categories: Category[]) => void;
    children: Snippet<[Category]>;
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
      await reorderCategories(categoryIds);
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
    delayTouchStart: 0,
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
