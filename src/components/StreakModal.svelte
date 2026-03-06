<script lang="ts">
  import type { TrackedItem, HeatmapData } from '../types';
  import ModalWrapper from './ModalWrapper.svelte';
  import StreakHeatmap from './StreakHeatmap.svelte';
  import { i18n } from '$lib/i18n';
  import * as streakApi from '$lib/api/streakApi';

  type SelectedHeatmapDay = { date: string; count: number; level: number };

  let {
    show,
    onClose,
  }: {
    show: boolean;
    onClose: () => void;
  } = $props();

  let trackedItems = $state<TrackedItem[]>([]);
  let selectedItemId = $state<number | null>(null);
  let heatmapData = $state<HeatmapData | null>(null);
  let loading = $state(true);
  let loadingHeatmap = $state(false);
  let selectedHeatmapDay = $state<SelectedHeatmapDay | null>(null);

  function getTodayDateString(): string {
    return new Date().toISOString().split('T')[0];
  }

  function buildSelectedHeatmapDay(data: HeatmapData, date: string): SelectedHeatmapDay {
    const log = data.logs.find((entry) => entry.completed_on === date);
    const intensity = data.combo_intensity.find((entry) => entry.completed_on === date);
    const count = log?.completed_count ?? 0;

    return {
      date,
      count,
      level: intensity?.level ?? (count > 0 ? 1 : 0)
    };
  }

  function formatCompletedDate(dateStr: string): string {
    const date = new Date(`${dateStr}T00:00:00`);
    if (Number.isNaN(date.getTime())) {
      return dateStr;
    }

    return date.toLocaleDateString(i18n.locale, {
      year: 'numeric',
      month: 'short',
      day: 'numeric'
    });
  }

  async function loadTrackedItems() {
    loading = true;
    try {
      trackedItems = await streakApi.getTrackedItems();
      // Auto-select first item if available
      if (trackedItems.length > 0 && !selectedItemId) {
        await selectItem(trackedItems[0].id);
      }
    } catch (error) {
      console.error('Failed to load tracked items:', error);
    }
    loading = false;
  }

  async function selectItem(itemId: number) {
    selectedItemId = itemId;
    loadingHeatmap = true;
    try {
      const data = await streakApi.getItemHeatmapData(itemId);
      heatmapData = data;
      selectedHeatmapDay = data
        ? buildSelectedHeatmapDay(data, getTodayDateString())
        : null;
    } catch (error) {
      console.error('Failed to load heatmap data:', error);
      heatmapData = null;
      selectedHeatmapDay = null;
    }
    loadingHeatmap = false;
  }

  // Load data when modal opens
  $effect(() => {
    if (show) {
      selectedItemId = null;
      selectedHeatmapDay = null;
      heatmapData = null;
      loadTrackedItems();
    }
  });

</script>

