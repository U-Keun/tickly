import type { TodoItem } from '../../types';

interface HomeBootstrapDeps {
  initializeTheme: () => Promise<void>;
  initializeFonts: () => Promise<void>;
  loadLocale: () => Promise<void>;
  processRepeatsAndReload: () => Promise<void>;
  loadCategories: () => Promise<void>;
  loadItems: () => Promise<void>;
  loadAllTags: () => Promise<void>;
  loadTagsForItems: (itemList: TodoItem[]) => Promise<void>;
  getItems: () => TodoItem[];
  scheduleResetTimer: () => Promise<void>;
  rescheduleAll: (itemList: TodoItem[]) => Promise<void>;
}

export function createHomeBootstrap(deps: HomeBootstrapDeps) {
  async function run(): Promise<void> {
    await deps.initializeTheme();
    await deps.initializeFonts();
    await deps.loadLocale();

    await deps.processRepeatsAndReload();

    await deps.loadCategories();
    await deps.loadItems();
    await deps.loadAllTags();
    await deps.loadTagsForItems(deps.getItems());

    await deps.scheduleResetTimer();
    await deps.rescheduleAll(deps.getItems());
  }

  return { run };
}
