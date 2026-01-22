<script lang="ts">
  import type { TodoItem } from '../types';
  import { i18n } from '$lib/i18n';

  interface Props {
    item: TodoItem;
    onSaveMemo: (id: number, memo: string | null) => void;
    onEditText: (id: number, text: string) => void;
    onEditModeChange?: (editing: boolean) => void;
    closeDrawer: () => void;
  }

  let { item, onSaveMemo, onEditText, onEditModeChange, closeDrawer }: Props = $props();

  let isEditMode = $state(false);
  let editText = $state('');
  let memoText = $state('');
  let isSaving = $state(false);

  // Sync texts when item changes
  $effect(() => {
    editText = item.text;
    memoText = item.memo || '';
  });

  function enterEditMode() {
    isEditMode = true;
    onEditModeChange?.(true);
  }

  function cancelEdit() {
    editText = item.text;
    memoText = item.memo || '';
    isEditMode = false;
    onEditModeChange?.(false);
  }

  async function saveChanges() {
    if (isSaving) return;
    isSaving = true;

    const trimmedText = editText.trim();
    const trimmedMemo = memoText.trim();
    const newMemo = trimmedMemo || null;

    // Save text if changed
    if (trimmedText && trimmedText !== item.text) {
      onEditText(item.id, trimmedText);
    }

    // Save memo if changed
    if (newMemo !== item.memo) {
      onSaveMemo(item.id, newMemo);
    }

    isSaving = false;
    isEditMode = false;
    onEditModeChange?.(false);
    closeDrawer();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      e.preventDefault();
      cancelEdit();
    }
  }
</script>

<div class="memo-drawer">
  {#if isEditMode}
    <!-- Edit Mode -->
    <div class="edit-section">
      <input
        id="text-input"
        bind:value={editText}
        onkeydown={handleKeydown}
        class="text-input"
        type="text"
        placeholder={i18n.t('todoPlaceholderAlt')}
      />
    </div>
    <div class="edit-section">
      <textarea
        id="memo-input"
        bind:value={memoText}
        onkeydown={handleKeydown}
        class="memo-textarea"
        placeholder={i18n.t('memoPlaceholderAlt')}
        rows="3"
      ></textarea>
    </div>
    <div class="actions">
      <button
        type="button"
        class="btn-cancel"
        onclick={cancelEdit}
        title={i18n.t('cancel')}
      >
        <!-- Lucide X icon -->
        <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <line x1="18" y1="6" x2="6" y2="18"></line>
          <line x1="6" y1="6" x2="18" y2="18"></line>
        </svg>
      </button>
      <button
        type="button"
        class="btn-save"
        onclick={saveChanges}
        disabled={isSaving}
        title={i18n.t('save')}
      >
        <!-- Lucide Check icon -->
        <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="20 6 9 17 4 12"></polyline>
        </svg>
      </button>
    </div>
  {:else}
    <!-- View Mode -->
    {#if item.memo}
      <div class="memo-display">
        <p class="memo-text">{item.memo}</p>
      </div>
    {/if}
    <div class="actions">
      <button
        type="button"
        class="btn-edit"
        onclick={enterEditMode}
        title={i18n.t('edit')}
      >
        <!-- Lucide Pencil icon -->
        <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z"></path>
          <path d="m15 5 4 4"></path>
        </svg>
      </button>
    </div>
  {/if}
</div>

<style>
  .memo-drawer {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .edit-section {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .label {
    font-size: 14px;
    font-weight: 500;
    color: var(--color-ink);
  }

  .text-input {
    width: 100%;
    padding: 10px;
    border: 1px solid var(--color-stroke);
    border-radius: 8px;
    font-size: 14px;
    line-height: 1.4;
    box-sizing: border-box;
    background: var(--color-white);
    color: var(--color-ink);
  }

  .text-input:focus {
    outline: none;
    border-color: var(--color-accent-sky);
  }

  .memo-textarea {
    width: 100%;
    padding: 10px;
    border: 1px solid var(--color-stroke);
    border-radius: 8px;
    font-size: 14px;
    line-height: 1.4;
    resize: none;
    box-sizing: border-box;
    background: var(--color-white);
    color: var(--color-ink);
  }

  .memo-textarea:focus {
    outline: none;
    border-color: var(--color-accent-sky);
  }

  .memo-textarea::placeholder,
  .text-input::placeholder {
    color: var(--color-ink-muted);
  }

  .memo-display {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .memo-text {
    margin: 0;
    font-size: 14px;
    color: var(--color-ink-muted);
    line-height: 1.4;
    white-space: pre-wrap;
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
  }

  .btn-cancel,
  .btn-save,
  .btn-edit {
    width: 36px;
    height: 36px;
    padding: 0;
    border-radius: 50%;
    border: none;
    cursor: pointer;
    transition: background-color 0.2s;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .icon {
    width: 18px;
    height: 18px;
  }

  .btn-cancel {
    background: var(--color-canvas);
    color: var(--color-ink-muted);
  }

  .btn-cancel:hover {
    background: var(--color-mist);
  }

  .btn-save {
    background: var(--color-accent-sky);
    color: var(--color-ink);
  }

  .btn-save:hover {
    background: var(--color-accent-sky-strong);
  }

  .btn-save:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .btn-edit {
    background: var(--color-accent-sky);
    color: var(--color-ink);
  }

  .btn-edit:hover {
    background: var(--color-accent-sky-strong);
  }
</style>
