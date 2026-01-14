<script lang="ts">
  import type { Category } from '../types';
  import { iosFocusFix } from '$lib/iosFocusFix';

  interface Props {
    categories: Category[];
    selectedCategoryId: number | null;
    onSelectCategory: (id: number) => void;
    onAddCategory: (name: string) => Promise<void>;
    onEditCategory: (id: number, name: string) => Promise<void>;
    onCategoryLongPress: (category: Category) => void;
  }

  let { categories, selectedCategoryId, onSelectCategory, onAddCategory, onEditCategory, onCategoryLongPress }: Props = $props();

  // Local state for inline editing
  let editingCategoryId = $state<number | null>(null);
  let editingCategoryName = $state('');
  let isAddingCategory = $state(false);
  let newCategoryName = $state('');
  let isSubmittingCategory = $state(false);
  let longPressTimer: number | null = null;

  function handleCategoryTouchStart(category: Category) {
    longPressTimer = window.setTimeout(() => {
      onCategoryLongPress(category);
    }, 500);
  }

  function handleCategoryTouchEnd() {
    if (longPressTimer) {
      clearTimeout(longPressTimer);
      longPressTimer = null;
    }
  }

  function startAddCategory() {
    isAddingCategory = true;
    newCategoryName = '';
    isSubmittingCategory = false;
  }

  async function saveNewCategory() {
    if (newCategoryName.trim()) {
      isSubmittingCategory = true;
      try {
        await onAddCategory(newCategoryName.trim());
        isAddingCategory = false;
        newCategoryName = '';
      } catch (error) {
        console.error('Failed to add category:', error);
        isSubmittingCategory = false;
      }
    }
  }

  function cancelAddCategory() {
    if (!isSubmittingCategory) {
      isAddingCategory = false;
      newCategoryName = '';
    }
  }

  function handleAddCategoryBlur() {
    if (isSubmittingCategory) return;

    // If there's a value, save it (for iOS Done button)
    // If empty, cancel
    if (newCategoryName.trim()) {
      saveNewCategory();
    } else {
      cancelAddCategory();
    }
  }

  function startEditCategory(category: Category) {
    editingCategoryId = category.id;
    editingCategoryName = category.name;
  }

  async function saveEditCategory() {
    if (editingCategoryId !== null && editingCategoryName.trim()) {
      try {
        await onEditCategory(editingCategoryId, editingCategoryName.trim());
        editingCategoryId = null;
        editingCategoryName = '';
      } catch (error) {
        console.error('Failed to edit category:', error);
        alert('카테고리 수정 실패: ' + error);
      }
    }
  }

  function cancelEditCategory() {
    editingCategoryId = null;
    editingCategoryName = '';
  }

  // Expose startEditCategory for parent component
  export function triggerEdit(category: Category) {
    startEditCategory(category);
  }
</script>

<!-- Category Tabs -->
<div class="category-tabs bg-canvas border-b border-stroke flex-shrink-0">
  <div class="max-w-2xl mx-auto px-4">
    <div class="flex overflow-x-auto gap-2 py-3 flex-nowrap scrollbar-hide">
      {#each categories as category (category.id)}
        {#if editingCategoryId === category.id}
          <!-- Editing Mode -->
          <div class="flex items-center gap-1 px-3 py-2 bg-accent-sky rounded-full">
            <input
              use:iosFocusFix
              bind:value={editingCategoryName}
              onkeydown={(e) => {
                if (e.key === 'Enter') saveEditCategory();
                if (e.key === 'Escape') cancelEditCategory();
              }}
              onblur={saveEditCategory}
              class="w-24 px-2 py-1 text-sm border border-accent-sky-strong rounded text-ink focus:outline-none focus:ring-0 focus:border-accent-sky-strong focus:bg-paper"
              type="text"
              autofocus
            />
          </div>
        {:else}
          <!-- Normal Mode -->
          <button
            onclick={() => onSelectCategory(category.id)}
            ontouchstart={() => handleCategoryTouchStart(category)}
            ontouchend={handleCategoryTouchEnd}
            ontouchcancel={handleCategoryTouchEnd}
            class="px-4 py-2 rounded-full text-sm font-medium whitespace-nowrap transition-colors {
              selectedCategoryId === category.id
                ? 'bg-accent-sky-strong text-ink'
                : 'bg-paper text-ink-muted hover:bg-mist'
            }"
          >
            {category.name}
          </button>
        {/if}
      {/each}

      <!-- Add Category Button or Input -->
      {#if isAddingCategory}
        <div class="flex items-center gap-1 px-3 py-2 min-w-[96px] bg-accent-mint rounded-full">
          <input
            use:iosFocusFix
            bind:value={newCategoryName}
            onkeydown={(e) => {
              if (e.key === 'Enter') saveNewCategory();
              if (e.key === 'Escape') cancelAddCategory();
            }}
            onblur={handleAddCategoryBlur}
            class="w-24 px-2 py-1 text-sm border border-accent-mint-strong rounded text-ink focus:outline-none focus:ring-0 focus:border-accent-mint-strong focus:bg-paper"
            type="text"
            placeholder="카테고리명"
            autofocus
          />
        </div>
      {:else}
        <button
          onclick={startAddCategory}
          class="min-w-[96px] px-4 py-2 rounded-full text-sm font-medium bg-accent-mint text-ink hover:bg-accent-mint-strong whitespace-nowrap flex items-center justify-center"
          title="카테고리 추가"
        >
          + 추가
        </button>
      {/if}
    </div>
  </div>
</div>
