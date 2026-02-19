<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { flip } from 'svelte/animate';
  import { goto } from '$app/navigation';
  import type { Category } from '../types';
  import LeafTodoItem from '../components/LeafTodoItem.svelte';
  import MemoDrawer from '../components/MemoDrawer.svelte';
  import AddItemModal from '../components/AddItemModal.svelte';
  import SwipeableItem from '../components/SwipeableItem.svelte';
  import CategoryTabs from '../components/CategoryTabs.svelte';
  import ConfirmModal from '../components/ConfirmModal.svelte';
  import CategoryMenuModal from '../components/CategoryMenuModal.svelte';
  import ReorderItemsModal from '../components/ReorderItemsModal.svelte';
  import ReorderCategoriesModal from '../components/ReorderCategoriesModal.svelte';
  import IntroAnimation from '../components/IntroAnimation.svelte';
  import FloatingActions from '../components/FloatingActions.svelte';
  import StreakModal from '../components/StreakModal.svelte';
  import TagFilterModal from '../components/TagFilterModal.svelte';
  import EditItemModal from '../components/EditItemModal.svelte';
  import { initializeTheme } from '../lib/themes';
  import { initializeFonts } from '../lib/fonts';
  import { appStore, modalStore } from '../lib/stores';
  import * as todoApi from '../lib/api/todoApi';
  import * as settingsApi from '../lib/api/settingsApi';
  import { i18n } from '$lib/i18n';
  import { scheduleReminder, cancelReminder, rescheduleAll } from '$lib/notification';

  // Local UI state only
  let isEditingItem = $state(false);
  let showFab = $state(false);

  // Edit item modal state (rendered at root to avoid CSS transform clipping)
  let editingItem = $state<import('../types').TodoItem | null>(null);
  let showEditModal = $state(false);
  let capturedCloseDrawer = $state<(() => void) | null>(null);

  function openEditModal(item: import('../types').TodoItem, closeDrawer: () => void) {
    editingItem = item;
    capturedCloseDrawer = closeDrawer;
    showEditModal = true;
    isEditingItem = true;
  }

  function handleEditSave() {
    showEditModal = false;
    isEditingItem = false;
    capturedCloseDrawer?.();
    capturedCloseDrawer = null;
    editingItem = null;
  }

  function handleEditCancel() {
    showEditModal = false;
    isEditingItem = false;
    editingItem = null;
  }

  // Computed display items (tag filter aware)
  let displayItems = $derived(
    appStore.activeTagFilter !== null ? appStore.filteredItems : appStore.items
  );

  // Track last processed date to avoid duplicate processing
  let lastProcessedDate = '';

  // Timer for auto-reset
  let resetTimer: ReturnType<typeof setTimeout> | null = null;

  // Reference to CategoryTabs component
  let categoryTabsComponent: any;

  // Category handlers
  function handleCategoryLongPress(category: Category) {
    modalStore.openCategoryMenu(category);
  }

  function handleEditFromMenu() {
    if (modalStore.selectedCategoryForMenu && categoryTabsComponent) {
      categoryTabsComponent.triggerEdit(modalStore.selectedCategoryForMenu);
      modalStore.closeCategoryMenu();
    }
  }

  async function confirmDeleteCategory() {
    const categoryToDelete = modalStore.selectedCategoryForMenu;
    if (!categoryToDelete) return;

    const success = await appStore.deleteCategory(categoryToDelete.id);
    if (success) {
      modalStore.closeDeleteCategoryConfirm();
    }
  }

  // Reset handler
  async function confirmReset() {
    await appStore.resetAllItems();
    modalStore.closeResetConfirm();
  }

  // Process repeating items and reload if any were reactivated
  async function processRepeatsAndReload() {
    const today = new Date().toISOString().split('T')[0];

    // Skip if already processed today
    if (lastProcessedDate === today) {
      return;
    }

    try {
      const reactivatedCount = await todoApi.processRepeats();
      lastProcessedDate = today;

      if (reactivatedCount > 0) {
        // Reload items to reflect changes
        await appStore.loadItems();
      }
    } catch (error) {
      console.error('Failed to process repeats:', error);
    }
  }

  // Handle visibility change (app coming to foreground)
  async function handleVisibilityChange() {
    if (document.visibilityState === 'visible') {
      await checkAndPerformAutoReset();
      await processRepeatsAndReload();
    }
  }

  // Check and perform auto-reset, then reload items if reset occurred
  async function checkAndPerformAutoReset() {
    try {
      const didReset = await todoApi.checkAndAutoReset();
      if (didReset) {
        await appStore.loadItems();
      }
    } catch (error) {
      console.error('Failed to check auto-reset:', error);
    }
  }

  // Calculate milliseconds until next reset time
  function getMsUntilResetTime(resetTime: string): number {
    const [hours, minutes] = resetTime.split(':').map(Number);
    const now = new Date();
    const resetDate = new Date();
    resetDate.setHours(hours, minutes, 0, 0);

    // If reset time has passed today, schedule for tomorrow
    if (resetDate <= now) {
      resetDate.setDate(resetDate.getDate() + 1);
    }

    return resetDate.getTime() - now.getTime();
  }

  // Schedule timer for next reset time
  async function scheduleResetTimer() {
    // Clear existing timer
    if (resetTimer) {
      clearTimeout(resetTimer);
      resetTimer = null;
    }

    try {
      const resetTime = await settingsApi.getSetting('reset_time');
      if (!resetTime) return;

      const msUntilReset = getMsUntilResetTime(resetTime);

      resetTimer = setTimeout(async () => {
        await checkAndPerformAutoReset();
        // Schedule next reset (for tomorrow)
        await scheduleResetTimer();
      }, msUntilReset);
    } catch (error) {
      console.error('Failed to schedule reset timer:', error);
    }
  }

  // Notification-aware wrappers
  async function handleAddItem(
    text: string,
    memo: string | null,
    repeatType: import('../types').RepeatType,
    repeatDetail: string | null,
    trackStreak: boolean,
    tagNames?: string[],
    reminderAt?: string | null,
    linkedApp?: string | null
  ) {
    await appStore.addItem(text, memo, repeatType, repeatDetail, trackStreak, tagNames ?? [], reminderAt ?? null, linkedApp ?? null);
    // Schedule notification if reminder was set
    if (reminderAt) {
      // Find the newly added item (last in list)
      const lastItem = appStore.items[appStore.items.length - 1];
      if (lastItem) {
        await scheduleReminder(lastItem.id, lastItem.text, reminderAt);
      }
    }
  }

  async function handleUpdateReminder(id: number, reminderAt: string | null) {
    await appStore.updateReminder(id, reminderAt);
    if (reminderAt) {
      const item = appStore.items.find(i => i.id === id);
      await scheduleReminder(id, item?.text ?? '', reminderAt);
    } else {
      await cancelReminder(id);
    }
  }

  async function handleToggleItem(id: number) {
    const item = appStore.items.find(i => i.id === id);
    const wasDone = item?.done;
    await appStore.toggleItem(id);
    if (!wasDone && item?.reminder_at) {
      // Item was completed → cancel reminder
      await cancelReminder(id);
    } else if (wasDone && item?.reminder_at) {
      // Item was re-activated → reschedule reminder
      await scheduleReminder(id, item.text, item.reminder_at);
    }
  }

  async function handleDeleteItem(id: number) {
    await cancelReminder(id);
    await appStore.deleteItem(id);
  }

  onMount(async () => {
    // Initialize theme, fonts, and locale from saved settings
    await initializeTheme();
    await initializeFonts();
    await i18n.loadLocale();

    // Process repeating items that are due
    await processRepeatsAndReload();

    await appStore.loadCategories();
    await appStore.loadItems();
    await appStore.loadAllTags();
    await appStore.loadTagsForItems(appStore.items);

    // Schedule auto-reset timer
    await scheduleResetTimer();

    // Reschedule all notifications
    await rescheduleAll(appStore.items);

    showFab = true;

    // Listen for visibility changes (app coming back to foreground)
    document.addEventListener('visibilitychange', handleVisibilityChange);
  });

  onDestroy(() => {
    document.removeEventListener('visibilitychange', handleVisibilityChange);
    // Clear reset timer
    if (resetTimer) {
      clearTimeout(resetTimer);
      resetTimer = null;
    }
  });
