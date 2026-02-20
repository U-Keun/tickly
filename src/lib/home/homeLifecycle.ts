import type { TodoItem } from '../../types';
import { createHomeResetScheduler } from './homeResetScheduler';

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

  const { scheduleResetTimer, clearResetTimer } = createHomeResetScheduler({
    getResetTime: deps.getResetTime,
    onReset: checkAndPerformAutoReset
  });

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
