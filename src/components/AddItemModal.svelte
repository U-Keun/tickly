<script lang="ts">
  import { i18n } from '$lib/i18n';
  import type { RepeatType } from '../types';
  import ModalWrapper from './ModalWrapper.svelte';
  import RepeatSelector from './RepeatSelector.svelte';

  interface Props {
    show: boolean;
    onAdd: (text: string, memo: string | null, repeatType: RepeatType, repeatDetail: string | null, trackStreak: boolean) => void;
    onCancel: () => void;
  }

  let { show, onAdd, onCancel }: Props = $props();

  let text = $state('');
  let memo = $state('');
  let repeatType = $state<RepeatType>('none');
  let repeatDetail = $state<number[]>([]);
  let trackStreak = $state(false);
  let textInputElement = $state<HTMLInputElement | null>(null);

  // Reset and focus when modal opens
  $effect(() => {
    if (show) {
      text = '';
      memo = '';
      repeatType = 'none';
      repeatDetail = [];
      trackStreak = false;
      setTimeout(() => textInputElement?.focus(), 100);
    }
  });

  function handleSubmit() {
    const trimmedText = text.trim();
    if (!trimmedText) return;

    const trimmedMemo = memo.trim() || null;
    const repeatDetailJson = repeatDetail.length > 0 ? JSON.stringify(repeatDetail) : null;
    onAdd(trimmedText, trimmedMemo, repeatType, repeatDetailJson, trackStreak);
    onCancel();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSubmit();
    }
  }

  function handleRepeatTypeChange(type: RepeatType) {
    repeatType = type;
  }

  function handleRepeatDetailChange(detail: number[]) {
    repeatDetail = detail;
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

  <div class="form-group">
    <RepeatSelector
      {repeatType}
      {repeatDetail}
      onRepeatTypeChange={handleRepeatTypeChange}
      onRepeatDetailChange={handleRepeatDetailChange}
    />
  </div>

  <div class="form-group streak-section">
    <label class="streak-toggle-label">
      <span class="streak-label-text">{i18n.t('trackStreak')}</span>
      <button
        type="button"
        class="streak-toggle"
        class:active={trackStreak}
        onclick={() => trackStreak = !trackStreak}
        aria-pressed={trackStreak}
      >
        <span class="toggle-track">
          <span class="toggle-thumb"></span>
        </span>
      </button>
    </label>
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

  .streak-section {
    padding-top: 4px;
  }

  .streak-toggle-label {
    display: flex;
    align-items: center;
    justify-content: space-between;
    cursor: pointer;
  }

  .streak-label-text {
    font-size: 14px;
    color: var(--color-ink);
  }

  .streak-toggle {
    background: none;
    border: none;
    padding: 0;
    cursor: pointer;
  }

  .toggle-track {
    display: block;
    width: 44px;
    height: 24px;
    background: var(--color-stroke);
    border-radius: 12px;
    position: relative;
    transition: background 0.2s;
  }

  .streak-toggle.active .toggle-track {
    background: var(--color-accent-mint-strong);
  }

  .toggle-thumb {
    position: absolute;
    top: 2px;
    left: 2px;
    width: 20px;
    height: 20px;
    background: var(--color-white);
    border-radius: 50%;
    transition: transform 0.2s;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
  }

  .streak-toggle.active .toggle-thumb {
    transform: translateX(20px);
  }
</style>
