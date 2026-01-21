<script lang="ts">
  import type { Category } from '../types';
  import ModalWrapper from './ModalWrapper.svelte';
  import { i18n } from '$lib/i18n';

  interface Props {
    show: boolean;
    category: Category | null;
    onEdit: () => void;
    onDelete: () => void;
    onClose: () => void;
  }

  let { show, category, onEdit, onDelete, onClose }: Props = $props();
</script>

<ModalWrapper show={show && !!category} onClose={onClose}>
  {#if category}
    <h3 class="modal-title">{category.name}</h3>
    <div class="menu-list">
      <button class="menu-item edit" onclick={onEdit}>
        {i18n.t('editName')}
      </button>
      <button class="menu-item danger" onclick={onDelete}>
        {i18n.t('categoryDelete')}
      </button>
      <button class="menu-item cancel" onclick={onClose}>
        {i18n.t('cancel')}
      </button>
    </div>
  {/if}
</ModalWrapper>

<style>
  .modal-title {
    font-size: 18px;
    font-weight: 600;
    margin-bottom: 16px;
    text-align: center;
    color: var(--color-ink);
  }

  .menu-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .menu-item {
    padding: 14px 16px;
    border-radius: 10px;
    font-size: 15px;
    font-weight: 500;
    text-align: left;
    cursor: pointer;
    transition: background-color 0.2s;
    border: none;
  }

  .menu-item.edit {
    background: var(--color-accent-sky);
    color: var(--color-ink);
  }

  .menu-item.edit:hover {
    background: var(--color-accent-sky-strong);
  }

  .menu-item.danger {
    background: #fef2f2;
    color: #dc2626;
  }

  .menu-item.danger:hover {
    background: #fee2e2;
  }

  .menu-item.cancel {
    background: var(--color-canvas);
    color: var(--color-ink-muted);
  }

  .menu-item.cancel:hover {
    background: var(--color-mist);
  }
</style>
