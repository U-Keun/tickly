<script lang="ts">
  import { onMount } from 'svelte';
  import type { CompletionLog, HeatmapIntensity } from '../types';

  let {
    logs,
    comboIntensity,
    onDaySelect,
  }: {
    logs: CompletionLog[];
    comboIntensity: HeatmapIntensity[];
    onDaySelect?: (day: { date: string; count: number; level: number }) => void;
  } = $props();

  let scrollContainer: HTMLDivElement;
  const MAX_LEVEL = 10;

  type DayCell = { date: string; count: number; level: number; weekday: number };

  // Create a map of date -> count for quick lookup
  let logMap = $derived.by(() => {
    const map = new Map<string, number>();
    logs.forEach((log) => {
      map.set(log.completed_on, log.completed_count);
    });
    return map;
  });

  let comboIntensityMap = $derived.by(() => {
    const map = new Map<string, number>();
    comboIntensity.forEach((entry) => {
      map.set(entry.completed_on, entry.level);
    });
    return map;
  });

  // Generate 365 days of data
  let days = $derived.by(() => {
    const result: DayCell[] = [];
    const today = new Date();

    for (let i = 364; i >= 0; i--) {
      const d = new Date(today);
      d.setDate(d.getDate() - i);
      const dateStr = d.toISOString().split('T')[0];
      const count = logMap.get(dateStr) || 0;
      const level = comboIntensityMap.get(dateStr) || (count > 0 ? 1 : 0);
      result.push({
        date: dateStr,
        count,
        level,
        weekday: d.getDay(),
      });
    }

    return result;
  });

  // Group days into weeks (columns)
  let weeks = $derived.by(() => {
    const result: DayCell[][] = [];
    let currentWeek: DayCell[] = [];

    // Fill in empty days at the start to align with the first week
    const firstDay = days[0];
    if (firstDay) {
      for (let i = 0; i < firstDay.weekday; i++) {
        currentWeek.push({ date: '', count: -1, level: 0, weekday: i });
      }
    }

    days.forEach((day) => {
      currentWeek.push(day);
      if (day.weekday === 6) {
        result.push(currentWeek);
        currentWeek = [];
      }
    });

    // Push remaining days
    if (currentWeek.length > 0) {
      result.push(currentWeek);
    }

    return result;
  });

  function getColorClass(count: number): string {
    if (count < 0) return 'invisible';
    if (count === 0) return 'day-empty';
    return 'day-done';
  }

  function getDoneMixPercent(level: number): number {
    const normalizedLevel = Math.min(Math.max(level, 1), MAX_LEVEL);
    const mixTable = [14, 22, 30, 38, 48, 58, 68, 78, 89, 100];
    return mixTable[normalizedLevel - 1];
  }

  function getDoneStyle(level: number): string {
    const mixPercent = getDoneMixPercent(level);
    return `background-color: color-mix(in srgb, var(--color-accent-mint-strong) ${mixPercent}%, var(--color-mist));`;
  }

  function handleDaySelect(day: DayCell) {
    if (day.count < 0 || !day.date) {
      return;
    }
    onDaySelect?.({ date: day.date, count: day.count, level: day.level });
  }

  function getDayAriaLabel(day: DayCell): string {
    if (!day.date || day.count < 0) {
      return 'empty';
    }

    if (day.count > 0) {
      return `${day.date}, completed, combo level ${day.level}`;
    }

    return `${day.date}, not completed`;
  }

  // Scroll to the right (current date) on mount
  onMount(() => {
    requestAnimationFrame(() => {
      if (scrollContainer) {
        scrollContainer.scrollLeft = scrollContainer.scrollWidth;
      }
    });
  });
</script>

<div class="heatmap-container">
  <!-- Heatmap Grid -->
  <div class="heatmap-scroll" bind:this={scrollContainer}>
    <div class="heatmap-grid">
      {#each weeks as week}
        <div class="week-column">
          {#each week as day}
            <button
              type="button"
              class="day-cell {getColorClass(day.count)}"
              style={day.count > 0 ? getDoneStyle(day.level) : undefined}
              disabled={day.count < 0}
              aria-label={getDayAriaLabel(day)}
              onclick={() => handleDaySelect(day)}
            ></button>
          {/each}
        </div>
      {/each}
    </div>
  </div>
</div>

<style>
  .heatmap-container {
    width: 100%;
  }

  .heatmap-scroll {
    overflow-x: auto;
    -webkit-overflow-scrolling: touch;
    padding-bottom: 4px;
  }

  .heatmap-grid {
    display: flex;
    gap: 3px;
    min-width: fit-content;
  }

  .week-column {
    display: flex;
    flex-direction: column;
    gap: 3px;
  }

  .day-cell {
    width: 12px;
    height: 12px;
    border: none;
    padding: 0;
    outline: none;
    border-radius: 2px;
    cursor: pointer;
  }

  .day-cell:focus-visible {
    outline: 1px solid var(--color-accent-sky-strong);
    outline-offset: 1px;
  }

  .day-empty {
    background: var(--color-mist);
    opacity: 1;
  }

  .day-done {
    background: var(--color-accent-mint-strong);
    transition: background-color 0.2s ease;
  }

  .day-cell.invisible {
    visibility: hidden;
    cursor: default;
  }
</style>
