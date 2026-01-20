import { invoke } from './client';
import type { Category } from '../../types';

export async function getCategories(): Promise<Category[]> {
  return invoke<Category[]>('get_categories');
}

export async function addCategory(name: string): Promise<Category> {
  return invoke<Category>('add_category', { name });
}

export async function editCategory(id: number, name: string): Promise<void> {
  return invoke<void>('edit_category', { id, name });
}

export async function deleteCategory(id: number): Promise<void> {
  return invoke<void>('delete_category', { id });
}

export async function reorderCategories(categoryIds: number[]): Promise<void> {
  return invoke<void>('reorder_categories', { categoryIds });
}
