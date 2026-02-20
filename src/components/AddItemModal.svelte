<script lang="ts">
  import type { RepeatType, Tag } from '../types';
  import { i18n } from '$lib/i18n';
  import { stringifyRepeatDetail } from '$lib/repeatDetail';
  import ModalWrapper from './ModalWrapper.svelte';
  import ItemAdvancedSection from './ItemAdvancedSection.svelte';

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

    <ItemAdvancedSection
      {show}
      {repeatType}
      {repeatDetail}
      {trackStreak}
      {reminderTime}
      {linkedApp}
      {allTags}
      tags={pendingTags}
      activeCount={activeAdvancedCount}
      onRepeatTypeChange={(value) => repeatType = value}
      onRepeatDetailChange={(value) => repeatDetail = value}
      onTrackStreakChange={(value) => trackStreak = value}
      onReminderTimeChange={(value) => reminderTime = value}
      onLinkedAppChange={(value) => linkedApp = value}
      onAddTag={handleAddPendingTag}
      onRemoveTag={handleRemovePendingTag}
    />
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
