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
  import { initializeTheme } from '../lib/themes';
  import { initializeFonts } from '../lib/fonts';
  import { appStore, modalStore } from '../lib/stores';
  import * as todoApi from '../lib/api/todoApi';
  import { i18n } from '$lib/i18n';

  // Local UI state only
  let isEditingItem = $state(false);
  let showFab = $state(false);

  // Track last processed date to avoid duplicate processing
  let lastProcessedDate = '';

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
        console.log(`Reactivated ${reactivatedCount} repeating items`);
        // Reload items to reflect changes
        await appStore.loadItems();
      }
    } catch (error) {
      console.error('Failed to process repeats:', error);
    }
  }

  // Handle visibility change (app coming to foreground)
  function handleVisibilityChange() {
    if (document.visibilityState === 'visible') {
      processRepeatsAndReload();
    }
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

    showFab = true;

    // Listen for visibility changes (app coming back to foreground)
    document.addEventListener('visibilitychange', handleVisibilityChange);
  });

  onDestroy(() => {
    document.removeEventListener('visibilitychange', handleVisibilityChange);
  });
</script>

<div class="app-container bg-paper flex flex-col">
  <!-- Category Tabs Component -->
  <div>
    <CategoryTabs
      bind:this={categoryTabsComponent}
      categories={appStore.categories}
      selectedCategoryId={appStore.selectedCategoryId}
      onSelectCategory={appStore.selectCategory}
      onAddCategory={appStore.addCategory}
      onEditCategory={appStore.editCategory}
      onCategoryLongPress={handleCategoryLongPress}
      onReorderCategories={modalStore.openReorderCategoriesModal}
    />
  </div>

  <!-- Main Content -->
  <main class="main-content max-w-2xl w-full mx-auto flex flex-col flex-1 min-h-0">
    <!-- Scrollable Todo List -->
    <div class="todo-list-scroll">
      {#if appStore.items.length === 0}
        <div class="p-8 text-center text-ink-muted">
          <p>{i18n.t('emptyListTitle')}</p>
          <p class="text-sm mt-1">{i18n.t('emptyListSubtitle')}</p>
        </div>
      {:else}
        <div class="item-list">
          {#each appStore.items as item (item.id)}
            <div animate:flip={{ duration: 300 }} class="item-wrapper">
              <SwipeableItem {item} onDelete={appStore.deleteItem}>
                {#snippet children()}
                  <LeafTodoItem
                    {item}
                    onToggle={appStore.toggleItem}
                    onEdit={appStore.editItem}
                  >
                    {#snippet drawerContent({ item: drawerItem, closeDrawer })}
                      <MemoDrawer
                        item={drawerItem}
                        onSaveMemo={appStore.updateMemo}
                        onEditText={appStore.editItem}
                        onUpdateRepeat={appStore.updateRepeat}
                        onUpdateTrackStreak={appStore.updateTrackStreak}
                        onEditModeChange={(editing) => isEditingItem = editing}
                        {closeDrawer}
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
    onReset={modalStore.openResetConfirm}
    onReorder={modalStore.openReorderModal}
    onStreak={modalStore.openStreakModal}
    onSettings={() => goto('/settings')}
  />

  <!-- Add Item Modal -->
  <AddItemModal
    show={modalStore.showAddItemModal}
    onAdd={appStore.addItem}
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

  <!-- Intro Animation Component -->
  <IntroAnimation />
</div>
