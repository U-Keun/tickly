import { invoke } from './client';
import type { Tag, TodoItem } from '../../types';

export async function getAllTags(): Promise<Tag[]> {
  return invoke<Tag[]>('get_all_tags');
}

export async function createTag(name: string): Promise<Tag> {
  return invoke<Tag>('create_tag', { name });
}

export async function deleteTag(id: number): Promise<void> {
  return invoke<void>('delete_tag', { id });
}

export async function addTagToItem(itemId: number, tagName: string): Promise<Tag> {
  return invoke<Tag>('add_tag_to_item', { itemId, tagName });
}

export async function removeTagFromItem(itemId: number, tagId: number): Promise<void> {
  return invoke<void>('remove_tag_from_item', { itemId, tagId });
}

export async function getTagsForItem(itemId: number): Promise<Tag[]> {
  return invoke<Tag[]>('get_tags_for_item', { itemId });
}

export async function getItemsByTag(tagId: number): Promise<TodoItem[]> {
  return invoke<TodoItem[]>('get_items_by_tag', { tagId });
}
