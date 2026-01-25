import { invoke } from './client';
import type { TodoItem, RepeatType } from '../../types';

export async function getItems(categoryId: number | null): Promise<TodoItem[]> {
  return invoke<TodoItem[]>('get_items', { categoryId });
}

export async function addItem(
  text: string,
  categoryId: number | null,
  repeatType?: RepeatType,
  repeatDetail?: string | null
): Promise<TodoItem> {
  return invoke<TodoItem>('add_item', {
    text,
    categoryId,
    repeatType: repeatType || null,
    repeatDetail: repeatDetail || null,
  });
}

export async function toggleItem(id: number): Promise<TodoItem | null> {
  return invoke<TodoItem | null>('toggle_item', { id });
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

export async function updateItemRepeat(
  id: number,
  repeatType: RepeatType,
  repeatDetail: string | null
): Promise<void> {
  return invoke<void>('update_item_repeat', { id, repeatType, repeatDetail });
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

export async function processRepeats(): Promise<number> {
  return invoke<number>('process_repeats');
}
