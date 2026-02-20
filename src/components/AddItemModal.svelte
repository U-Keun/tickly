<script lang="ts">
  import type { RepeatType, Tag } from '../types';
  import { i18n } from '$lib/i18n';
  import { stringifyRepeatDetail } from '$lib/repeatDetail';
  import ModalWrapper from './ModalWrapper.svelte';
  import RepeatSelector from './RepeatSelector.svelte';
  import TagInput from './TagInput.svelte';
  import LinkedAppModal from './LinkedAppModal.svelte';
  import { getAppByKey, getAppLabel } from '$lib/linkedApps';

  type MaybePromise = void | Promise<void>;

  interface Props {
    show: boolean;
    allTags?: Tag[];
    onAdd: (
      text: string,
      memo: string | null,
      repeatType: RepeatType,
      repeatDetail: string | null,
      trackStreak: boolean,
      tagNames?: string[],
      reminderAt?: string | null,
      linkedApp?: string | null
    ) => MaybePromise;
    onCancel: () => MaybePromise;
  }

  let { show, allTags = [], onAdd, onCancel }: Props = $props();

  let text = $state('');
  let memo = $state('');
  let repeatType = $state<RepeatType>('none');
  let repeatDetail = $state<number[]>([]);
  let trackStreak = $state(false);
  let reminderTime = $state('');
  let linkedApp = $state<string | null>(null);
  let showAdvanced = $state(false);
  let showLinkedAppModal = $state(false);
  let pendingTags = $state<Tag[]>([]);
  let nextLocalTagId = $state(-1);
  let isSaving = $state(false);
  let textInputElement = $state<HTMLInputElement | null>(null);
  let activeAdvancedCount = $derived(
    (repeatType !== 'none' ? 1 : 0) +
    (trackStreak ? 1 : 0) +
    (reminderTime ? 1 : 0) +
    (pendingTags.length > 0 ? 1 : 0) +
    (linkedApp ? 1 : 0)
  );

  $effect(() => {
    if (show) {
      text = '';
      memo = '';
      repeatType = 'none';
      repeatDetail = [];
      trackStreak = false;
      reminderTime = '';
      linkedApp = null;
      showAdvanced = false;
      showLinkedAppModal = false;
      pendingTags = [];
      nextLocalTagId = -1;
      isSaving = false;
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

  async function handleSubmit() {
    if (isSaving) return;
    const trimmedText = text.trim();
    if (!trimmedText) return;

    isSaving = true;
    try {
      const trimmedMemo = memo.trim() || null;
      const repeatDetailJson = stringifyRepeatDetail(repeatDetail);
      const tagNames = pendingTags.map(t => t.name);
      const reminderAt = reminderTime || null;
      await onAdd(trimmedText, trimmedMemo, repeatType, repeatDetailJson, trackStreak, tagNames, reminderAt, linkedApp);
      await onCancel();
    } finally {
      isSaving = false;
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    const targetIsInput = event.target instanceof HTMLInputElement;
    if (event.key === 'Enter' && (targetIsInput || event.metaKey || event.ctrlKey)) {
      event.preventDefault();
      handleSubmit();
    }
  }
</script>

<ModalWrapper {show} onClose={onCancel} position="topCompact">
  <div class="modal-top">
    <h3 class="modal-title">{i18n.t('addItemTitle')}</h3>

    <div class="form-group">
      <input
        bind:this={textInputElement}
        bind:value={text}
        onkeydown={handleKeydown}
        id="item-text"
        type="text"
        class="form-input"
        placeholder={i18n.t('todoPlaceholder')}
        aria-label={i18n.t('todoLabel')}
        autocomplete="off"
      />
    </div>

    <div class="form-group">
      <textarea
        bind:value={memo}
        onkeydown={handleKeydown}
        id="item-memo"
        class="form-textarea"
        placeholder={i18n.t('memoPlaceholder')}
        aria-label={i18n.t('memoLabel')}
        rows="3"
      ></textarea>
    </div>

    <div class="advanced-head">
      <button
        type="button"
        class="advanced-toggle"
        onclick={() => showAdvanced = !showAdvanced}
      >
        <span class="advanced-toggle-left">
          <span class="advanced-toggle-text">{i18n.t('advancedSettings')}</span>
          {#if activeAdvancedCount > 0}
            <span class="advanced-count">{activeAdvancedCount}</span>
          {/if}
        </span>
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
    </div>
  </div>

  <div class="advanced-panel" class:open={showAdvanced} aria-hidden={!showAdvanced} inert={!showAdvanced}>
    <div class="modal-scrollable">
      <div class="advanced-section">
        <div class="form-group">
          <RepeatSelector
            {repeatType}
            {repeatDetail}
            onRepeatTypeChange={(t) => repeatType = t}
            onRepeatDetailChange={(d) => repeatDetail = d}
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
              aria-label="Toggle streak tracking"
            >
              <span class="toggle-track">
                <span class="toggle-thumb"></span>
              </span>
            </button>
          </label>
        </div>

        <hr class="advanced-divider" />

        <div class="form-group reminder-section">
          <span class="form-label">{i18n.t('reminder')}</span>
          <div class="reminder-row">
            <input
              type="time"
              class="reminder-input"
              bind:value={reminderTime}
            />
            {#if reminderTime}
              <button
                type="button"
                class="reminder-clear"
                onclick={() => reminderTime = ''}
                title={i18n.t('reminderClear')}
              >
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <line x1="18" y1="6" x2="6" y2="18"></line>
                  <line x1="6" y1="6" x2="18" y2="18"></line>
                </svg>
              </button>
            {/if}
          </div>
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

        <hr class="advanced-divider" />

        <div class="form-group">
          <span class="form-label">{i18n.t('linkedApp')}</span>
          {#if linkedApp}
            {@const currentApp = getAppByKey(linkedApp)}
            {#if currentApp}
              <div class="linked-app-current">
                <span class="linked-app-icon">{currentApp.icon}</span>
                <span class="linked-app-name">{getAppLabel(linkedApp, i18n.locale)}</span>
                <button
                  type="button"
                  class="linked-app-change-btn"
                  onclick={() => showLinkedAppModal = true}
                >{i18n.t('edit')}</button>
                <button
                  type="button"
                  class="linked-app-remove-btn"
                  onclick={() => linkedApp = null}
                  aria-label="Remove linked app"
                >
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <line x1="18" y1="6" x2="6" y2="18"></line>
                    <line x1="6" y1="6" x2="18" y2="18"></line>
                  </svg>
                </button>
              </div>
            {/if}
          {:else}
            <button
              type="button"
              class="linked-app-connect-btn"
              onclick={() => showLinkedAppModal = true}
            >
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"></path>
                <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"></path>
              </svg>
              {i18n.t('linkedAppConnect')}
            </button>
          {/if}
        </div>
      </div>
    </div>
  </div>

  <div class="button-group">
    <button type="button" class="btn btn-cancel" onclick={onCancel}>
      {i18n.t('cancel')}
    </button>
    <button
      type="button"
      class="btn btn-primary"
      onclick={handleSubmit}
      disabled={!text.trim() || isSaving}
    >
      {isSaving ? i18n.t('saving') : i18n.t('add')}
    </button>
  </div>

  <LinkedAppModal
    show={showLinkedAppModal}
    selectedKey={linkedApp}
    onSelect={(key) => { linkedApp = key; }}
    onClose={() => showLinkedAppModal = false}
  />
</ModalWrapper>

<style>
  .modal-top {
    flex-shrink: 0;
  }

  .modal-title {
    font-size: 18px;
    font-weight: 600;
    margin-bottom: 16px;
    color: var(--color-ink);
  }

  .form-group {
    margin-bottom: 14px;
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
    resize: vertical;
    min-height: 88px;
    line-height: 1.45;
  }

  .form-input::placeholder,
  .form-textarea::placeholder {
    color: var(--color-ink-muted);
  }

  .advanced-head {
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin-bottom: 2px;
  }

  .advanced-toggle {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: 10px 12px;
    background: none;
    border: 1px solid var(--color-stroke);
    border-radius: 10px;
    cursor: pointer;
    color: var(--color-ink-muted);
    font-size: 14px;
    transition: background-color 0.2s;
  }

  .advanced-toggle:hover {
    background: var(--color-canvas);
  }

  .advanced-toggle-left {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .advanced-toggle-text {
    font-size: 14px;
  }

  .advanced-count {
    min-width: 20px;
    height: 20px;
    padding: 0 6px;
    border-radius: 999px;
    background: var(--color-accent-sky);
    color: var(--color-ink);
    font-size: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .advanced-toggle-icon {
    transition: transform 0.25s ease;
  }

  .advanced-toggle-icon.rotated {
    transform: rotate(180deg);
  }

  .advanced-panel {
    display: grid;
    grid-template-rows: 0fr;
    overflow: hidden;
    transition: grid-template-rows 360ms cubic-bezier(0.22, 1, 0.36, 1);
  }

  .advanced-panel.open {
    grid-template-rows: 1fr;
  }

  .modal-scrollable {
    min-height: 0;
    overflow-y: auto;
    -webkit-overflow-scrolling: touch;
    max-height: 42vh;
    margin: 0 -24px;
    padding: 4px 24px 8px;
    opacity: 0;
    transform: translateY(-6px);
    transition:
      opacity 280ms cubic-bezier(0.22, 1, 0.36, 1),
      transform 360ms cubic-bezier(0.22, 1, 0.36, 1);
    pointer-events: none;
    will-change: opacity, transform;
  }

  .advanced-panel.open > .modal-scrollable {
    opacity: 1;
    transform: translateY(0);
    pointer-events: auto;
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

  .reminder-section {
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

  .reminder-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .reminder-input {
    flex: 1;
    padding: 10px 12px;
    border: 2px solid var(--color-stroke);
    border-radius: 10px;
    font-size: 14px;
    background: var(--color-white);
    color: var(--color-ink);
  }

  .reminder-input:focus {
    outline: none;
    border-color: var(--color-accent-sky);
  }

  .reminder-clear {
    width: 32px;
    height: 32px;
    padding: 0;
    border: none;
    border-radius: 50%;
    background: var(--color-canvas);
    color: var(--color-ink-muted);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background-color 0.2s;
  }

  .reminder-clear:hover {
    background: var(--color-mist);
  }

  .linked-app-connect-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    width: 100%;
    padding: 10px 12px;
    border: 1px dashed var(--color-stroke);
    border-radius: 10px;
    background: var(--color-white);
    color: var(--color-ink-muted);
    font-size: 14px;
    cursor: pointer;
    transition: background-color 0.15s;
  }

  .linked-app-connect-btn:hover {
    background: var(--color-canvas);
  }

  .linked-app-current {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 10px 12px;
    border: 1px solid var(--color-accent-sky-strong);
    border-radius: 10px;
    background: var(--color-white);
  }

  .linked-app-icon {
    font-size: 16px;
    flex-shrink: 0;
  }

  .linked-app-name {
    flex: 1;
    font-size: 14px;
    color: var(--color-ink);
  }

  .linked-app-change-btn {
    min-height: 32px;
    padding: 4px 10px;
    border: 1px solid var(--color-accent-sky-strong);
    border-radius: 8px;
    background: none;
    color: var(--color-ink);
    font-size: 13px;
    cursor: pointer;
    transition: background-color 0.15s;
  }

  .linked-app-change-btn:hover {
    background: var(--color-accent-sky);
  }

  .linked-app-remove-btn {
    width: 28px;
    height: 28px;
    padding: 0;
    border: none;
    border-radius: 50%;
    background: none;
    color: var(--color-ink-muted);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background-color 0.15s;
  }

  .linked-app-remove-btn:hover {
    background: var(--color-canvas);
  }

  .button-group {
    display: flex;
    gap: 12px;
    justify-content: flex-end;
    padding-top: 16px;
    margin-top: 8px;
    flex-shrink: 0;
    border-top: 1px solid var(--color-stroke);
  }

  .btn {
    min-height: 44px;
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
