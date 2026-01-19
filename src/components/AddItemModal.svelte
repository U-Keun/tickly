<script lang="ts">
  interface Props {
    show: boolean;
    onAdd: (text: string, memo: string | null) => void;
    onCancel: () => void;
  }

  let { show, onAdd, onCancel }: Props = $props();

  let text = $state('');
  let memo = $state('');
  let textInputElement = $state<HTMLInputElement | null>(null);

  // Reset and focus when modal opens
  $effect(() => {
    if (show) {
      text = '';
      memo = '';
      setTimeout(() => textInputElement?.focus(), 100);
    }
  });

  function handleSubmit() {
    const trimmedText = text.trim();
    if (!trimmedText) return;

    const trimmedMemo = memo.trim() || null;
    onAdd(trimmedText, trimmedMemo);
    onCancel();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSubmit();
    } else if (e.key === 'Escape') {
      e.preventDefault();
      onCancel();
    }
  }
</script>

{#if show}
  <div
    class="modal-overlay"
    onclick={onCancel}
    role="dialog"
    aria-modal="true"
    aria-labelledby="add-item-title"
  >
    <div class="modal-content" onclick={(e) => e.stopPropagation()}>
      <h3 id="add-item-title" class="modal-title">항목 추가</h3>

      <div class="form-group">
        <label for="item-text" class="form-label">할 일</label>
        <input
          bind:this={textInputElement}
          bind:value={text}
          onkeydown={handleKeydown}
          id="item-text"
          type="text"
          class="form-input"
          placeholder="할 일을 입력하세요"
          autocomplete="off"
        />
      </div>

      <div class="form-group">
        <label for="item-memo" class="form-label">메모 (선택)</label>
        <textarea
          bind:value={memo}
          onkeydown={handleKeydown}
          id="item-memo"
          class="form-textarea"
          placeholder="메모를 입력하세요"
          rows="3"
        ></textarea>
      </div>

      <div class="button-group">
        <button type="button" class="btn-cancel" onclick={onCancel}>
          취소
        </button>
        <button
          type="button"
          class="btn-add"
          onclick={handleSubmit}
          disabled={!text.trim()}
        >
          추가
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: flex-start;
    justify-content: center;
    padding-top: 15vh;
    padding-bottom: env(safe-area-inset-bottom, 0);
    z-index: 50;
    overflow-y: auto;
    -webkit-overflow-scrolling: touch;
  }

  .modal-content {
    background: var(--color-white);
    border-radius: 16px;
    padding: 24px;
    width: 100%;
    max-width: 340px;
    margin: 0 16px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
  }

  .modal-title {
    font-size: 18px;
    font-weight: 600;
    margin-bottom: 20px;
    color: var(--color-ink);
  }

  .form-group {
    margin-bottom: 16px;
  }

  .form-label {
    display: block;
    font-size: 14px;
    font-weight: 500;
    color: var(--color-ink-muted);
    margin-bottom: 6px;
  }

  .form-input,
  .form-textarea {
    width: 100%;
    padding: 12px;
    border: 2px solid var(--color-stroke);
    border-radius: 10px;
    font-size: 16px;
    box-sizing: border-box;
    transition: border-color 0.2s;
    background: var(--color-white);
    color: var(--color-ink);
  }

  .form-input:focus,
  .form-textarea:focus {
    outline: none;
    border-color: var(--color-accent-sky);
  }

  .form-textarea {
    resize: none;
    line-height: 1.4;
  }

  .form-input::placeholder,
  .form-textarea::placeholder {
    color: var(--color-ink-muted);
  }

  .button-group {
    display: flex;
    gap: 12px;
    justify-content: flex-end;
    margin-top: 20px;
  }

  .btn-cancel,
  .btn-add {
    padding: 12px 20px;
    border-radius: 10px;
    font-size: 15px;
    font-weight: 500;
    cursor: pointer;
    transition: background-color 0.2s;
    border: none;
  }

  .btn-cancel {
    background: var(--color-canvas);
    color: var(--color-ink-muted);
  }

  .btn-cancel:hover {
    background: var(--color-mist);
  }

  .btn-add {
    background: var(--color-accent-sky);
    color: var(--color-ink);
  }

  .btn-add:hover:not(:disabled) {
    background: var(--color-accent-sky-strong);
  }

  .btn-add:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
