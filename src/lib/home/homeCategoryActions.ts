import type { Category } from '../../types';

interface CategoryTabsLike {
  triggerEdit: (category: Category) => void;
}

interface HomeCategoryActionDeps {
  openCategoryMenu: (category: Category) => void;
  closeCategoryMenu: () => void;
  closeDeleteCategoryConfirm: () => void;
  closeResetConfirm: () => void;
  getSelectedCategoryForMenu: () => Category | null;
  getCategoryTabsComponent: () => CategoryTabsLike | null;
  deleteCategory: (id: number) => Promise<boolean>;
  resetAllItems: () => Promise<void>;
}

export function createHomeCategoryActions(deps: HomeCategoryActionDeps) {
  function handleCategoryLongPress(category: Category): void {
    deps.openCategoryMenu(category);
  }

  function handleEditFromMenu(): void {
    const selectedCategory = deps.getSelectedCategoryForMenu();
    const categoryTabsComponent = deps.getCategoryTabsComponent();

    if (!selectedCategory || !categoryTabsComponent) {
      return;
    }

    categoryTabsComponent.triggerEdit(selectedCategory);
    deps.closeCategoryMenu();
  }

  async function confirmDeleteCategory(): Promise<void> {
    const categoryToDelete = deps.getSelectedCategoryForMenu();
    if (!categoryToDelete) {
      return;
    }

    const success = await deps.deleteCategory(categoryToDelete.id);
    if (success) {
      deps.closeDeleteCategoryConfirm();
    }
  }

  async function confirmReset(): Promise<void> {
    await deps.resetAllItems();
    deps.closeResetConfirm();
  }

  return {
    handleCategoryLongPress,
    handleEditFromMenu,
    confirmDeleteCategory,
    confirmReset
  };
}
