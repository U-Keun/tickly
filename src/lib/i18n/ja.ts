import type { Translations } from './ko';

export const ja: Translations = {
  // Main page - empty state
  emptyListTitle: 'まだ項目がありません。',
  emptyListSubtitle: '項目を追加してみましょう！',

  // Bottom navigation
  reorder: '並べ替え',
  home: 'ホーム',
  settings: '設定',

  // FAB buttons
  addItem: '項目を追加',
  resetCheck: 'チェックをリセット',
  menu: 'メニュー',

  // AddItemModal
  addItemTitle: '項目を追加',
  todoLabel: '項目',
  todoPlaceholder: '項目を入力してください',
  memoLabel: 'メモ（任意）',
  memoPlaceholder: 'メモを入力してください',
  cancel: 'キャンセル',
  add: '追加',

  // Reset confirm
  resetConfirmTitle: 'チェックをリセット',
  resetConfirmMessage: 'すべてのチェックをリセットしますか？',
  reset: 'リセット',

  // Category
  categoryDelete: 'カテゴリを削除',
  categoryDeleteConfirmTemplate: (name: string) =>
    `「${name}」カテゴリを削除しますか？\n項目もすべて削除されます。`,
  delete: '削除',
  edit: '編集',
  editName: '名前を編集',
  categoryPlaceholder: 'カテゴリ名',
  addCategory: 'カテゴリを追加',
  categoryEditFailed: 'カテゴリの編集に失敗しました：',
  reorderCategories: 'カテゴリの並べ替え',

  // Settings
  settingsTitle: '設定',
  themeChange: 'テーマ変更',
  languageChange: '言語変更',
  back: '戻る',

  // Language settings
  languageTitle: '言語',

  // Reorder items modal
  reorderItemsTitle: '項目の並べ替え',
  reorderItemsSubtitle: 'ドラッグして現在のカテゴリの順序を変更できます。',
  close: '閉じる',
  noItemsToReorder: '並べ替える項目がありません。',
  done: '完了',

  // Reorder categories modal
  reorderCategoriesTitle: 'カテゴリの並べ替え',
  reorderCategoriesSubtitle: 'ドラッグしてカテゴリの順序を変更できます。',
  noCategoriesToReorder: '並べ替えるカテゴリがありません。',

  // MemoDrawer
  todoPlaceholderAlt: 'タスクを入力してください...',
  memoPlaceholderAlt: 'メモを入力してください...',
  save: '保存',

  // Theme settings
  themeTitle: 'テーマ設定',
  presetTheme: 'プリセットテーマ',
  preview: 'プレビュー',
  customColors: 'カスタムカラー',
  custom: 'カスタム',

  // Theme presets
  themeDefault: 'デフォルト',
  themeDark: 'ダーク',
  themeOcean: 'オーシャン',
  themeForest: 'フォレスト',
  themeSunset: 'サンセット',

  // Color labels
  colorPaper: '背景（Paper）',
  colorCanvas: 'キャンバス（Canvas）',
  colorMist: 'ミスト（Mist）',
  colorStroke: 'ストローク（Stroke）',
  colorInk: 'テキスト（Ink）',
  colorInkMuted: 'テキスト薄（Ink Muted）',
  colorAccentSky: 'スカイ（Sky）',
  colorAccentSkyStrong: 'スカイ濃（Sky Strong）',
  colorAccentMint: 'ミント（Mint）',
  colorAccentMintStrong: 'ミント濃（Mint Strong）',
  colorAccentPeach: 'ピーチ（Peach）',
  colorAccentPeachStrong: 'ピーチ濃（Peach Strong）',
  colorWhite: '白（White）',
  colorBorder: '境界線（Border）',

  // Font settings
  fontChange: 'フォント変更',
  fontTitle: 'フォント設定',
  fontPreset: 'フォント選択',
  fontSize: 'サイズ',
  fontSizeSmall: '小',
  fontSizeMedium: '中',
  fontSizeLarge: '大',
  fontSystem: 'システムデフォルト',
  fontNotoSans: 'Noto Sans JP',
  fontPretendard: 'Pretendard',
  fontMonospace: 'モノスペース',
  fontPreviewText: 'あいうえおかきくけこ ABCDEFG 1234567890',

  // Repeat settings
  repeatLabel: '繰り返し',
  repeatNone: 'なし',
  repeatDaily: '毎日',
  repeatWeekly: '毎週',
  repeatMonthly: '毎月',
  repeatDaysLabel: '繰り返す曜日',
  repeatDatesLabel: '繰り返す日付',
  sun: '日',
  mon: '月',
  tue: '火',
  wed: '水',
  thu: '木',
  fri: '金',
  sat: '土',

  // Streak heatmap
  streak: 'ストリーク',
  streakHeatmapTitle: 'ストリーク',
  totalDays: '合計完了日数',
  currentStreak: '現在のストリーク',
  longestStreak: '最長ストリーク',
  loading: '読み込み中...',
  trackStreak: 'ストリーク追跡',
  trackingStreak: 'ストリーク追跡中',
  noTrackedItems: '追跡中の項目がありません',
  addStreakHint: '項目の詳細でストリーク追跡を有効にしましょう！',

  // Reset time settings
  resetTimeChange: 'リセット時間',
  resetTimeTitle: 'リセット時間',
  resetTimeDescription: '毎日のタスクがリセットされる時間',

  // Account & Sync settings
  syncTitle: '同期',
  loginRequired: 'ログインが必要です',
  loginDescription: 'クラウド同期を使用するにはログインしてください',
  signInWithApple: 'Appleでサインイン',
  signInWithGoogle: 'Googleでサインイン',
  logout: 'ログアウト',
  logoutConfirm: '本当にログアウトしますか？',
  syncEnabled: '同期を有効化',
  lastSynced: '最終同期',
  pendingChanges: '保留中の変更',
  syncNow: '今すぐ同期',
  syncing: '同期中...',
  never: 'なし',
  cloudSync: 'クラウド同期',

  // Realtime sync
  realtimeSync: 'リアルタイム同期',
  realtimeConnected: '接続済み',
  realtimeConnecting: '接続中...',
  realtimeReconnecting: '再接続中...',
  realtimeDisconnected: '未接続',

  // Tags
  tags: 'タグ',
  tagFilter: 'タグフィルター',
  tagAdd: 'タグを追加',
  tagPlaceholder: 'タグを入力...',
  tagEmpty: 'タグがありません',
  tagFilterClear: 'フィルター解除',
  tagFilterActive: 'タグフィルター適用中',
  tagManage: 'タグ管理',
  tagDeleteConfirmTemplate: (name: string) =>
    `「${name}」タグを削除しますか？\nすべての項目から削除されます。`,

  // Graph view
  graphView: 'グラフビュー',
  graphEmpty: '項目がありません',
  graphEmptyHint: '項目を追加するとグラフに表示されます',

  // Reminder / Notification
  reminder: 'リマインダー',
  reminderSet: 'リマインダー設定済み',
  reminderPlaceholder: '時間を選択...',
  reminderClear: 'リマインダー解除',

  // Advanced settings
  advancedSettings: '詳細設定',

  // Common
  saving: '保存中...',
};
