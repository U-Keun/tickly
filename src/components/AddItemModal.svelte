<script lang="ts">
  import { i18n } from '$lib/i18n';
  import ModalWrapper from './ModalWrapper.svelte';

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
    }
  }
</script>

<ModalWrapper {show} onClose={onCancel} position="top">
  <h3 class="modal-title">{i18n.t('addItemTitle')}</h3>

  <div class="form-group">
    <label for="item-text" class="form-label">{i18n.t('todoLabel')}</label>
    <input
      bind:this={textInputElement}
      bind:value={text}
      onkeydown={handleKeydown}
      id="item-text"
      type="text"
      class="form-input"
      placeholder={i18n.t('todoPlaceholder')}
      autocomplete="off"
    />
  </div>

  <div class="form-group">
    <label for="item-memo" class="form-label">{i18n.t('memoLabel')}</label>
    <textarea
      bind:value={memo}
      onkeydown={handleKeydown}
      id="item-memo"
      class="form-textarea"
      placeholder={i18n.t('memoPlaceholder')}
      rows="3"
    ></textarea>
  </div>

  <div class="button-group">
    <button type="button" class="btn btn-cancel" onclick={onCancel}>
      {i18n.t('cancel')}
    </button>
    <button
      type="button"
      class="btn btn-primary"
      onclick={handleSubmit}
      disabled={!text.trim()}
    >
      {i18n.t('add')}
    </button>
  </div>
</ModalWrapper>

<style>
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

  .btn {
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

  .btn-primary {
    background: var(--color-accent-sky);
    color: var(--color-ink);
  }

  .btn-primary:hover:not(:disabled) {
    background: var(--color-accent-sky-strong);
  }

  .btn-primary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
