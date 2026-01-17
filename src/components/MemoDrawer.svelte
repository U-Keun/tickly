<script lang="ts">
  import type { TodoItem } from '../types';

  interface Props {
    item: TodoItem;
    onSaveMemo: (id: number, memo: string | null) => void;
    closeDrawer: () => void;
  }

  let { item, onSaveMemo, closeDrawer }: Props = $props();

  let memoText = $state('');
  let isSaving = $state(false);

  // Sync memoText when item changes
  $effect(() => {
    memoText = item.memo || '';
  });

  async function saveMemo() {
    if (isSaving) return;
    isSaving = true;

    const trimmed = memoText.trim();
    const newMemo = trimmed || null;

    if (newMemo !== item.memo) {
      onSaveMemo(item.id, newMemo);
    }

    isSaving = false;
    closeDrawer();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      saveMemo();
    }
  }
</script>

<div class="memo-drawer">
  <label class="memo-label" for="memo-input">
    메모
  </label>
  <textarea
    id="memo-input"
    bind:value={memoText}
    onkeydown={handleKeydown}
    class="memo-textarea"
    placeholder="메모를 입력하세요..."
    rows="3"
  ></textarea>
  <div class="actions">
    <button
      type="button"
      class="btn-cancel"
      onclick={closeDrawer}
    >
      취소
    </button>
    <button
      type="button"
      class="btn-save"
      onclick={saveMemo}
      disabled={isSaving}
    >
      저장
    </button>
  </div>
</div>

<style>
  .memo-drawer {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .memo-label {
    font-size: 14px;
    font-weight: 500;
    color: #333;
  }

  .memo-textarea {
    width: 100%;
    padding: 10px;
    border: 1px solid #ddd;
    border-radius: 8px;
    font-size: 14px;
    line-height: 1.4;
    resize: none;
    box-sizing: border-box;
  }

  .memo-textarea:focus {
    outline: none;
    border-color: #87CEEB;
  }

  .memo-textarea::placeholder {
    color: #999;
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
  }

  .btn-cancel,
  .btn-save {
    padding: 8px 16px;
    border-radius: 8px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: background-color 0.2s;
  }

  .btn-cancel {
    background: #f0f0f0;
    border: none;
    color: #666;
  }

  .btn-cancel:hover {
    background: #e0e0e0;
  }

  .btn-save {
    background: #87CEEB;
    border: none;
    color: #333;
  }

  .btn-save:hover {
    background: #7AC5E0;
  }

  .btn-save:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
</style>
