import type { Category } from '../../types';

// Modal state
let showResetConfirm = $state(false);
let showCategoryMenu = $state(false);
let showDeleteCategoryConfirm = $state(false);
let showAddItemModal = $state(false);
let showReorderModal = $state(false);
let showReorderCategoriesModal = $state(false);
let showStreakModal = $state(false);
let selectedCategoryForMenu = $state<Category | null>(null);

// Reset confirm modal actions
function openResetConfirm(): void {
  showResetConfirm = true;
}

function closeResetConfirm(): void {
  showResetConfirm = false;
}

// Add item modal actions
function openAddItemModal(): void {
  showAddItemModal = true;
}

function closeAddItemModal(): void {
  showAddItemModal = false;
}

// Reorder modal actions
function openReorderModal(): void {
  showReorderModal = true;
}

function closeReorderModal(): void {
  showReorderModal = false;
}

// Reorder categories modal actions
function openReorderCategoriesModal(): void {
  showReorderCategoriesModal = true;
}

function closeReorderCategoriesModal(): void {
  showReorderCategoriesModal = false;
}

// Streak modal actions
function openStreakModal(): void {
  showStreakModal = true;
}

function closeStreakModal(): void {
  showStreakModal = false;
}

// Category menu modal actions
function openCategoryMenu(category: Category): void {
  selectedCategoryForMenu = category;
  showCategoryMenu = true;
}

function closeCategoryMenu(): void {
  showCategoryMenu = false;
  selectedCategoryForMenu = null;
}

// Delete category confirm modal actions
function openDeleteCategoryConfirm(): void {
  showCategoryMenu = false;
  showDeleteCategoryConfirm = true;
}

function closeDeleteCategoryConfirm(): void {
  showDeleteCategoryConfirm = false;
  selectedCategoryForMenu = null;
}

// Utility to close all modals
function closeAllModals(): void {
  showResetConfirm = false;
  showCategoryMenu = false;
  showDeleteCategoryConfirm = false;
  showAddItemModal = false;
  showReorderModal = false;
  showReorderCategoriesModal = false;
  showStreakModal = false;
  selectedCategoryForMenu = null;
}

// Export store with getters and actions
export const modalStore = {
  // Getters (reactive)
  get showResetConfirm() { return showResetConfirm; },
  get showCategoryMenu() { return showCategoryMenu; },
  get showDeleteCategoryConfirm() { return showDeleteCategoryConfirm; },
  get showAddItemModal() { return showAddItemModal; },
  get showReorderModal() { return showReorderModal; },
  get showReorderCategoriesModal() { return showReorderCategoriesModal; },
  get showStreakModal() { return showStreakModal; },
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

  // Category menu modal
  openCategoryMenu,
  closeCategoryMenu,

  // Delete category confirm modal
  openDeleteCategoryConfirm,
  closeDeleteCategoryConfirm,

  // Utility
  closeAllModals,
};
