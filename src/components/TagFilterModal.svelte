<script lang="ts">
  import type { Tag } from '../types';
  import { i18n } from '$lib/i18n';
  import ModalWrapper from './ModalWrapper.svelte';

  interface Props {
    show: boolean;
    tags: Tag[];
    activeTagId: number | null;
    onSelect: (tagId: number) => void;
    onClear: () => void;
    onClose: () => void;
  }

  let { show, tags, activeTagId, onSelect, onClear, onClose }: Props = $props();

  function handleSelect(tagId: number) {
    onSelect(tagId);
    onClose();
  }

  function handleClear() {
    onClear();
    onClose();
  }
</script>

<ModalWrapper {show} onClose={onClose} size="sm" position="center">
  <h3 class="modal-title">{i18n.t('tagFilter')}</h3>

  {#if tags.length === 0}
    <p class="empty-text">{i18n.t('tagEmpty')}</p>
  {:else}
    <div class="tag-list">
      {#each tags as tag (tag.id)}
        <button
          type="button"
          class="tag-item"
          class:active={activeTagId === tag.id}
          onclick={() => handleSelect(tag.id)}
        >
          <span class="tag-name">{tag.name}</span>
          {#if activeTagId === tag.id}
            <svg class="check-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="20 6 9 17 4 12"></polyline>
            </svg>
          {/if}
        </button>
      {/each}
    </div>

    {#if activeTagId !== null}
      <button
        type="button"
        class="clear-btn"
        onclick={handleClear}
      >
        {i18n.t('tagFilterClear')}
      </button>
    {/if}
  {/if}
</ModalWrapper>

<style>
  .modal-title {
    font-size: 18px;
    font-weight: 600;
    color: var(--color-ink);
    margin: 0 0 16px 0;
  }

  .empty-text {
    font-size: 14px;
    color: var(--color-ink-muted);
    text-align: center;
    padding: 16px 0;
  }

  .tag-list {
    display: flex;
    flex-direction: column;
    gap: 2px;
    max-height: 300px;
    overflow-y: auto;
  }

  .tag-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: 10px 12px;
    background: none;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    font-size: 15px;
    color: var(--color-ink);
    text-align: left;
    transition: background 0.15s;
  }

  .tag-item:hover {
    background: var(--color-canvas);
  }

  .tag-item.active {
    background: var(--color-accent-sky);
  }

  .tag-name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .check-icon {
    flex-shrink: 0;
    color: var(--color-accent-sky-strong);
  }

  .clear-btn {
    display: block;
    width: 100%;
    margin-top: 12px;
    padding: 10px;
    background: var(--color-canvas);
    border: none;
    border-radius: 8px;
    font-size: 14px;
    color: var(--color-ink-muted);
    cursor: pointer;
    transition: background 0.15s;
  }

  .clear-btn:hover {
    background: var(--color-mist);
  }
</style>
