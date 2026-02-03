import type { TodoItem, Category, RepeatType } from '../../types';
import * as categoryApi from '../api/categoryApi';
import * as todoApi from '../api/todoApi';
import * as streakApi from '../api/streakApi';
import { syncStore } from './syncStore.svelte';

// Core app state
let items = $state<TodoItem[]>([]);
let categories = $state<Category[]>([]);
let selectedCategoryId = $state<number | null>(null);

// Actions
async function loadCategories(): Promise<void> {
  try {
    categories = await categoryApi.getCategories();
    if (categories.length > 0 && selectedCategoryId === null) {
      selectedCategoryId = categories[0].id;
    }
  } catch (error) {
    console.error('Failed to load categories:', error);
  }
}

async function loadItems(): Promise<void> {
  try {
    // Check and perform auto-reset if needed before loading items
    const didReset = await todoApi.checkAndAutoReset();
    if (didReset) {
      console.log('Auto-reset performed');
    }
    items = await todoApi.getItems(selectedCategoryId);
  } catch (error) {
    console.error('Failed to load items:', error);
  }
}

async function selectCategory(categoryId: number): Promise<void> {
  selectedCategoryId = categoryId;
  await loadItems();
}

async function addCategory(name: string): Promise<void> {
  try {
    const newCategory = await categoryApi.addCategory(name);
    categories = [...categories, newCategory];
    await selectCategory(newCategory.id);
    syncStore.scheduleSync();
  } catch (error) {
    console.error('Failed to add category:', error);
  }
}

async function editCategory(id: number, name: string): Promise<void> {
  try {
    await categoryApi.editCategory(id, name);
    categories = categories.map(cat =>
      cat.id === id ? { ...cat, name } : cat
    );
    syncStore.scheduleSync();
  } catch (error) {
    console.error('Failed to edit category:', error);
  }
}

async function deleteCategory(id: number): Promise<boolean> {
  if (categories.length <= 1) {
    alert('최소 1개의 카테고리는 유지해야 합니다.');
    return false;
  }

  try {
    await categoryApi.deleteCategory(id);
    categories = categories.filter(cat => cat.id !== id);
    if (selectedCategoryId === id) {
      await selectCategory(categories[0].id);
    }
    syncStore.scheduleSync();
    return true;
  } catch (error) {
    console.error('Failed to delete category:', error);
    alert('카테고리 삭제 실패: ' + error);
    return false;
  }
}

async function addItem(
  text: string,
  memo: string | null = null,
  repeatType: RepeatType = 'none',
  repeatDetail: string | null = null,
  trackStreak: boolean = false
): Promise<void> {
  try {
    const newItem = await todoApi.addItem(text, selectedCategoryId, repeatType, repeatDetail, trackStreak);
    if (memo) {
      await todoApi.updateItemMemo(newItem.id, memo);
      newItem.memo = memo;
    }
    items = [...items, newItem];
    syncStore.scheduleSync();
  } catch (error) {
    console.error('Failed to add item:', error);
  }
}

async function toggleItem(id: number): Promise<void> {
  try {
    const updatedItem = await todoApi.toggleItem(id);
    if (updatedItem) {
      items = items.map(item =>
        item.id === id ? updatedItem : item
      );
      // Re-sort items (done items go to bottom)
      items = [...items].sort((a, b) => {
        if (a.done !== b.done) return a.done ? 1 : -1;
        return a.display_order - b.display_order;
      });
      syncStore.scheduleSync();
    }
  } catch (error) {
    console.error('Failed to toggle item:', error);
  }
}

async function deleteItem(id: number): Promise<void> {
  try {
    await todoApi.deleteItem(id);
    items = items.filter(item => item.id !== id);
    syncStore.scheduleSync();
  } catch (error) {
    console.error('Failed to delete item:', error);
  }
}

async function editItem(id: number, text: string): Promise<void> {
  try {
    await todoApi.editItem(id, text);
    items = items.map(item =>
      item.id === id ? { ...item, text } : item
    );
    syncStore.scheduleSync();
  } catch (error) {
    console.error('Failed to edit item:', error);
  }
}

async function updateMemo(id: number, memo: string | null): Promise<void> {
  try {
    await todoApi.updateItemMemo(id, memo);
    items = items.map(item =>
      item.id === id ? { ...item, memo } : item
    );
    syncStore.scheduleSync();
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
    items = items.map(item =>
      item.id === id ? { ...item, repeat_type: repeatType, repeat_detail: repeatDetail } : item
    );
    syncStore.scheduleSync();
  } catch (error) {
    console.error('Failed to update repeat:', error);
  }
}

async function updateTrackStreak(id: number, trackStreak: boolean): Promise<void> {
  try {
    await streakApi.updateTrackStreak(id, trackStreak);
    items = items.map(item =>
      item.id === id ? { ...item, track_streak: trackStreak } : item
    );
    syncStore.scheduleSync();
  } catch (error) {
    console.error('Failed to update track_streak:', error);
  }
}

async function resetAllItems(): Promise<void> {
  try {
    await todoApi.resetAllItems(selectedCategoryId);
    items = items.map(item => ({ ...item, done: false }));
    syncStore.scheduleSync();
  } catch (error) {
    console.error('Failed to reset items:', error);
  }
}

function setItems(newItems: TodoItem[]): void {
  items = newItems;
}

function setCategories(newCategories: Category[]): void {
  categories = newCategories;
}

function goToFirstCategory(): void {
  if (categories.length > 0) {
    selectCategory(categories[0].id);
  }
}

async function refreshAll(): Promise<void> {
  await loadCategories();

  // If no category selected or selected category doesn't exist, select the first one
  if (categories.length > 0) {
    const selectedExists = categories.some(c => c.id === selectedCategoryId);
    if (!selectedExists) {
      selectedCategoryId = categories[0].id;
    }
  }

  await loadItems();
}

// Export store with getters and actions
export const appStore = {
  // Getters (reactive)
  get items() { return items; },
  get categories() { return categories; },
  get selectedCategoryId() { return selectedCategoryId; },

  // Data loading
  loadCategories,
  loadItems,
  refreshAll,

  // Category actions
  selectCategory,
  addCategory,
  editCategory,
  deleteCategory,
  setCategories,
  goToFirstCategory,

  // Item actions
  addItem,
  toggleItem,
  deleteItem,
  editItem,
  updateMemo,
  updateRepeat,
  updateTrackStreak,
  resetAllItems,
  setItems,
};
