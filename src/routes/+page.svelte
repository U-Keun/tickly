<script lang="ts">
  import { onMount } from 'svelte';
  import { flip } from 'svelte/animate';
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
  import BottomNav from '../components/BottomNav.svelte';
  import FloatingActions from '../components/FloatingActions.svelte';
  import { initializeTheme } from '../lib/themes';
  import { initializeFonts } from '../lib/fonts';
  import { appStore, modalStore } from '../lib/stores';
  import * as todoApi from '../lib/api/todoApi';
  import { i18n } from '$lib/i18n';

  // Local UI state only
  let isEditingItem = $state(false);
  let showFab = $state(false);
  let fabDelay = $state(300);
  let safeAreaBottom = $state(0);

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

  function measureSafeArea(): number {
    const testEl = document.createElement('div');
    testEl.style.cssText = 'position:fixed;bottom:0;height:env(safe-area-inset-bottom,0);visibility:hidden;pointer-events:none;';
    document.body.appendChild(testEl);
    const height = testEl.offsetHeight;
    document.body.removeChild(testEl);
    return height;
  }

  onMount(async () => {
    // Initialize theme, fonts, and locale from saved settings
    await initializeTheme();
    await initializeFonts();
    await i18n.loadLocale();

    // Measure safe area after iOS WebView stabilizes
    // Normal iPhone safe area is around 34px, reject abnormal values (>50px or <=0)
    const applySafeArea = () => {
      const measured = measureSafeArea();
      if (measured > 0 && measured <= 50) {
        safeAreaBottom = measured;
        sessionStorage.setItem('safeAreaBottom', String(measured));
      } else if (measured > 50) {
        // Abnormally large value - use standard iPhone safe area
        safeAreaBottom = 34;
        sessionStorage.setItem('safeAreaBottom', '34');
      }
    };

    // Use cached value if available, otherwise measure
    const cachedSafeArea = sessionStorage.getItem('safeAreaBottom');
    if (cachedSafeArea) {
      safeAreaBottom = Number(cachedSafeArea);
    } else {
      // Try multiple times with increasing delays (first load only)
      setTimeout(applySafeArea, 2000);
      setTimeout(applySafeArea, 3000);
    }

    // Also update on orientation change
    window.addEventListener('resize', applySafeArea);

    // Check and auto-reset if new day
    try {
      const wasReset = await todoApi.checkAndAutoReset();
      if (wasReset) {
        console.log('Auto-reset performed for new day');
      }
    } catch (error) {
      console.error('Failed to check auto-reset:', error);
    }

    await appStore.loadCategories();
    await appStore.loadItems();

    // Show FAB with animation (no delay if intro already played)
    if (sessionStorage.getItem('introPlayed')) {
      fabDelay = 0;
    }
    showFab = true;
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

  <!-- Bottom Navigation Bar -->
  <BottomNav
    onReorder={modalStore.openReorderModal}
    onHome={appStore.goToFirstCategory}
  />

  <!-- Floating Action Buttons -->
  <FloatingActions
    show={showFab && !isEditingItem}
    {safeAreaBottom}
    delay={fabDelay}
    onAdd={modalStore.openAddItemModal}
    onReset={modalStore.openResetConfirm}
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

  <!-- Intro Animation Component -->
  <IntroAnimation />
</div>
