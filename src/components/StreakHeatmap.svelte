<script lang="ts">
  import { onMount } from 'svelte';
  import type { CompletionLog } from '../types';

  let {
    logs,
  }: {
    logs: CompletionLog[];
  } = $props();

  let scrollContainer: HTMLDivElement;

  type DayCell = { date: string; count: number; weekday: number };

  // Create a map of date -> count for quick lookup
  let logMap = $derived.by(() => {
    const map = new Map<string, number>();
    logs.forEach((log) => {
      map.set(log.completed_on, log.completed_count);
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
      result.push({
        date: dateStr,
        count,
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
        currentWeek.push({ date: '', count: -1, weekday: i });
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

  // Get color class based on count (2 levels: done or not)
  function getColorClass(count: number): string {
    if (count < 0) return 'invisible';
    if (count === 0) return 'bg-mist';
    return 'bg-accent-mint-strong';
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
            <div class="day-cell {getColorClass(day.count)}"></div>
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
    border-radius: 2px;
  }

  .day-cell.invisible {
    visibility: hidden;
  }
</style>
