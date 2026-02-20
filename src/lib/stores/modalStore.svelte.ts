import type { Category } from '../../types';

// Modal state
type ModalFlag =
  | 'showResetConfirm'
  | 'showCategoryMenu'
  | 'showDeleteCategoryConfirm'
  | 'showAddItemModal'
  | 'showReorderModal'
  | 'showReorderCategoriesModal'
  | 'showStreakModal'
  | 'showTagFilterModal';

const modalKeys: ModalFlag[] = [
  'showResetConfirm',
  'showCategoryMenu',
  'showDeleteCategoryConfirm',
  'showAddItemModal',
  'showReorderModal',
  'showReorderCategoriesModal',
  'showStreakModal',
  'showTagFilterModal',
];

let modalState = $state<Record<ModalFlag, boolean>>({
  showResetConfirm: false,
  showCategoryMenu: false,
  showDeleteCategoryConfirm: false,
  showAddItemModal: false,
  showReorderModal: false,
  showReorderCategoriesModal: false,
  showStreakModal: false,
  showTagFilterModal: false,
});
let selectedCategoryForMenu = $state<Category | null>(null);

function setModalOpen(key: ModalFlag, isOpen: boolean): void {
  modalState[key] = isOpen;
}

// Reset confirm modal actions
function openResetConfirm(): void {
  setModalOpen('showResetConfirm', true);
}

function closeResetConfirm(): void {
  setModalOpen('showResetConfirm', false);
}

// Add item modal actions
function openAddItemModal(): void {
  setModalOpen('showAddItemModal', true);
}

function closeAddItemModal(): void {
  setModalOpen('showAddItemModal', false);
}

// Reorder modal actions
function openReorderModal(): void {
  setModalOpen('showReorderModal', true);
}

function closeReorderModal(): void {
  setModalOpen('showReorderModal', false);
}

// Reorder categories modal actions
function openReorderCategoriesModal(): void {
  setModalOpen('showReorderCategoriesModal', true);
}

function closeReorderCategoriesModal(): void {
  setModalOpen('showReorderCategoriesModal', false);
}

// Streak modal actions
function openStreakModal(): void {
  setModalOpen('showStreakModal', true);
}

function closeStreakModal(): void {
  setModalOpen('showStreakModal', false);
}

// Tag filter modal actions
function openTagFilterModal(): void {
  setModalOpen('showTagFilterModal', true);
}

function closeTagFilterModal(): void {
  setModalOpen('showTagFilterModal', false);
}

// Category menu modal actions
function openCategoryMenu(category: Category): void {
  selectedCategoryForMenu = category;
  setModalOpen('showCategoryMenu', true);
}

function closeCategoryMenu(): void {
  setModalOpen('showCategoryMenu', false);
  selectedCategoryForMenu = null;
}

// Delete category confirm modal actions
function openDeleteCategoryConfirm(): void {
  setModalOpen('showCategoryMenu', false);
  setModalOpen('showDeleteCategoryConfirm', true);
}

function closeDeleteCategoryConfirm(): void {
  setModalOpen('showDeleteCategoryConfirm', false);
  selectedCategoryForMenu = null;
}

// Utility to close all modals
function closeAllModals(): void {
  for (const key of modalKeys) {
    setModalOpen(key, false);
  }
  selectedCategoryForMenu = null;
}

// Export store with getters and actions
export const modalStore = {
  // Getters (reactive)
  get showResetConfirm() { return modalState.showResetConfirm; },
  get showCategoryMenu() { return modalState.showCategoryMenu; },
  get showDeleteCategoryConfirm() { return modalState.showDeleteCategoryConfirm; },
  get showAddItemModal() { return modalState.showAddItemModal; },
  get showReorderModal() { return modalState.showReorderModal; },
  get showReorderCategoriesModal() { return modalState.showReorderCategoriesModal; },
  get showStreakModal() { return modalState.showStreakModal; },
  get showTagFilterModal() { return modalState.showTagFilterModal; },
  get selectedCategoryForMenu() { return selectedCategoryForMenu; },

  // Reset confirm modal
  openResetConfirm,
  closeResetConfirm,

  // Add item modal
  openAddItemModal,
  closeAddItemModal,

  // Reorder modal
  openReorderModal,
  closeReorderModal,

  // Reorder categories modal
  openReorderCategoriesModal,
  closeReorderCategoriesModal,

  // Streak modal
  openStreakModal,
  closeStreakModal,

  // Tag filter modal
  openTagFilterModal,
  closeTagFilterModal,

  // Category menu modal
  openCategoryMenu,
  closeCategoryMenu,

  // Delete category confirm modal
  openDeleteCategoryConfirm,
  closeDeleteCategoryConfirm,

  // Utility
  closeAllModals,
};
