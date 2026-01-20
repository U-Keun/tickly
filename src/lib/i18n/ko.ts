export const ko = {
  // Main page - empty state
  emptyListTitle: '아직 항목이 없습니다.',
  emptyListSubtitle: '항목을 추가해보세요!',

  // Bottom navigation
  reorder: '순서 바꾸기',
  home: '홈',
  settings: '설정',

  // FAB buttons
  addItem: '항목 추가',
  resetCheck: '체크 초기화',

  // AddItemModal
  addItemTitle: '항목 추가',
  todoLabel: '할 일',
  todoPlaceholder: '할 일을 입력하세요',
  memoLabel: '메모 (선택)',
  memoPlaceholder: '메모를 입력하세요',
  cancel: '취소',
  add: '추가',
  confirm: '확인',

  // Reset confirm
  resetConfirmTitle: '체크 초기화',
  resetConfirmMessage: '모든 체크를 초기화하시겠습니까?',
  reset: '초기화',

  // Category
  categoryDelete: '카테고리 삭제',
  categoryDeleteConfirm: '카테고리를 삭제하시겠습니까?\n항목들도 함께 삭제됩니다.',
  categoryDeleteConfirmTemplate: (name: string) =>
    `"${name}" 카테고리를 삭제하시겠습니까?\n항목들도 함께 삭제됩니다.`,
  delete: '삭제',
  edit: '수정',
  editName: '이름 수정',
  categoryPlaceholder: '카테고리명',
  addCategory: '카테고리 추가',
  categoryEditFailed: '카테고리 수정 실패: ',
  reorderCategories: '카테고리 순서 변경',

  // Settings
  settingsTitle: '설정',
  themeChange: '테마 변경',
  languageChange: '언어 변경',
  back: '뒤로 가기',

  // Language settings
  languageTitle: '언어',
  korean: '한국어',
  english: 'English',

  // Reorder items modal
  reorderItemsTitle: '항목 순서 정렬',
  reorderItemsSubtitle: '드래그해서 현재 카테고리의 순서를 바꿔보세요.',
  close: '닫기',
  noItemsToReorder: '정렬할 항목이 없습니다.',
  done: '완료',

  // Reorder categories modal
  reorderCategoriesTitle: '카테고리 순서 정렬',
  reorderCategoriesSubtitle: '드래그해서 카테고리 순서를 바꿔보세요.',
  noCategoriesToReorder: '정렬할 카테고리가 없습니다.',

  // MemoDrawer
  itemLabel: '항목',
  todoPlaceholderAlt: '할 일을 입력하세요...',
  memoPlaceholderAlt: '메모를 입력하세요...',
  save: '저장',
};

export type Translations = typeof ko;
