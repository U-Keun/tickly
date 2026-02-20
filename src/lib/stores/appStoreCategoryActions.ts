import type { Category, TodoItem } from '../../types';
import * as categoryApi from '../api/categoryApi';

interface CategoryActionsContext {
  getCategories: () => Category[];
  setCategories: (nextCategories: Category[]) => void;
  getSelectedCategoryId: () => number | null;
  setSelectedCategoryId: (nextCategoryId: number | null) => void;
  getItems: () => TodoItem[];
  loadItems: () => Promise<void>;
  loadTagsForItems: (itemList: TodoItem[]) => Promise<void>;
  finalizeMutation: () => Promise<void>;
}

export function createCategoryActions(context: CategoryActionsContext) {
  async function loadCategories(): Promise<void> {
    try {
      const nextCategories = await categoryApi.getCategories();
      context.setCategories(nextCategories);

      if (nextCategories.length > 0 && context.getSelectedCategoryId() === null) {
        context.setSelectedCategoryId(nextCategories[0].id);
      }
    } catch (error) {
      console.error('Failed to load categories:', error);
    }
  }

  async function selectCategory(categoryId: number): Promise<void> {
    context.setSelectedCategoryId(categoryId);
    await context.loadItems();
    await context.loadTagsForItems(context.getItems());
  }

  async function addCategory(name: string): Promise<void> {
    try {
      const newCategory = await categoryApi.addCategory(name);
      context.setCategories([...context.getCategories(), newCategory]);
      await selectCategory(newCategory.id);
      await context.finalizeMutation();
    } catch (error) {
      console.error('Failed to add category:', error);
    }
  }

  async function editCategory(id: number, name: string): Promise<void> {
    try {
      await categoryApi.editCategory(id, name);
      context.setCategories(
        context.getCategories().map((category) =>
          category.id === id ? { ...category, name } : category
        )
      );
      await context.finalizeMutation();
    } catch (error) {
      console.error('Failed to edit category:', error);
    }
  }

  async function deleteCategory(id: number): Promise<boolean> {
    const currentCategories = context.getCategories();
    if (currentCategories.length <= 1) {
      alert('최소 1개의 카테고리는 유지해야 합니다.');
      return false;
    }

    try {
      await categoryApi.deleteCategory(id);

      const nextCategories = currentCategories.filter((category) => category.id !== id);
      context.setCategories(nextCategories);

      if (context.getSelectedCategoryId() === id) {
        await selectCategory(nextCategories[0].id);
      }

      await context.finalizeMutation();
      return true;
    } catch (error) {
      console.error('Failed to delete category:', error);
      alert('카테고리 삭제 실패: ' + error);
      return false;
    }
  }

  function setCategories(nextCategories: Category[]): void {
    context.setCategories(nextCategories);
  }

  function goToFirstCategory(): void {
    const currentCategories = context.getCategories();
    if (currentCategories.length > 0) {
      void selectCategory(currentCategories[0].id);
    }
  }

  return {
    loadCategories,
    selectCategory,
    addCategory,
    editCategory,
    deleteCategory,
    setCategories,
    goToFirstCategory
  };
}
