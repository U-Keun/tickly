<script lang="ts">
  import { slide } from 'svelte/transition';
  import { cubicOut } from 'svelte/easing';
  import type { TodoItem, RepeatType, Tag } from '../types';
  import { i18n } from '$lib/i18n';
  import RepeatSelector from './RepeatSelector.svelte';
  import TagInput from './TagInput.svelte';
  import TagChip from './TagChip.svelte';

  interface Props {
    item: TodoItem;
    itemTags?: Tag[];
    allTags?: Tag[];
    onSaveMemo: (id: number, memo: string | null) => void;
    onEditText: (id: number, text: string) => void;
    onUpdateRepeat: (id: number, repeatType: RepeatType, repeatDetail: string | null) => void;
    onUpdateTrackStreak: (id: number, trackStreak: boolean) => void;
    onAddTag?: (itemId: number, tagName: string) => void;
    onRemoveTag?: (itemId: number, tagId: number) => void;
    onEditModeChange?: (editing: boolean) => void;
    closeDrawer: () => void;
  }

  let { item, itemTags = [], allTags = [], onSaveMemo, onEditText, onUpdateRepeat, onUpdateTrackStreak, onAddTag, onRemoveTag, onEditModeChange, closeDrawer }: Props = $props();

  let isEditMode = $state(false);
  let editText = $state('');
  let memoText = $state('');
  let repeatType = $state<RepeatType>('none');
  let repeatDetail = $state<number[]>([]);
  let trackStreak = $state(false);
  let isSaving = $state(false);
  let showAdvanced = $state(false);

  // Sync texts when item changes
  $effect(() => {
    editText = item.text;
    memoText = item.memo || '';
    repeatType = item.repeat_type;
    repeatDetail = item.repeat_detail ? JSON.parse(item.repeat_detail) : [];
    trackStreak = item.track_streak;
  });

  function enterEditMode() {
    isEditMode = true;
    onEditModeChange?.(true);
  }

  function cancelEdit() {
    editText = item.text;
    memoText = item.memo || '';
    repeatType = item.repeat_type;
    repeatDetail = item.repeat_detail ? JSON.parse(item.repeat_detail) : [];
    trackStreak = item.track_streak;
    showAdvanced = false;
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

    // Save repeat settings if changed
    const newRepeatDetail = repeatDetail.length > 0 ? JSON.stringify(repeatDetail) : null;
    if (repeatType !== item.repeat_type || newRepeatDetail !== item.repeat_detail) {
      onUpdateRepeat(item.id, repeatType, newRepeatDetail);
    }

    // Save track_streak if changed
    if (trackStreak !== item.track_streak) {
      onUpdateTrackStreak(item.id, trackStreak);
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

  function handleRepeatTypeChange(type: RepeatType) {
    repeatType = type;
  }

  function handleRepeatDetailChange(detail: number[]) {
    repeatDetail = detail;
  }

  // Helper to get repeat display text
  function getRepeatDisplayText(): string {
    if (item.repeat_type === 'none') return i18n.t('repeatNone');
    if (item.repeat_type === 'daily') return i18n.t('repeatDaily');
    if (item.repeat_type === 'weekly') {
      const days = item.repeat_detail ? JSON.parse(item.repeat_detail) as number[] : [];
      const dayNames: Array<'sun' | 'mon' | 'tue' | 'wed' | 'thu' | 'fri' | 'sat'> = ['sun', 'mon', 'tue', 'wed', 'thu', 'fri', 'sat'];
      const dayLabels = days.map(d => i18n.t(dayNames[d])).join(', ');
      return `${i18n.t('repeatWeekly')} (${dayLabels})`;
    }
    if (item.repeat_type === 'monthly') {
      const dates = item.repeat_detail ? JSON.parse(item.repeat_detail) as number[] : [];
      return `${i18n.t('repeatMonthly')} (${dates.join(', ')})`;
    }
    return '';
  }
</script>

<div class="memo-drawer">
  {#if isEditMode}
    <!-- Edit Mode -->
    <div class="edit-container" transition:slide={{ duration: 300, easing: cubicOut }}>
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
          <div class="edit-section">
            <RepeatSelector
              {repeatType}
              {repeatDetail}
              onRepeatTypeChange={handleRepeatTypeChange}
              onRepeatDetailChange={handleRepeatDetailChange}
            />
          </div>
          <hr class="advanced-divider" />
          <div class="edit-section streak-toggle-section">
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
          {#if onAddTag && onRemoveTag}
            <hr class="advanced-divider" />
            <div class="edit-section">
              <span class="section-label">{i18n.t('tags')}</span>
              <TagInput
                currentTags={itemTags}
                {allTags}
                onAdd={(name) => onAddTag?.(item.id, name)}
                onRemove={(tagId) => onRemoveTag?.(item.id, tagId)}
              />
            </div>
          {/if}
        </div>
      {/if}
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
    </div>
  {:else}
    <!-- View Mode -->
    <div class="view-container">
      {#if item.memo || item.repeat_type !== 'none' || item.track_streak || itemTags.length > 0}
        <div class="memo-display">
          {#if itemTags.length > 0}
            <div class="tags-display">
              {#each itemTags as tag (tag.id)}
                <TagChip name={tag.name} />
              {/each}
            </div>
          {/if}
          {#if item.memo}
            <p class="memo-text">{item.memo}</p>
          {/if}
          {#if item.repeat_type !== 'none'}
            <p class="repeat-info">
              <span class="repeat-icon">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M17 2.1l4 4-4 4"></path>
                  <path d="M3 12.2v-2a4 4 0 0 1 4-4h12.8M7 21.9l-4-4 4-4"></path>
                  <path d="M21 11.8v2a4 4 0 0 1-4 4H4.2"></path>
                </svg>
              </span>
              {getRepeatDisplayText()}
            </p>
          {/if}
          {#if item.track_streak}
            <p class="streak-info">
              <span class="streak-icon">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M17.657 18.657A8 8 0 016.343 7.343S7 9 9 10c0-2 .5-5 2.986-7C14 5 16.09 5.777 17.656 7.343A7.975 7.975 0 0120 13a7.975 7.975 0 01-2.343 5.657z" />
                  <path d="M9.879 16.121A3 3 0 1012.015 11L11 14H9c0 .768.293 1.536.879 2.121z" />
                </svg>
              </span>
              {i18n.t('trackingStreak')}
            </p>
          {/if}
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
    </div>
  {/if}
</div>

<style>
  .memo-drawer {
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .edit-container {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .view-container {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .edit-section {
    display: flex;
    flex-direction: column;
    gap: 6px;
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

  .repeat-info {
    margin: 6px 0 0 0;
    font-size: 13px;
    color: var(--color-ink-muted);
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .repeat-icon {
    display: flex;
    align-items: center;
    color: var(--color-accent-sky-strong);
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

  .advanced-toggle {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 4px;
    width: 100%;
    padding: 8px;
    background: none;
    border: 1px solid var(--color-stroke);
    border-radius: 8px;
    cursor: pointer;
    color: var(--color-ink-muted);
    font-size: 13px;
    transition: background-color 0.2s;
  }

  .advanced-toggle:hover {
    background: var(--color-canvas);
  }

  .advanced-toggle-text {
    font-size: 13px;
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
    gap: 10px;
  }

  .advanced-divider {
    border: none;
    border-top: 1px solid var(--color-stroke);
    margin: 2px 0;
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

  .section-label {
    font-size: 14px;
    color: var(--color-ink);
    font-weight: 500;
  }

  .tags-display {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    margin-bottom: 2px;
  }

  .streak-info {
    margin: 6px 0 0 0;
    font-size: 13px;
    color: var(--color-ink-muted);
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .streak-icon {
    display: flex;
    align-items: center;
    color: var(--color-accent-peach-strong);
  }
</style>
