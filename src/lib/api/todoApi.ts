import { invoke } from './client';
import type { TodoItem } from '../../types';

export async function getItems(categoryId: number | null): Promise<TodoItem[]> {
  return invoke<TodoItem[]>('get_items', { categoryId });
}

export async function addItem(text: string, categoryId: number | null): Promise<TodoItem> {
  return invoke<TodoItem>('add_item', { text, categoryId });
}

export async function toggleItem(id: number): Promise<void> {
  return invoke<void>('toggle_item', { id });
}

export async function deleteItem(id: number): Promise<void> {
  return invoke<void>('delete_item', { id });
}

export async function editItem(id: number, text: string): Promise<void> {
  return invoke<void>('edit_item', { id, text });
}

export async function updateItemMemo(id: number, memo: string | null): Promise<void> {
  return invoke<void>('update_item_memo', { id, memo });
}

export async function reorderItems(itemIds: number[]): Promise<void> {
  return invoke<void>('reorder_items', { itemIds });
}

export async function resetAllItems(categoryId: number | null): Promise<void> {
  return invoke<void>('reset_all_items', { categoryId });
}

export async function checkAndAutoReset(): Promise<boolean> {
  return invoke<boolean>('check_and_auto_reset');
}
