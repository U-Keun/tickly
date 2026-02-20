import type { TodoItem } from '../../types';

interface HomeEditModalActionDeps {
  getCapturedCloseDrawer: () => (() => void) | null;
  setCapturedCloseDrawer: (handler: (() => void) | null) => void;
  setEditingItem: (item: TodoItem | null) => void;
  setShowEditModal: (show: boolean) => void;
  setIsEditingItem: (editing: boolean) => void;
}

export function createHomeEditModalActions(deps: HomeEditModalActionDeps) {
  function openEditModal(item: TodoItem, closeDrawer: () => void): void {
    deps.setEditingItem(item);
    deps.setCapturedCloseDrawer(closeDrawer);
    deps.setShowEditModal(true);
    deps.setIsEditingItem(true);
  }

  function handleEditSave(): void {
    deps.setShowEditModal(false);
    deps.setIsEditingItem(false);
    deps.getCapturedCloseDrawer()?.();
    deps.setCapturedCloseDrawer(null);
    deps.setEditingItem(null);
  }

  function handleEditCancel(): void {
    deps.setShowEditModal(false);
    deps.setIsEditingItem(false);
    deps.setEditingItem(null);
  }

  return {
    openEditModal,
    handleEditSave,
    handleEditCancel
  };
}
