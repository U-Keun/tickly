<script lang="ts">
  import type { TodoItem, RepeatType, Tag } from '../types';
  import { i18n } from '$lib/i18n';
  import { parseRepeatDetail, stringifyRepeatDetail } from '$lib/repeatDetail';
  import ModalWrapper from './ModalWrapper.svelte';
  import ItemAdvancedSection from './ItemAdvancedSection.svelte';

  type MaybePromise = void | Promise<void>;

  interface Props {
    show: boolean;
    item: TodoItem;
    itemTags: Tag[];
    allTags: Tag[];
    onSaveMemo: (id: number, memo: string | null) => MaybePromise;
    onEditText: (id: number, text: string) => MaybePromise;
    onUpdateRepeat: (id: number, repeatType: RepeatType, repeatDetail: string | null) => MaybePromise;
    onUpdateTrackStreak: (id: number, trackStreak: boolean) => MaybePromise;
    onUpdateReminder: (id: number, reminderAt: string | null) => MaybePromise;
    onUpdateLinkedApp: (id: number, linkedApp: string | null) => MaybePromise;
    onAddTag?: (itemId: number, tagName: string) => void;
    onRemoveTag?: (itemId: number, tagId: number) => void;
    onSave: () => MaybePromise;
    onCancel: () => MaybePromise;
  }

  let {
    show, item, itemTags, allTags,
    onSaveMemo, onEditText, onUpdateRepeat, onUpdateTrackStreak,
    onUpdateReminder, onUpdateLinkedApp, onAddTag, onRemoveTag,
    onSave, onCancel,
  }: Props = $props();

  let editText = $state('');
  let memoText = $state('');
  let repeatType = $state<RepeatType>('none');
  let repeatDetail = $state<number[]>([]);
  let trackStreak = $state(false);
  let reminderTime = $state('');
  let linkedApp = $state<string | null>(null);
  let isSaving = $state(false);
  let textInputElement = $state<HTMLInputElement | null>(null);
  let activeAdvancedCount = $derived(
    (repeatType !== 'none' ? 1 : 0) +
    (trackStreak ? 1 : 0) +
    (reminderTime ? 1 : 0) +
    (itemTags.length > 0 ? 1 : 0) +
    (linkedApp ? 1 : 0)
  );

  $effect(() => {
    if (show) {
      editText = item.text;
      memoText = item.memo || '';
      repeatType = item.repeat_type;
      repeatDetail = parseRepeatDetail(item.repeat_detail);
      trackStreak = item.track_streak;
      reminderTime = item.reminder_at || '';
      linkedApp = item.linked_app || null;
      setTimeout(() => textInputElement?.focus(), 100);
    }
  });

  async function handleSave() {
    if (isSaving) return;
    const trimmedText = editText.trim();
    if (!trimmedText) return;
    isSaving = true;
    try {
      const updates: MaybePromise[] = [];

      if (trimmedText !== item.text) {
        updates.push(onEditText(item.id, trimmedText));
      }

      const trimmedMemo = memoText.trim() || null;
      if (trimmedMemo !== item.memo) {
        updates.push(onSaveMemo(item.id, trimmedMemo));
      }

      const newRepeatDetail = stringifyRepeatDetail(repeatDetail);
      if (repeatType !== item.repeat_type || newRepeatDetail !== item.repeat_detail) {
        updates.push(onUpdateRepeat(item.id, repeatType, newRepeatDetail));
      }

      if (trackStreak !== item.track_streak) {
        updates.push(onUpdateTrackStreak(item.id, trackStreak));
      }

      const newReminderAt = reminderTime || null;
      if (newReminderAt !== (item.reminder_at || null)) {
        updates.push(onUpdateReminder(item.id, newReminderAt));
      }

      if (linkedApp !== (item.linked_app || null)) {
        updates.push(onUpdateLinkedApp(item.id, linkedApp));
      }

      await Promise.all(updates);
      await onSave();
    } finally {
      isSaving = false;
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    const targetIsInput = event.target instanceof HTMLInputElement;
    if (event.key === 'Enter' && (targetIsInput || event.metaKey || event.ctrlKey)) {
      event.preventDefault();
      handleSave();
    }
  }
</script>

<ModalWrapper {show} onClose={onCancel} position="topCompact">
  <div class="modal-top">
    <h3 class="modal-title">{i18n.t('editItemTitle')}</h3>

    <div class="form-group">
      <input
        bind:this={textInputElement}
        bind:value={editText}
        onkeydown={handleKeydown}
        id="edit-text"
        type="text"
        class="form-input"
        placeholder={i18n.t('todoPlaceholder')}
        aria-label={i18n.t('todoLabel')}
        autocomplete="off"
      />
    </div>

    <div class="form-group">
      <textarea
        bind:value={memoText}
        onkeydown={handleKeydown}
        id="edit-memo"
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
      tags={itemTags}
      showTags={Boolean(onAddTag && onRemoveTag)}
      activeCount={activeAdvancedCount}
      onRepeatTypeChange={(value) => repeatType = value}
      onRepeatDetailChange={(value) => repeatDetail = value}
      onTrackStreakChange={(value) => trackStreak = value}
      onReminderTimeChange={(value) => reminderTime = value}
      onLinkedAppChange={(value) => linkedApp = value}
      onAddTag={onAddTag ? (name) => onAddTag(item.id, name) : undefined}
      onRemoveTag={onRemoveTag ? (tagId) => onRemoveTag(item.id, tagId) : undefined}
    />
  </div>
  <div class="button-group">
    <button type="button" class="btn btn-cancel" onclick={onCancel}>
      {i18n.t('cancel')}
    </button>
    <button
      type="button"
      class="btn btn-primary"
      onclick={handleSave}
      disabled={!editText.trim() || isSaving}
    >
      {isSaving ? i18n.t('saving') : i18n.t('save')}
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
