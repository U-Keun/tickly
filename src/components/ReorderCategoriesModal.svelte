<script lang="ts">
  import type { Category } from '../types';
  import DragDropCategoryList from './DragDropCategoryList.svelte';

  interface Props {
    show: boolean;
    categories: Category[];
    onCategoriesReorder: (categories: Category[]) => void;
    onClose: () => void;
  }

  let { show, categories = $bindable([]), onCategoriesReorder, onClose }: Props = $props();

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      event.preventDefault();
      onClose();
    }
  }
</script>

{#if show}
  <div
    class="fixed inset-0 bg-black/50 z-50 flex items-start justify-center overflow-y-auto"
    style="padding-top: 12vh; padding-bottom: env(safe-area-inset-bottom, 0);"
    role="dialog"
    aria-modal="true"
    aria-labelledby="reorder-categories-title"
    tabindex="0"
    onclick={onClose}
    onkeydown={handleKeydown}
  >
    <div
      class="bg-white w-full max-w-md mx-4 rounded-2xl shadow-xl p-6"
      onclick={(event) => event.stopPropagation()}
    >
      <div class="flex items-center justify-between mb-4">
        <div>
          <h3 id="reorder-categories-title" class="text-lg font-semibold text-ink">카테고리 순서 정렬</h3>
          <p class="text-sm text-ink-muted mt-1">드래그해서 카테고리 순서를 바꿔보세요.</p>
        </div>
        <button
          class="text-ink-muted hover:text-ink"
          type="button"
          aria-label="닫기"
          onclick={onClose}
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      {#if categories.length === 0}
        <div class="py-10 text-center text-ink-muted text-sm">
          정렬할 카테고리가 없습니다.
        </div>
      {:else}
        <DragDropCategoryList {categories} onCategoriesReorder={onCategoriesReorder}>
          {#snippet children(category)}
            <div class="flex items-center gap-3 py-3 px-2">
              <div class="drag-handle-zone cursor-grab text-ink-muted">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 6h8M8 12h8M8 18h8" />
                </svg>
              </div>
              <div class="flex-1 min-w-0">
                <p class="text-sm font-medium text-ink truncate">
                  {category.name}
                </p>
              </div>
            </div>
          {/snippet}
        </DragDropCategoryList>
      {/if}
    </div>
  </div>
{/if}
