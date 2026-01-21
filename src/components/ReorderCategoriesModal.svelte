<script lang="ts">
  import type { Category } from '../types';
  import DragDropCategoryList from './DragDropCategoryList.svelte';
  import ModalWrapper from './ModalWrapper.svelte';
  import { i18n } from '$lib/i18n';

  interface Props {
    show: boolean;
    categories: Category[];
    onCategoriesReorder: (categories: Category[]) => void;
    onClose: () => void;
  }

  let { show, categories = $bindable([]), onCategoriesReorder, onClose }: Props = $props();
</script>

<ModalWrapper {show} onClose={onClose} size="md" position="top">
  <div class="modal-header">
    <div>
      <h3 class="modal-title">{i18n.t('reorderCategoriesTitle')}</h3>
      <p class="modal-subtitle">{i18n.t('reorderCategoriesSubtitle')}</p>
    </div>
    <button class="close-btn" type="button" aria-label={i18n.t('close')} onclick={onClose}>
      <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
      </svg>
    </button>
  </div>

  {#if categories.length === 0}
    <div class="empty-state">
      {i18n.t('noCategoriesToReorder')}
    </div>
  {:else}
    <DragDropCategoryList {categories} onCategoriesReorder={onCategoriesReorder}>
      {#snippet children(category)}
        <div class="list-item">
          <div class="drag-handle-zone">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 6h8M8 12h8M8 18h8" />
            </svg>
          </div>
          <div class="item-content">
            <p class="item-text">{category.name}</p>
          </div>
        </div>
      {/snippet}
    </DragDropCategoryList>
  {/if}
</ModalWrapper>

<style>
  .modal-header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    margin-bottom: 16px;
  }

  .modal-title {
    font-size: 18px;
    font-weight: 600;
    color: var(--color-ink);
  }

  .modal-subtitle {
    font-size: 14px;
    color: var(--color-ink-muted);
    margin-top: 4px;
  }

  .close-btn {
    padding: 4px;
    background: none;
    border: none;
    color: var(--color-ink-muted);
    cursor: pointer;
    border-radius: 4px;
  }

  .close-btn:hover {
    color: var(--color-ink);
  }

  .empty-state {
    padding: 40px 0;
    text-align: center;
    color: var(--color-ink-muted);
    font-size: 14px;
  }

  .list-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 8px;
  }

  .drag-handle-zone {
    cursor: grab;
    color: var(--color-ink-muted);
  }

  .item-content {
    flex: 1;
    min-width: 0;
  }

  .item-text {
    font-size: 14px;
    font-weight: 500;
    color: var(--color-ink);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