<ModalWrapper {show} {onClose} size="md" position="center">
  <div class="streak-modal">
    <!-- Header -->
    <div class="flex items-center justify-between mb-4">
      <h2 class="text-lg font-semibold text-ink">{i18n.t('streakHeatmapTitle')}</h2>
      <button
        type="button"
        onclick={onClose}
        class="p-1 text-ink-muted hover:text-ink transition-colors"
        aria-label="Close"
      >
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </div>

    {#if loading}
      <div class="py-8 text-center text-ink-muted">
        <p>{i18n.t('loading')}</p>
      </div>
    {:else if trackedItems.length === 0}
      <!-- Empty State -->
      <div class="empty-state">
        <div class="empty-icon">
          <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
            <path d="M17.657 18.657A8 8 0 016.343 7.343S7 9 9 10c0-2 .5-5 2.986-7C14 5 16.09 5.777 17.656 7.343A7.975 7.975 0 0120 13a7.975 7.975 0 01-2.343 5.657z" />
            <path d="M9.879 16.121A3 3 0 1012.015 11L11 14H9c0 .768.293 1.536.879 2.121z" />
          </svg>
        </div>
        <p class="empty-title">{i18n.t('noTrackedItems')}</p>
        <p class="empty-subtitle">{i18n.t('addStreakHint')}</p>
      </div>
    {:else}
      <!-- Item Tabs -->
      <div class="item-tabs">
        {#each trackedItems as item}
          <button
            type="button"
            class="item-tab"
            class:active={selectedItemId === item.id}
            onclick={() => selectItem(item.id)}
          >
            {item.text}
          </button>
        {/each}
      </div>

      {#if loadingHeatmap}
        <div class="py-6 text-center text-ink-muted">
          <p>{i18n.t('loading')}</p>
        </div>
      {:else if heatmapData}
        <!-- Stats -->
        <div class="grid grid-cols-3 gap-3 mb-4">
          <div class="stat-card">
            <div class="stat-value">{heatmapData.total_days}</div>
            <div class="stat-label">{i18n.t('totalDays')}</div>
          </div>
          <div class="stat-card">
            <div class="stat-value">{heatmapData.current_streak}</div>
            <div class="stat-label">{i18n.t('currentStreak')}</div>
          </div>
          <div class="stat-card">
            <div class="stat-value">{heatmapData.longest_streak}</div>
            <div class="stat-label">{i18n.t('longestStreak')}</div>
          </div>
        </div>

        <!-- Heatmap -->
        <StreakHeatmap
          logs={heatmapData.logs}
          comboIntensity={heatmapData.combo_intensity}
          onDaySelect={(day) => selectedHeatmapDay = day}
        />

        <div class="date-panel mt-3">
          {#if selectedHeatmapDay}
            <p class="date-empty">
              {formatCompletedDate(selectedHeatmapDay.date)} · {selectedHeatmapDay.count > 0 ? i18n.t('completed') : i18n.t('notCompleted')}
            </p>
          {:else}
            <p class="date-empty">{i18n.t('loading')}</p>
          {/if}
        </div>
      {/if}
    {/if}
  </div>
</ModalWrapper>

<style>
  .streak-modal {
    min-height: 200px;
    overflow-x: hidden;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 32px 16px;
    text-align: center;
  }

  .empty-icon {
    color: var(--color-ink-muted);
    margin-bottom: 16px;
    opacity: 0.5;
  }

  .empty-title {
    font-size: 16px;
    font-weight: 500;
    color: var(--color-ink);
    margin: 0 0 8px 0;
  }

  .empty-subtitle {
    font-size: 14px;
    color: var(--color-ink-muted);
    margin: 0;
  }

  .item-tabs {
    display: flex;
    gap: 8px;
    overflow-x: auto;
    padding-bottom: 12px;
    margin-bottom: 12px;
    -webkit-overflow-scrolling: touch;
  }

  .item-tab {
    flex-shrink: 0;
    padding: 8px 16px;
    border: none;
    border-radius: 20px;
    font-size: 14px;
    background: var(--color-canvas);
    color: var(--color-ink-muted);
    cursor: pointer;
    transition: all 0.2s;
    white-space: nowrap;
  }

  .item-tab:hover {
    background: var(--color-mist);
  }

  .item-tab.active {
    background: var(--color-accent-mint-strong);
    color: var(--color-ink);
  }

  .stat-card {
    background: var(--color-canvas);
    border-radius: 8px;
    padding: 12px 8px;
    text-align: center;
  }

  .stat-value {
    font-size: 1.5rem;
    font-weight: 600;
    color: var(--color-ink);
    line-height: 1.2;
  }

  .stat-label {
    font-size: 0.75rem;
    color: var(--color-ink-muted);
    margin-top: 4px;
  }

  .date-panel {
    background: var(--color-canvas);
    border-radius: 10px;
    padding: 10px;
    margin-bottom: 12px;
  }

  .date-empty {
    margin: 0;
    font-size: 13px;
    color: var(--color-ink-muted);
  }

</style>
