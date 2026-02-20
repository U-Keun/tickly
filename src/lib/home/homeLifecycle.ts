import type { TodoItem } from '../../types';

interface HomeLifecycleDeps {
  loadItems: () => Promise<void>;
  loadTagsForItems: (itemList: TodoItem[]) => Promise<void>;
  getItems: () => TodoItem[];
  processRepeats: () => Promise<number>;
  checkAndAutoReset: () => Promise<boolean>;
  getResetTime: () => Promise<string | null>;
  rescheduleAll: (itemList: TodoItem[]) => Promise<void>;
}

export function createHomeLifecycle(deps: HomeLifecycleDeps) {
  let lastProcessedDate = '';
  let resetTimer: ReturnType<typeof setTimeout> | null = null;

  async function processRepeatsAndReload(): Promise<void> {
    const today = new Date().toISOString().split('T')[0];
    if (lastProcessedDate === today) {
      return;
    }

    try {
      const reactivatedCount = await deps.processRepeats();
      lastProcessedDate = today;

      if (reactivatedCount > 0) {
        await deps.loadItems();
      }
    } catch (error) {
      console.error('Failed to process repeats:', error);
    }
  }

  async function checkAndPerformAutoReset(): Promise<void> {
    try {
      const didReset = await deps.checkAndAutoReset();
      if (didReset) {
        await deps.loadItems();
      }
    } catch (error) {
      console.error('Failed to check auto-reset:', error);
    }
  }

  function getMsUntilResetTime(resetTime: string): number {
    const [hours, minutes] = resetTime.split(':').map(Number);
    const now = new Date();
    const resetDate = new Date();
    resetDate.setHours(hours, minutes, 0, 0);

    if (resetDate <= now) {
      resetDate.setDate(resetDate.getDate() + 1);
    }

    return resetDate.getTime() - now.getTime();
  }

  function clearResetTimer(): void {
    if (resetTimer) {
      clearTimeout(resetTimer);
      resetTimer = null;
    }
  }

  async function scheduleResetTimer(): Promise<void> {
    clearResetTimer();

    try {
      const resetTime = await deps.getResetTime();
      if (!resetTime) return;

      const msUntilReset = getMsUntilResetTime(resetTime);
      resetTimer = setTimeout(async () => {
        await checkAndPerformAutoReset();
        await scheduleResetTimer();
      }, msUntilReset);
    } catch (error) {
      console.error('Failed to schedule reset timer:', error);
    }
  }

  async function handleVisibilityChange(): Promise<void> {
    if (document.visibilityState !== 'visible') {
      return;
    }

    await deps.loadItems();

    const itemsAfterReload = deps.getItems();
    await deps.loadTagsForItems(itemsAfterReload);
    await processRepeatsAndReload();
    await deps.rescheduleAll(deps.getItems());
  }

  return {
    processRepeatsAndReload,
    scheduleResetTimer,
    clearResetTimer,
    handleVisibilityChange,
  };
}
