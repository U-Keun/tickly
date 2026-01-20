import type { Translations } from './ko';

export const en: Translations = {
  // Main page - empty state
  emptyListTitle: 'No items yet.',
  emptyListSubtitle: 'Add an item to get started!',

  // Bottom navigation
  reorder: 'Reorder',
  home: 'Home',
  settings: 'Settings',

  // FAB buttons
  addItem: 'Add Item',
  resetCheck: 'Reset Checks',

  // AddItemModal
  addItemTitle: 'Add Item',
  todoLabel: 'To-do',
  todoPlaceholder: 'Enter your task',
  memoLabel: 'Memo (optional)',
  memoPlaceholder: 'Enter a memo',
  cancel: 'Cancel',
  add: 'Add',
  confirm: 'OK',

  // Reset confirm
  resetConfirmTitle: 'Reset Checks',
  resetConfirmMessage: 'Reset all checked items?',
  reset: 'Reset',

  // Category
  categoryDelete: 'Delete Category',
  categoryDeleteConfirm: 'Delete this category?\nAll items will be removed.',
  categoryDeleteConfirmTemplate: (name: string) =>
    `Delete "${name}" category?\nAll items will be removed.`,
  delete: 'Delete',
  edit: 'Edit',
  editName: 'Edit Name',
  categoryPlaceholder: 'Category name',
  addCategory: 'Add Category',
  categoryEditFailed: 'Failed to edit category: ',
  reorderCategories: 'Reorder Categories',

  // Settings
  settingsTitle: 'Settings',
  themeChange: 'Change Theme',
  languageChange: 'Change Language',
  back: 'Back',

  // Language settings
  languageTitle: 'Language',
  korean: '한국어',
  english: 'English',

  // Reorder items modal
  reorderItemsTitle: 'Reorder Items',
  reorderItemsSubtitle: 'Drag to reorder items in this category.',
  close: 'Close',
  noItemsToReorder: 'No items to reorder.',
  done: 'Done',

  // Reorder categories modal
  reorderCategoriesTitle: 'Reorder Categories',
  reorderCategoriesSubtitle: 'Drag to reorder categories.',
  noCategoriesToReorder: 'No categories to reorder.',

  // MemoDrawer
  itemLabel: 'Item',
  todoPlaceholderAlt: 'Enter your task...',
  memoPlaceholderAlt: 'Enter a memo...',
  save: 'Save',

  // Theme settings
  themeTitle: 'Theme Settings',
  presetTheme: 'Preset Themes',
  preview: 'Preview',
  customColors: 'Custom Colors',
  custom: 'Custom',

  // Theme presets
  themeDefault: 'Default',
  themeDark: 'Dark',
  themeOcean: 'Ocean',
  themeForest: 'Forest',
  themeSunset: 'Sunset',

  // Color labels
  colorPaper: 'Paper',
  colorCanvas: 'Canvas',
  colorMist: 'Mist',
  colorStroke: 'Stroke',
  colorInk: 'Ink',
  colorInkMuted: 'Ink Muted',
  colorAccentSky: 'Sky',
  colorAccentSkyStrong: 'Sky Strong',
  colorAccentMint: 'Mint',
  colorAccentMintStrong: 'Mint Strong',
  colorAccentPeach: 'Peach',
  colorAccentPeachStrong: 'Peach Strong',
  colorWhite: 'White',
  colorBorder: 'Border',
};
