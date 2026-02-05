<script lang="ts">
  import { slide } from 'svelte/transition';
  import { cubicOut } from 'svelte/easing';
  import type { RepeatType } from '../types';
  import { i18n, type TranslationKey } from '$lib/i18n';

  interface Props {
    repeatType: RepeatType;
    repeatDetail: number[];
    onRepeatTypeChange: (type: RepeatType) => void;
    onRepeatDetailChange: (detail: number[]) => void;
  }

  let { repeatType, repeatDetail, onRepeatTypeChange, onRepeatDetailChange }: Props = $props();

  // Local state for controlling transitions
  let localType = $state<RepeatType>('none');

  // Sync local state with prop
  $effect(() => {
    localType = repeatType;
  });

  const repeatTypes: { value: RepeatType; labelKey: TranslationKey }[] = [
    { value: 'none', labelKey: 'repeatNone' },
    { value: 'daily', labelKey: 'repeatDaily' },
    { value: 'weekly', labelKey: 'repeatWeekly' },
    { value: 'monthly', labelKey: 'repeatMonthly' },
  ];

  // Weekdays: 0 = Sunday, 1 = Monday, ..., 6 = Saturday
  const weekdays: { value: number; labelKey: TranslationKey }[] = [
    { value: 0, labelKey: 'sun' },
    { value: 1, labelKey: 'mon' },
    { value: 2, labelKey: 'tue' },
    { value: 3, labelKey: 'wed' },
    { value: 4, labelKey: 'thu' },
    { value: 5, labelKey: 'fri' },
    { value: 6, labelKey: 'sat' },
  ];

  // Monthly days: 1-31
  const monthlyDays = Array.from({ length: 31 }, (_, i) => i + 1);

  function handleTypeChange(type: RepeatType) {
    // Update local state first for transition
    localType = type;
    onRepeatTypeChange(type);
    // Reset detail when changing type
    if (type !== repeatType) {
      onRepeatDetailChange([]);
    }
  }

  function toggleWeekday(day: number) {
    if (repeatDetail.includes(day)) {
      onRepeatDetailChange(repeatDetail.filter(d => d !== day));
    } else {
      onRepeatDetailChange([...repeatDetail, day].sort((a, b) => a - b));
    }
  }

  function toggleMonthlyDay(day: number) {
    if (repeatDetail.includes(day)) {
      onRepeatDetailChange(repeatDetail.filter(d => d !== day));
    } else {
      onRepeatDetailChange([...repeatDetail, day].sort((a, b) => a - b));
    }
  }
</script>

<div class="repeat-selector">
  <span class="label">{i18n.t('repeatLabel')}</span>

  <div class="type-buttons">
    {#each repeatTypes as rt}
      <button
        type="button"
        class="type-btn"
        class:selected={repeatType === rt.value}
        onclick={() => handleTypeChange(rt.value)}
      >
        {i18n.t(rt.labelKey)}
      </button>
    {/each}
  </div>

  <div class="detail-container">
    {#if localType === 'weekly'}
      <div class="detail-section" transition:slide={{ duration: 300, easing: cubicOut }}>
        <span class="detail-label">{i18n.t('repeatDaysLabel')}</span>
        <div class="weekday-buttons">
          {#each weekdays as wd}
            <button
              type="button"
              class="weekday-btn"
              class:selected={repeatDetail.includes(wd.value)}
              onclick={() => toggleWeekday(wd.value)}
            >
              {i18n.t(wd.labelKey)}
            </button>
          {/each}
        </div>
      </div>
    {/if}

    {#if localType === 'monthly'}
      <div class="detail-section" transition:slide={{ duration: 300, easing: cubicOut }}>
        <span class="detail-label">{i18n.t('repeatDatesLabel')}</span>
        <div class="monthly-grid">
          {#each monthlyDays as day}
            <button
              type="button"
              class="monthly-btn"
              class:selected={repeatDetail.includes(day)}
              onclick={() => toggleMonthlyDay(day)}
            >
              {day}
            </button>
          {/each}
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  .repeat-selector {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .label {
    font-size: 14px;
    font-weight: 500;
    color: var(--color-ink-muted);
  }

  .type-buttons {
    display: flex;
    gap: 6px;
  }

  .type-btn {
    flex: 1;
    padding: 8px 4px;
    border: 2px solid var(--color-stroke);
    border-radius: 8px;
    background: var(--color-white);
    color: var(--color-ink);
    font-size: 13px;
    cursor: pointer;
    transition: all 0.2s;
    text-align: center;
    white-space: nowrap;
  }

  .type-btn:hover {
    border-color: var(--color-accent-sky);
  }

  .type-btn.selected {
    background: var(--color-accent-sky);
    border-color: var(--color-accent-sky);
    color: var(--color-ink);
  }

  .detail-container {
    overflow: hidden;
  }

  .detail-section {
    padding-top: 6px;
  }

  .detail-label {
    display: block;
    font-size: 13px;
    color: var(--color-ink-muted);
    margin-bottom: 8px;
  }

  .weekday-buttons {
    display: flex;
    gap: 4px;
  }

  .weekday-btn {
    flex: 1;
    height: 36px;
    border: 2px solid var(--color-stroke);
    border-radius: 8px;
    background: var(--color-white);
    color: var(--color-ink);
    font-size: 13px;
    cursor: pointer;
    transition: all 0.2s;
    text-align: center;
  }

  .weekday-btn:hover {
    border-color: var(--color-accent-sky);
  }

  .weekday-btn.selected {
    background: var(--color-accent-sky);
    border-color: var(--color-accent-sky);
    color: var(--color-ink);
  }

  .monthly-grid {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    gap: 4px;
  }

  .monthly-btn {
    width: 100%;
    aspect-ratio: 1;
    border: 1px solid var(--color-stroke);
    border-radius: 6px;
    background: var(--color-white);
    color: var(--color-ink);
    font-size: 12px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .monthly-btn:hover {
    border-color: var(--color-accent-sky);
  }

  .monthly-btn.selected {
    background: var(--color-accent-sky);
    border-color: var(--color-accent-sky);
    color: var(--color-ink);
  }
</style>
