import type { TodoItem } from '../../types';
import { invokeTauri } from './tauriClient';

export async function getItems(categoryId: number | null): Promise<TodoItem[]> {
  return invokeTauri<TodoItem[]>('get_items', { categoryId });
}

export async function addItem(text: string, categoryId: number | null): Promise<TodoItem> {
  return invokeTauri<TodoItem>('add_item', { text, categoryId });
}

export async function editItem(id: number, text: string): Promise<void> {
  await invokeTauri('edit_item', { id, text });
}

export async function deleteItem(id: number): Promise<void> {
  await invokeTauri('delete_item', { id });
}

export async function toggleItem(id: number): Promise<void> {
  await invokeTauri('toggle_item', { id });
}

export async function updateItemMemo(id: number, memo: string | null): Promise<void> {
  await invokeTauri('update_item_memo', { id, memo });
}

export async function reorderItems(itemIds: number[]): Promise<void> {
  await invokeTauri('reorder_items', { itemIds });
}

export async function resetAllItems(categoryId: number | null): Promise<void> {
  await invokeTauri('reset_all_items', { categoryId });
}

export async function checkAndAutoReset(): Promise<boolean> {
  return invokeTauri<boolean>('check_and_auto_reset');
}
