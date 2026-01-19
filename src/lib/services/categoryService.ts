import type { Category } from '../../types';
import { invokeTauri } from './tauriClient';

export async function getCategories(): Promise<Category[]> {
  return invokeTauri<Category[]>('get_categories');
}

export async function addCategory(name: string): Promise<Category> {
  return invokeTauri<Category>('add_category', { name });
}

export async function editCategory(id: number, name: string): Promise<void> {
  await invokeTauri('edit_category', { id, name });
}

export async function deleteCategory(id: number): Promise<void> {
  await invokeTauri('delete_category', { id });
}

export async function reorderCategories(categoryIds: number[]): Promise<void> {
  await invokeTauri('reorder_categories', { categoryIds });
}
