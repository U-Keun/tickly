import type { TodoItem, Category, RepeatType, Tag } from '../../types';
import { createCategoryActions } from './appStoreCategoryActions';
import { createTagActions } from './appStoreTagActions';
import { syncStore } from './syncStore.svelte';
import * as streakApi from '../api/streakApi';
import * as tagApi from '../api/tagApi';
import * as todoApi from '../api/todoApi';
import * as widgetApi from '../api/widgetApi';

// Core app state
let items = $state<TodoItem[]>([]);
let categories = $state<Category[]>([]);
let selectedCategoryId = $state<number | null>(null);

// Tag state
let allTags = $state<Tag[]>([]);
let activeTagFilter = $state<number | null>(null);
let filteredItems = $state<TodoItem[]>([]);
let itemTagsMap = $state<Record<number, Tag[]>>({});

function sortItemsByDoneAndOrder(itemList: TodoItem[]): TodoItem[] {
  return [...itemList].sort((a, b) => {
    if (a.done !== b.done) {
      return a.done ? 1 : -1;
    }
    return a.display_order - b.display_order;
  });
}

function replaceItem(id: number, nextItem: TodoItem): void {
  items = items.map((item) => (item.id === id ? nextItem : item));
}

function patchItem(id: number, patch: Partial<TodoItem>): void {
  items = items.map((item) => (item.id === id ? { ...item, ...patch } : item));
}

async function finalizeMutation(): Promise<void> {
  await refreshWidgetCache();
  syncStore.scheduleSync();
}

const tagActions = createTagActions({
  getItemTagsMap: () => itemTagsMap,
  setItemTagsMap: (nextMap) => {
    itemTagsMap = nextMap;
  },
  getAllTags: () => allTags,
  setAllTags: (nextTags) => {
    allTags = nextTags;
  },
  getActiveTagFilter: () => activeTagFilter,
  setActiveTagFilter: (nextFilter) => {
    activeTagFilter = nextFilter;
  },
  setFilteredItems: (nextItems) => {
    filteredItems = nextItems;
  },
  finalizeMutation
});

// Actions
async function refreshWidgetCache(): Promise<void> {
  try {
    await widgetApi.refreshWidgetCache();
  } catch (error) {
    console.error('Failed to refresh widget cache:', error);
  }
}

async function processWidgetActions(): Promise<number> {
  try {
    return await widgetApi.processWidgetActions();
  } catch (error) {
    console.error('Failed to process widget actions:', error);
    return 0;
  }
}

async function loadItems(): Promise<void> {
  try {
    // Check and perform auto-reset if needed before loading items
    await todoApi.checkAndAutoReset();
    items = await todoApi.getItems(selectedCategoryId);
    await refreshWidgetCache();
  } catch (error) {
    console.error('Failed to load items:', error);
  }
}

const categoryActions = createCategoryActions({
  getCategories: () => categories,
  setCategories: (nextCategories) => {
    categories = nextCategories;
  },
  getSelectedCategoryId: () => selectedCategoryId,
  setSelectedCategoryId: (nextCategoryId) => {
    selectedCategoryId = nextCategoryId;
  },
  getItems: () => items,
  loadItems,
  loadTagsForItems: tagActions.loadTagsForItems,
  finalizeMutation
});

async function addItem(
  text: string,
  memo: string | null = null,
  repeatType: RepeatType = 'none',
  repeatDetail: string | null = null,
  trackStreak: boolean = false,
  tagNames: string[] = [],
  reminderAt: string | null = null,
  linkedApp: string | null = null
): Promise<void> {
  try {
    const newItem = await todoApi.addItem(text, selectedCategoryId, repeatType, repeatDetail, trackStreak, reminderAt);
    if (memo) {
      await todoApi.updateItemMemo(newItem.id, memo);
      newItem.memo = memo;
    }
    if (linkedApp) {
      await todoApi.updateItemLinkedApp(newItem.id, linkedApp);
      newItem.linked_app = linkedApp;
    }
    items = [...items, newItem];
    // Attach tags if provided
    if (tagNames.length > 0) {
      const tags: Tag[] = [];
      for (const name of tagNames) {
        const tag = await tagApi.addTagToItem(newItem.id, name);
        tags.push(tag);
      }
      itemTagsMap = { ...itemTagsMap, [newItem.id]: tags };
      await tagActions.loadAllTags();
    }
    await finalizeMutation();
  } catch (error) {
    console.error('Failed to add item:', error);
  }
}

async function toggleItem(id: number): Promise<void> {
  try {
    const updatedItem = await todoApi.toggleItem(id);
    if (updatedItem) {
      replaceItem(id, updatedItem);
      items = sortItemsByDoneAndOrder(items);
      await finalizeMutation();
    }
  } catch (error) {
    console.error('Failed to toggle item:', error);
  }
}

