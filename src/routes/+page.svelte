<script lang="ts">
  import { goto } from '$app/navigation';
  import { flip } from 'svelte/animate';
  import { onDestroy, onMount } from 'svelte';

  import type { Category, RepeatType, TodoItem } from '../types';

  import ActiveTagFilterBanner from '../components/ActiveTagFilterBanner.svelte';
  import CategoryTabs from '../components/CategoryTabs.svelte';
  import FloatingActions from '../components/FloatingActions.svelte';
  import HomeModals from '../components/HomeModals.svelte';
  import IntroAnimation from '../components/IntroAnimation.svelte';
  import LeafTodoItem from '../components/LeafTodoItem.svelte';
  import MemoDrawer from '../components/MemoDrawer.svelte';
  import SwipeableItem from '../components/SwipeableItem.svelte';
  import { initializeFonts } from '../lib/fonts';
  import { createHomeItemActions } from '../lib/home/homeItemActions';
  import { createHomeLifecycle } from '../lib/home/homeLifecycle';
  import { i18n } from '../lib/i18n';
  import { cancelReminder, rescheduleAll, scheduleReminder } from '../lib/notification';
  import { appStore, modalStore } from '../lib/stores';
  import { initializeTheme } from '../lib/themes';
  import * as settingsApi from '../lib/api/settingsApi';
  import * as todoApi from '../lib/api/todoApi';

  // Local UI state only
  let isEditingItem = $state(false);
  let showFab = $state(false);

  // Edit item modal state (rendered at root to avoid CSS transform clipping)
  let editingItem = $state<TodoItem | null>(null);
  let showEditModal = $state(false);
  let capturedCloseDrawer = $state<(() => void) | null>(null);

  function openEditModal(item: TodoItem, closeDrawer: () => void) {
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
  let activeTagName = $derived(
    appStore.activeTagFilter !== null
      ? (appStore.allTags.find((tag) => tag.id === appStore.activeTagFilter)?.name ?? '')
      : ''
  );

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

  async function confirmReset() {
    await appStore.resetAllItems();
    modalStore.closeResetConfirm();
  }

  const homeLifecycle = createHomeLifecycle({
    loadItems: appStore.loadItems,
    loadTagsForItems: appStore.loadTagsForItems,
    getItems: () => appStore.items,
    processRepeats: todoApi.processRepeats,
    checkAndAutoReset: todoApi.checkAndAutoReset,
    getResetTime: () => settingsApi.getSetting('reset_time'),
    rescheduleAll,
  });

  const {
    processRepeatsAndReload,
    scheduleResetTimer,
    clearResetTimer,
    handleVisibilityChange,
  } = homeLifecycle;

  const {
    handleAddItem,
    handleUpdateReminder,
    handleToggleItem,
    handleDeleteItem,
  } = createHomeItemActions({
    addItem: appStore.addItem,
    updateReminder: appStore.updateReminder,
    toggleItem: appStore.toggleItem,
    deleteItem: appStore.deleteItem,
    getItems: () => appStore.items,
    scheduleReminder,
    cancelReminder,
  });

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
    clearResetTimer();
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
    <ActiveTagFilterBanner
      show={appStore.activeTagFilter !== null}
      tagName={activeTagName}
      onClear={appStore.clearTagFilter}
    />

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

  <HomeModals
    {handleAddItem}
    {handleEditFromMenu}
    {confirmReset}
    {confirmDeleteCategory}
    {editingItem}
    {showEditModal}
    {handleUpdateReminder}
    {handleEditSave}
    {handleEditCancel}
  />

  <!-- Intro Animation Component -->
  <IntroAnimation />
</div>
