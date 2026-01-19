<script lang="ts">
  import type { TodoItem } from '../types';
  import DragDropList from './DragDropList.svelte';

  interface Props {
    show: boolean;
    items: TodoItem[];
    onItemsReorder: (items: TodoItem[]) => void;
    onClose: () => void;
  }

  let { show, items = $bindable([]), onItemsReorder, onClose }: Props = $props();

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
    aria-labelledby="reorder-title"
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
          <h3 id="reorder-title" class="text-lg font-semibold text-ink">항목 순서 정렬</h3>
          <p class="text-sm text-ink-muted mt-1">드래그해서 현재 카테고리의 순서를 바꿔보세요.</p>
        </div>
        <button
          class="text-ink-muted hover:text-ink"
          type="button"
          aria-label="닫기"
          onclick={onClose}
        >
          ✕
        </button>
      </div>

      {#if items.length === 0}
        <div class="py-10 text-center text-ink-muted text-sm">
          정렬할 항목이 없습니다.
        </div>
      {:else}
        <DragDropList {items} onItemsReorder={onItemsReorder}>
          {#snippet children(item)}
            <div class="flex items-center gap-3 py-3 px-2">
              <div class="drag-handle-zone cursor-grab text-ink-muted">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 6h8M8 12h8M8 18h8" />
                </svg>
              </div>
              <div class="flex-1 min-w-0">
                <p class="text-sm font-medium text-ink truncate">
                  {item.text}
                </p>
              </div>
              {#if item.done}
                <span class="text-xs text-green-600 font-medium">완료</span>
              {/if}
            </div>
          {/snippet}
        </DragDropList>
      {/if}
    </div>
  </div>
{/if}
