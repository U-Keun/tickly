import type { Tag, TodoItem } from '../../types';
import * as tagApi from '../api/tagApi';

type ItemTagsMap = Record<number, Tag[]>;

interface TagActionsContext {
  getItemTagsMap: () => ItemTagsMap;
  setItemTagsMap: (nextMap: ItemTagsMap) => void;
  getAllTags: () => Tag[];
  setAllTags: (nextTags: Tag[]) => void;
  getActiveTagFilter: () => number | null;
  setActiveTagFilter: (nextFilter: number | null) => void;
  setFilteredItems: (nextItems: TodoItem[]) => void;
  finalizeMutation: () => Promise<void>;
}

export function createTagActions(context: TagActionsContext) {
  async function loadAllTags(): Promise<void> {
    try {
      context.setAllTags(await tagApi.getAllTags());
    } catch (error) {
      console.error('Failed to load tags:', error);
    }
  }

  async function loadTagsForItems(itemList: TodoItem[]): Promise<void> {
    try {
      const entries = await Promise.all(
        itemList.map(async (item) => [item.id, await tagApi.getTagsForItem(item.id)] as const)
      );

      const map: ItemTagsMap = { ...context.getItemTagsMap() };
      for (const [itemId, tags] of entries) {
        map[itemId] = tags;
      }
      context.setItemTagsMap(map);
    } catch (error) {
      console.error('Failed to load item tags:', error);
    }
  }

  async function addTagToItem(itemId: number, tagName: string): Promise<Tag | null> {
    try {
      const tag = await tagApi.addTagToItem(itemId, tagName);
      const currentMap = context.getItemTagsMap();
      const currentTags = currentMap[itemId] || [];

      if (!currentTags.find((itemTag) => itemTag.id === tag.id)) {
        context.setItemTagsMap({ ...currentMap, [itemId]: [...currentTags, tag] });
      }

      await loadAllTags();
      await context.finalizeMutation();
      return tag;
    } catch (error) {
      console.error('Failed to add tag to item:', error);
      return null;
    }
  }

  async function removeTagFromItem(itemId: number, tagId: number): Promise<void> {
    try {
      await tagApi.removeTagFromItem(itemId, tagId);
      const currentMap = context.getItemTagsMap();
      const currentTags = currentMap[itemId] || [];
      context.setItemTagsMap({
        ...currentMap,
        [itemId]: currentTags.filter((itemTag) => itemTag.id !== tagId)
      });
      await context.finalizeMutation();
    } catch (error) {
      console.error('Failed to remove tag from item:', error);
    }
  }

  async function deleteTag(tagId: number): Promise<void> {
    try {
      await tagApi.deleteTag(tagId);
      context.setAllTags(context.getAllTags().filter((tag) => tag.id !== tagId));

      const newMap: ItemTagsMap = {};
      for (const [id, tags] of Object.entries(context.getItemTagsMap())) {
        newMap[Number(id)] = tags.filter((tag) => tag.id !== tagId);
      }
      context.setItemTagsMap(newMap);

      if (context.getActiveTagFilter() === tagId) {
        context.setActiveTagFilter(null);
        context.setFilteredItems([]);
      }

      await context.finalizeMutation();
    } catch (error) {
      console.error('Failed to delete tag:', error);
    }
  }

  async function setTagFilter(tagId: number): Promise<void> {
    try {
      context.setActiveTagFilter(tagId);
      context.setFilteredItems(await tagApi.getItemsByTag(tagId));
    } catch (error) {
      console.error('Failed to filter by tag:', error);
    }
  }

  function clearTagFilter(): void {
    context.setActiveTagFilter(null);
    context.setFilteredItems([]);
  }

  return {
    loadAllTags,
    loadTagsForItems,
    addTagToItem,
    removeTagFromItem,
    deleteTag,
    setTagFilter,
    clearTagFilter
  };
}
