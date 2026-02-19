<script lang="ts">
  import type { TodoItem, Tag } from '../types';
  import { i18n } from '$lib/i18n';
  import TagChip from './TagChip.svelte';
  import { getAppByKey, getAppLabel, openApp } from '$lib/linkedApps';

  interface Props {
    item: TodoItem;
    itemTags?: Tag[];
    onOpenEdit: (item: TodoItem) => void;
  }

  let { item, itemTags = [], onOpenEdit }: Props = $props();

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
  <div class="view-container">
    {#if item.memo || item.repeat_type !== 'none' || item.track_streak || item.reminder_at || item.linked_app || itemTags.length > 0}
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
          <p class="info-row">
            <span class="info-icon sky">
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
          <p class="info-row">
            <span class="info-icon peach">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M17.657 18.657A8 8 0 016.343 7.343S7 9 9 10c0-2 .5-5 2.986-7C14 5 16.09 5.777 17.656 7.343A7.975 7.975 0 0120 13a7.975 7.975 0 01-2.343 5.657z" />
                <path d="M9.879 16.121A3 3 0 1012.015 11L11 14H9c0 .768.293 1.536.879 2.121z" />
              </svg>
            </span>
            {i18n.t('trackingStreak')}
          </p>
        {/if}
        {#if item.reminder_at}
          <p class="info-row">
            <span class="info-icon sky">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M6 8a6 6 0 0 1 12 0c0 7 3 9 3 9H3s3-2 3-9"></path>
                <path d="M10.3 21a1.94 1.94 0 0 0 3.4 0"></path>
              </svg>
            </span>
            {item.reminder_at}
          </p>
        {/if}
        {#if item.linked_app}
          {@const appInfo = getAppByKey(item.linked_app)}
          {#if appInfo}
            <div class="linked-app-row">
              <span class="linked-app-icon">{appInfo.icon}</span>
              <span class="linked-app-name">{getAppLabel(item.linked_app, i18n.locale)}</span>
              <button
                type="button"
                class="linked-app-open-btn"
                onclick={() => openApp(item.linked_app!)}
              >
                {i18n.t('linkedAppOpen')}
              </button>
            </div>
          {/if}
        {/if}
      </div>
    {/if}

    <div class="actions">
      <button
        type="button"
        class="btn-edit"
        onclick={() => onOpenEdit(item)}
        title={i18n.t('edit')}
      >
        <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z"></path>
          <path d="m15 5 4 4"></path>
        </svg>
      </button>
    </div>
  </div>
</div>

<style>
  .memo-drawer {
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .view-container {
    display: flex;
    flex-direction: column;
    gap: 10px;
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

  .info-row {
    margin: 2px 0 0 0;
    font-size: 13px;
    color: var(--color-ink-muted);
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .info-icon {
    display: flex;
    align-items: center;
  }

  .info-icon.sky {
    color: var(--color-accent-sky-strong);
  }

  .info-icon.peach {
    color: var(--color-accent-peach-strong);
  }

  .tags-display {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    margin-bottom: 2px;
  }

  .linked-app-row {
    display: flex;
    align-items: center;
    gap: 6px;
    margin-top: 2px;
  }

  .linked-app-icon {
    font-size: 14px;
  }

  .linked-app-name {
    font-size: 13px;
    color: var(--color-ink-muted);
    flex: 1;
  }

  .linked-app-open-btn {
    padding: 4px 12px;
    border: 1px solid var(--color-accent-sky-strong);
    border-radius: 12px;
    background: var(--color-accent-sky);
    color: var(--color-ink);
    font-size: 12px;
    cursor: pointer;
    transition: background-color 0.15s;
  }

  .linked-app-open-btn:active {
    background: var(--color-accent-sky-strong);
  }

  .actions {
    display: flex;
    justify-content: flex-end;
  }

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
    background: var(--color-accent-sky);
    color: var(--color-ink);
  }

  .btn-edit:hover {
    background: var(--color-accent-sky-strong);
  }

  .icon {
    width: 18px;
    height: 18px;
  }
</style>
