<script lang="ts">
  import type { RepeatType, Tag } from '../types';
  import { i18n } from '$lib/i18n';
  import { getAppByKey, getAppLabel } from '$lib/linkedApps';
  import LinkedAppModal from './LinkedAppModal.svelte';
  import RepeatSelector from './RepeatSelector.svelte';
  import TagInput from './TagInput.svelte';

  interface Props {
    show: boolean;
    repeatType: RepeatType;
    repeatDetail: number[];
    trackStreak: boolean;
    reminderTime: string;
    linkedApp: string | null;
    allTags?: Tag[];
    tags?: Tag[];
    activeCount?: number;
    showTags?: boolean;
    onRepeatTypeChange: (repeatType: RepeatType) => void;
    onRepeatDetailChange: (repeatDetail: number[]) => void;
    onTrackStreakChange: (trackStreak: boolean) => void;
    onReminderTimeChange: (reminderTime: string) => void;
    onLinkedAppChange: (linkedApp: string | null) => void;
    onAddTag?: (tagName: string) => void;
    onRemoveTag?: (tagId: number) => void;
  }

  let {
    show,
    repeatType,
    repeatDetail,
    trackStreak,
    reminderTime,
    linkedApp,
    allTags = [],
    tags = [],
    activeCount = 0,
    showTags = true,
    onRepeatTypeChange,
    onRepeatDetailChange,
    onTrackStreakChange,
    onReminderTimeChange,
    onLinkedAppChange,
    onAddTag,
    onRemoveTag,
  }: Props = $props();

  let showAdvanced = $state(false);
  let showLinkedAppModal = $state(false);
  let canEditTags = $derived(showTags && Boolean(onAddTag && onRemoveTag));

  $effect(() => {
    if (show) {
      showAdvanced = false;
      showLinkedAppModal = false;
    }
  });
</script>

<div class="advanced-head">
  <button
    type="button"
    class="advanced-toggle"
    onclick={() => showAdvanced = !showAdvanced}
  >
    <span class="advanced-toggle-left">
      <span class="advanced-toggle-text">{i18n.t('advancedSettings')}</span>
      {#if activeCount > 0}
        <span class="advanced-count">{activeCount}</span>
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

<div class="advanced-panel" class:open={showAdvanced} aria-hidden={!showAdvanced} inert={!showAdvanced}>
  <div class="modal-scrollable">
    <div class="advanced-section">
      <div class="form-group">
        <RepeatSelector
          {repeatType}
          {repeatDetail}
          onRepeatTypeChange={onRepeatTypeChange}
          onRepeatDetailChange={onRepeatDetailChange}
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
            onclick={() => onTrackStreakChange(!trackStreak)}
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
            value={reminderTime}
            oninput={(event) => onReminderTimeChange((event.currentTarget as HTMLInputElement).value)}
          />
          {#if reminderTime}
            <button
              type="button"
              class="reminder-clear"
              onclick={() => onReminderTimeChange('')}
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

      {#if canEditTags}
        <hr class="advanced-divider" />

        <div class="form-group">
          <span class="form-label">{i18n.t('tags')}</span>
          <TagInput
            currentTags={tags}
            {allTags}
            onAdd={(name) => onAddTag?.(name)}
            onRemove={(tagId) => onRemoveTag?.(tagId)}
          />
        </div>
      {/if}

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
                onclick={() => onLinkedAppChange(null)}
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

<LinkedAppModal
  show={showLinkedAppModal}
  selectedKey={linkedApp}
  onSelect={(key) => onLinkedAppChange(key)}
  onClose={() => showLinkedAppModal = false}
/>

<style>
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
</style>