</script>

<div class="app-container bg-paper flex flex-col">
  <!-- Category Tabs Component -->
  <div>
    <CategoryTabs
      bind:this={categoryTabsComponent}
      categories={appStore.categories}
      selectedCategoryId={appStore.selectedCategoryId}
      hasActiveTagFilter={appStore.activeTagFilter !== null}
      onSelectCategory={appStore.selectCategory}
      onAddCategory={appStore.addCategory}
      onEditCategory={appStore.editCategory}
      onCategoryLongPress={handleCategoryLongPress}
      onReorderCategories={modalStore.openReorderCategoriesModal}
    />
  </div>

  <!-- Main Content -->
  <main class="main-content max-w-2xl w-full mx-auto flex flex-col flex-1 min-h-0">
    <!-- Tag Filter Banner -->
    {#if appStore.activeTagFilter !== null}
      {@const activeTag = appStore.allTags.find(t => t.id === appStore.activeTagFilter)}
      <div class="tag-filter-banner">
        <span class="filter-text">
          <svg class="filter-icon" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A2 2 0 013 12V7a4 4 0 014-4z" />
          </svg>
          {activeTag?.name ?? ''}
        </span>
        <button class="filter-clear-btn" aria-label="Clear filter" onclick={() => appStore.clearTagFilter()}>
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="18" y1="6" x2="6" y2="18"></line>
            <line x1="6" y1="6" x2="18" y2="18"></line>
          </svg>
        </button>
      </div>
    {/if}

    <!-- Scrollable Todo List -->
    <div class="todo-list-scroll">
      {#if displayItems.length === 0}
        <div class="p-8 text-center text-ink-muted">
          <p>{i18n.t('emptyListTitle')}</p>
          <p class="text-sm mt-1">{i18n.t('emptyListSubtitle')}</p>
        </div>
      {:else}
        <div class="item-list">
          {#each displayItems as item (item.id)}
            <div animate:flip={{ duration: 300 }} class="item-wrapper">
              <SwipeableItem {item} onDelete={handleDeleteItem}>
                {#snippet children()}
                  <LeafTodoItem
                    {item}
                    itemTags={appStore.itemTagsMap[item.id] ?? []}
                    onToggle={handleToggleItem}
                    onEdit={appStore.editItem}
                  >
                    {#snippet drawerContent({ item: drawerItem, closeDrawer })}
                      <MemoDrawer
                        item={drawerItem}
                        itemTags={appStore.itemTagsMap[drawerItem.id] ?? []}
                        onOpenEdit={(item) => openEditModal(item, closeDrawer)}
                      />
                    {/snippet}
                  </LeafTodoItem>
                {/snippet}
              </SwipeableItem>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  </main>

  <!-- Floating Action Buttons -->
  <FloatingActions
    show={showFab && !isEditingItem}
    onAdd={modalStore.openAddItemModal}
    onGraph={() => goto('/graph')}
    onReorder={modalStore.openReorderModal}
    onStreak={modalStore.openStreakModal}
    onTagFilter={modalStore.openTagFilterModal}
    onSettings={() => goto('/settings')}
  />

  <!-- Add Item Modal -->
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

  <!-- Reset Confirmation Modal -->
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

  <!-- Category Menu Modal -->
  <CategoryMenuModal
    show={modalStore.showCategoryMenu}
    category={modalStore.selectedCategoryForMenu}
    onEdit={handleEditFromMenu}
    onDelete={modalStore.openDeleteCategoryConfirm}
    onClose={modalStore.closeCategoryMenu}
  />

  <!-- Delete Category Confirmation Modal -->
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

  <!-- Streak Heatmap Modal -->
  <StreakModal
    show={modalStore.showStreakModal}
    onClose={modalStore.closeStreakModal}
  />

  <!-- Tag Filter Modal -->
  <TagFilterModal
    show={modalStore.showTagFilterModal}
    tags={appStore.allTags}
    activeTagId={appStore.activeTagFilter}
    onSelect={appStore.setTagFilter}
    onClear={appStore.clearTagFilter}
    onClose={modalStore.closeTagFilterModal}
  />

  <!-- Edit Item Modal (rendered at root to avoid CSS transform clipping) -->
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

  <!-- Intro Animation Component -->
  <IntroAnimation />
</div>
