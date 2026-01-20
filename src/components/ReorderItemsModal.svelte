<script lang="ts">
  import type { TodoItem } from '../types';
  import DragDropList from './DragDropList.svelte';
  import { i18n } from '$lib/i18n';

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
          <h3 id="reorder-title" class="text-lg font-semibold text-ink">{i18n.t('reorderItemsTitle')}</h3>
          <p class="text-sm text-ink-muted mt-1">{i18n.t('reorderItemsSubtitle')}</p>
        </div>
        <button
          class="text-ink-muted hover:text-ink"
          type="button"
          aria-label={i18n.t('close')}
          onclick={onClose}
        >
          ✕
        </button>
      </div>

      {#if items.length === 0}
        <div class="py-10 text-center text-ink-muted text-sm">
          {i18n.t('noItemsToReorder')}
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
                {#if item.memo}
                  <p class="text-xs text-ink-muted truncate mt-1">
                    {item.memo}
                  </p>
                {/if}
              </div>
              {#if item.done}
                <span class="text-xs text-green-600 font-medium">{i18n.t('done')}</span>
              {/if}
            </div>
          {/snippet}
        </DragDropList>
      {/if}
    </div>
  </div>
{/if}
