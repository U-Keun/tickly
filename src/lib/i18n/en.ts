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
  menu: 'Menu',

  // AddItemModal
  addItemTitle: 'Add Item',
  todoLabel: 'Item',
  todoPlaceholder: 'Enter your item',
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

  // Font settings
  fontChange: 'Change Font',
  fontTitle: 'Font Settings',
  fontPreset: 'Font Selection',
  fontSize: 'Size',
  fontSizeSmall: 'Small',
  fontSizeMedium: 'Medium',
  fontSizeLarge: 'Large',
  fontSystem: 'System Default',
  fontNotoSans: 'Noto Sans KR',
  fontPretendard: 'Pretendard',
  fontMonospace: 'Monospace',
  fontPreviewText: 'The quick brown fox jumps over the lazy dog.',

  // Repeat settings
  repeatLabel: 'Repeat',
  repeatNone: 'None',
  repeatDaily: 'Daily',
  repeatWeekly: 'Weekly',
  repeatMonthly: 'Monthly',
  repeatDaysLabel: 'Repeat Days',
  repeatDatesLabel: 'Repeat Dates',
  sun: 'Sun',
  mon: 'Mon',
  tue: 'Tue',
  wed: 'Wed',
  thu: 'Thu',
  fri: 'Fri',
  sat: 'Sat',

  // Streak heatmap
  streak: 'Streak',
  streakHeatmapTitle: 'Streak',
  totalDays: 'Total Days',
  currentStreak: 'Current Streak',
  longestStreak: 'Longest Streak',
  completions: 'completions',
  less: 'Less',
  more: 'More',
  loading: 'Loading...',
  trackStreak: 'Track Streak',
  trackingStreak: 'Tracking streak',
  noTrackedItems: 'No tracked items yet',
  addStreakHint: 'Enable streak tracking in item details!',

  // Reset time settings
  resetTimeChange: 'Reset Time',
  resetTimeTitle: 'Reset Time',
  resetTimeDescription: 'Time when daily tasks reset',

  // Account & Sync settings
  account: 'Account',
  accountTitle: 'Account Settings',
  syncTitle: 'Sync',
  syncChange: 'Sync',
  loginRequired: 'Login required',
  loginDescription: 'Sign in to use cloud sync',
  signInWithApple: 'Sign in with Apple',
  signInWithGoogle: 'Sign in with Google',
  logout: 'Log out',
  logoutConfirm: 'Are you sure you want to log out?',
  loggedInAs: 'Logged in as',
  syncEnabled: 'Sync enabled',
  syncDisabled: 'Sync disabled',
  lastSynced: 'Last synced',
  pendingChanges: 'Pending changes',
  syncNow: 'Sync now',
  syncing: 'Syncing...',
  syncSuccess: 'Sync complete',
  syncFailed: 'Sync failed',
  justNow: 'Just now',
  minutesAgo: (n: number) => `${n} minutes ago`,
  hoursAgo: (n: number) => `${n} hours ago`,
  never: 'Never',
  cloudSync: 'Cloud Sync',
  cloudSyncDescription: 'Sync your data across devices',
  notConfigured: 'Not configured',
  supabaseNotConfigured: 'Supabase is not configured',

  // Common
  saving: 'Saving...',
};
