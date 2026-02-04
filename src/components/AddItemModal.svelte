<script lang="ts">
  import { slide } from 'svelte/transition';
  import { cubicOut } from 'svelte/easing';
  import { i18n } from '$lib/i18n';
  import type { RepeatType, Tag } from '../types';
  import ModalWrapper from './ModalWrapper.svelte';
  import RepeatSelector from './RepeatSelector.svelte';
  import TagInput from './TagInput.svelte';

  interface Props {
    show: boolean;
    allTags?: Tag[];
    onAdd: (text: string, memo: string | null, repeatType: RepeatType, repeatDetail: string | null, trackStreak: boolean, tagNames?: string[]) => void;
    onCancel: () => void;
  }

  let { show, allTags = [], onAdd, onCancel }: Props = $props();

  let text = $state('');
  let memo = $state('');
  let repeatType = $state<RepeatType>('none');
  let repeatDetail = $state<number[]>([]);
  let trackStreak = $state(false);
  let showAdvanced = $state(false);
  let pendingTags = $state<Tag[]>([]);
  let nextLocalTagId = $state(-1);
  let textInputElement = $state<HTMLInputElement | null>(null);

  // Reset and focus when modal opens
  $effect(() => {
    if (show) {
      text = '';
      memo = '';
      repeatType = 'none';
      repeatDetail = [];
      trackStreak = false;
      showAdvanced = false;
      pendingTags = [];
      nextLocalTagId = -1;
      setTimeout(() => textInputElement?.focus(), 100);
    }
  });

  function handleAddPendingTag(tagName: string) {
    if (pendingTags.find(t => t.name === tagName)) return;
    pendingTags = [...pendingTags, { id: nextLocalTagId--, name: tagName } as Tag];
  }

  function handleRemovePendingTag(tagId: number) {
    pendingTags = pendingTags.filter(t => t.id !== tagId);
  }

  function handleSubmit() {
    const trimmedText = text.trim();
    if (!trimmedText) return;

    const trimmedMemo = memo.trim() || null;
    const repeatDetailJson = repeatDetail.length > 0 ? JSON.stringify(repeatDetail) : null;
    const tagNames = pendingTags.map(t => t.name);
    onAdd(trimmedText, trimmedMemo, repeatType, repeatDetailJson, trackStreak, tagNames);
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

  <button
    type="button"
    class="advanced-toggle"
    onclick={() => showAdvanced = !showAdvanced}
  >
    <span class="advanced-toggle-text">{i18n.t('advancedSettings')}</span>
    <svg
      class="advanced-toggle-icon"
      class:rotated={showAdvanced}
      width="16" height="16" viewBox="0 0 24 24"
      fill="none" stroke="currentColor" stroke-width="2"
      stroke-linecap="round" stroke-linejoin="round"
    >
      <polyline points="6 9 12 15 18 9"></polyline>
    </svg>
  </button>
  {#if showAdvanced}
    <div class="advanced-section" transition:slide={{ duration: 250, easing: cubicOut }}>
      <div class="form-group">
        <RepeatSelector
          {repeatType}
          {repeatDetail}
          onRepeatTypeChange={handleRepeatTypeChange}
          onRepeatDetailChange={handleRepeatDetailChange}
        />
      </div>

      <hr class="advanced-divider" />

      <div class="form-group-section streak-toggle-section">
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

      <hr class="advanced-divider" />

      <div class="form-group">
        <span class="form-label">{i18n.t('tags')}</span>
        <TagInput
          currentTags={pendingTags}
          {allTags}
          onAdd={handleAddPendingTag}
          onRemove={handleRemovePendingTag}
        />
      </div>
    </div>
  {/if}

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

  .advanced-toggle {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 4px;
    width: 100%;
    padding: 10px;
    background: none;
    border: 1px solid var(--color-stroke);
    border-radius: 10px;
    cursor: pointer;
    color: var(--color-ink-muted);
    font-size: 14px;
    transition: background-color 0.2s;
    margin-bottom: 4px;
  }

  .advanced-toggle:hover {
    background: var(--color-canvas);
  }

  .advanced-toggle-text {
    font-size: 14px;
  }

  .advanced-toggle-icon {
    transition: transform 0.25s ease;
  }

  .advanced-toggle-icon.rotated {
    transform: rotate(180deg);
  }

  .advanced-section {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .advanced-divider {
    border: none;
    border-top: 1px solid var(--color-stroke);
    margin: 4px 0;
  }

  .streak-toggle-section {
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
    background: var(--color-mist);
    border-radius: 12px;
    position: relative;
    transition: background-color 0.2s;
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