async function toggleItemFromWidget(id: number): Promise<void> {
  try {
    await widgetApi.toggleItemFromWidget(id);
    await refreshAll();
    syncStore.scheduleSync();
  } catch (error) {
    console.error('Failed to toggle item from widget:', error);
  }
}

async function deleteItem(id: number): Promise<void> {
  try {
    await todoApi.deleteItem(id);
    items = items.filter(item => item.id !== id);
    await finalizeMutation();
  } catch (error) {
    console.error('Failed to delete item:', error);
  }
}

async function editItem(id: number, text: string): Promise<void> {
  try {
    await todoApi.editItem(id, text);
    patchItem(id, { text });
    await finalizeMutation();
  } catch (error) {
    console.error('Failed to edit item:', error);
  }
}

async function updateMemo(id: number, memo: string | null): Promise<void> {
  try {
    await todoApi.updateItemMemo(id, memo);
    patchItem(id, { memo });
    await finalizeMutation();
  } catch (error) {
    console.error('Failed to update memo:', error);
  }
}

async function updateRepeat(
  id: number,
  repeatType: RepeatType,
  repeatDetail: string | null
): Promise<void> {
  try {
    await todoApi.updateItemRepeat(id, repeatType, repeatDetail);
    patchItem(id, { repeat_type: repeatType, repeat_detail: repeatDetail });
    await finalizeMutation();
  } catch (error) {
    console.error('Failed to update repeat:', error);
  }
}

async function updateTrackStreak(id: number, trackStreak: boolean): Promise<void> {
  try {
    await streakApi.updateTrackStreak(id, trackStreak);
    patchItem(id, { track_streak: trackStreak });
    await finalizeMutation();
  } catch (error) {
    console.error('Failed to update track_streak:', error);
  }
}

async function updateLinkedApp(id: number, linkedApp: string | null): Promise<void> {
  try {
    await todoApi.updateItemLinkedApp(id, linkedApp);
    patchItem(id, { linked_app: linkedApp });
    await finalizeMutation();
  } catch (error) {
    console.error('Failed to update linked app:', error);
  }
}

async function updateReminder(id: number, reminderAt: string | null): Promise<void> {
  try {
    await todoApi.updateItemReminder(id, reminderAt);
    patchItem(id, { reminder_at: reminderAt });
    await finalizeMutation();
  } catch (error) {
    console.error('Failed to update reminder:', error);
  }
}

async function resetAllItems(): Promise<void> {
  try {
    await todoApi.resetAllItems(selectedCategoryId);
    items = items.map(item => ({ ...item, done: false }));
    await finalizeMutation();
  } catch (error) {
    console.error('Failed to reset items:', error);
  }
}

function setItems(newItems: TodoItem[]): void {
  items = newItems;
}

async function refreshAll(): Promise<void> {
  await categoryActions.loadCategories();

  // If no category selected or selected category doesn't exist, select the first one
  if (categories.length > 0) {
    const selectedExists = categories.some(c => c.id === selectedCategoryId);
    if (!selectedExists) {
      selectedCategoryId = categories[0].id;
    }
  }

  await loadItems();
  await tagActions.loadAllTags();
}

// Export store with getters and actions
export const appStore = {
  // Getters (reactive)
  get items() { return items; },
  get categories() { return categories; },
  get selectedCategoryId() { return selectedCategoryId; },

  // Tag getters (reactive)
  get allTags() { return allTags; },
  get activeTagFilter() { return activeTagFilter; },
  get filteredItems() { return filteredItems; },
  get itemTagsMap() { return itemTagsMap; },

  // Data loading
  loadCategories: categoryActions.loadCategories,
  loadItems,
  refreshAll,
  processWidgetActions,

  // Category actions
  selectCategory: categoryActions.selectCategory,
  addCategory: categoryActions.addCategory,
  editCategory: categoryActions.editCategory,
  deleteCategory: categoryActions.deleteCategory,
  setCategories: categoryActions.setCategories,
  goToFirstCategory: categoryActions.goToFirstCategory,

  // Item actions
  addItem,
  toggleItem,
  toggleItemFromWidget,
  deleteItem,
  editItem,
  updateMemo,
  updateRepeat,
  updateTrackStreak,
  updateLinkedApp,
  updateReminder,
  resetAllItems,
  setItems,

  // Tag actions
  loadAllTags: tagActions.loadAllTags,
  loadTagsForItems: tagActions.loadTagsForItems,
  addTagToItem: tagActions.addTagToItem,
  removeTagFromItem: tagActions.removeTagFromItem,
  deleteTag: tagActions.deleteTag,
  setTagFilter: tagActions.setTagFilter,
  clearTagFilter: tagActions.clearTagFilter,
};
