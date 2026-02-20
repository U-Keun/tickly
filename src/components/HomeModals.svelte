<script lang="ts">
  import type { RepeatType, TodoItem } from '../types';

  import AddItemModal from './AddItemModal.svelte';
  import CategoryMenuModal from './CategoryMenuModal.svelte';
  import ConfirmModal from './ConfirmModal.svelte';
  import EditItemModal from './EditItemModal.svelte';
  import ReorderCategoriesModal from './ReorderCategoriesModal.svelte';
  import ReorderItemsModal from './ReorderItemsModal.svelte';
  import StreakModal from './StreakModal.svelte';
  import TagFilterModal from './TagFilterModal.svelte';
  import { i18n } from '$lib/i18n';
  import { appStore, modalStore } from '$lib/stores';

  interface Props {
    handleAddItem: (
      text: string,
      memo: string | null,
      repeatType: RepeatType,
      repeatDetail: string | null,
      trackStreak: boolean,
      tagNames?: string[],
      reminderAt?: string | null,
      linkedApp?: string | null
    ) => Promise<void>;
    handleEditFromMenu: () => void;
    confirmReset: () => Promise<void>;
    confirmDeleteCategory: () => Promise<void>;
    editingItem: TodoItem | null;
    showEditModal: boolean;
    handleUpdateReminder: (id: number, reminderAt: string | null) => Promise<void>;
    handleEditSave: () => void;
    handleEditCancel: () => void;
  }

  let {
    handleAddItem,
    handleEditFromMenu,
    confirmReset,
    confirmDeleteCategory,
    editingItem,
    showEditModal,
    handleUpdateReminder,
    handleEditSave,
    handleEditCancel
  }: Props = $props();
</script>

<AddItemModal
  show={modalStore.showAddItemModal}
  allTags={appStore.allTags}
  onAdd={handleAddItem}
  onCancel={modalStore.closeAddItemModal}
/>

<ReorderItemsModal
  show={modalStore.showReorderModal}
  items={appStore.items}
  onItemsReorder={appStore.setItems}
  onClose={modalStore.closeReorderModal}
/>

<ReorderCategoriesModal
  show={modalStore.showReorderCategoriesModal}
  categories={appStore.categories}
  onCategoriesReorder={appStore.setCategories}
  onClose={modalStore.closeReorderCategoriesModal}
/>

<ConfirmModal
  show={modalStore.showResetConfirm}
  title={i18n.t('resetConfirmTitle')}
  message={i18n.t('resetConfirmMessage')}
  confirmLabel={i18n.t('reset')}
  cancelLabel={i18n.t('cancel')}
  confirmStyle="warning"
  onConfirm={confirmReset}
  onCancel={modalStore.closeResetConfirm}
/>

<CategoryMenuModal
  show={modalStore.showCategoryMenu}
  category={modalStore.selectedCategoryForMenu}
  onEdit={handleEditFromMenu}
  onDelete={modalStore.openDeleteCategoryConfirm}
  onClose={modalStore.closeCategoryMenu}
/>

<ConfirmModal
  show={modalStore.showDeleteCategoryConfirm}
  title={i18n.t('categoryDelete')}
  message={modalStore.selectedCategoryForMenu
    ? i18n.t('categoryDeleteConfirmTemplate')(modalStore.selectedCategoryForMenu.name)
    : ''}
  confirmLabel={i18n.t('delete')}
  cancelLabel={i18n.t('cancel')}
  confirmStyle="danger"
  onConfirm={confirmDeleteCategory}
  onCancel={modalStore.closeDeleteCategoryConfirm}
/>

<StreakModal
  show={modalStore.showStreakModal}
  onClose={modalStore.closeStreakModal}
/>

<TagFilterModal
  show={modalStore.showTagFilterModal}
  tags={appStore.allTags}
  activeTagId={appStore.activeTagFilter}
  onSelect={appStore.setTagFilter}
  onClear={appStore.clearTagFilter}
  onClose={modalStore.closeTagFilterModal}
/>

{#if editingItem}
  <EditItemModal
    show={showEditModal}
    item={editingItem}
    itemTags={appStore.itemTagsMap[editingItem.id] ?? []}
    allTags={appStore.allTags}
    onSaveMemo={appStore.updateMemo}
    onEditText={appStore.editItem}
    onUpdateRepeat={appStore.updateRepeat}
    onUpdateTrackStreak={appStore.updateTrackStreak}
    onUpdateReminder={handleUpdateReminder}
    onUpdateLinkedApp={appStore.updateLinkedApp}
    onAddTag={appStore.addTagToItem}
    onRemoveTag={appStore.removeTagFromItem}
    onSave={handleEditSave}
    onCancel={handleEditCancel}
  />
{/if}
