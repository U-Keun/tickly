<script lang="ts">
  import { onMount } from 'svelte';
  import { flip } from 'svelte/animate';
  import { fly } from 'svelte/transition';
  import { cubicOut } from 'svelte/easing';
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
  import { initializeTheme } from '../lib/themes';
  import { appStore, modalStore } from '../lib/stores';
  import * as todoApi from '../lib/api/todoApi';

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
    // Initialize theme from saved settings
    await initializeTheme();

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
  <main class="main-content max-w-2xl w-full mx-auto flex flex-col">
    <!-- Scrollable Todo List -->
    <div class="todo-list-scroll">
      {#if appStore.items.length === 0}
        <div class="p-8 text-center text-ink-muted">
          <p>아직 항목이 없습니다.</p>
          <p class="text-sm mt-1">항목을 추가해보세요!</p>
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
  <nav class="fixed bottom-0 left-0 right-0 z-20" style="background: var(--color-white); border-top: 1px solid var(--color-border);">
    <div class="flex justify-around items-center h-14">
      <!-- Reorder Button -->
      <button
        class="flex flex-col items-center justify-center flex-1 h-full"
        style="color: var(--color-ink-muted);"
        title="순서 바꾸기"
        onclick={modalStore.openReorderModal}
      >
        <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
        </svg>
      </button>

      <!-- Home Button -->
      <button
        class="flex flex-col items-center justify-center flex-1 h-full"
        style="color: var(--color-ink-muted);"
        title="홈"
        onclick={appStore.goToFirstCategory}
      >
        <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
        </svg>
      </button>

      <!-- Settings Button -->
      <button
        class="flex flex-col items-center justify-center flex-1 h-full"
        style="color: var(--color-ink-muted);"
        title="설정"
        onclick={() => goto('/settings')}
      >
        <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
        </svg>
      </button>
    </div>
  </nav>

  <!-- Floating Action Buttons -->
  {#if showFab && !isEditingItem}
  <div
    class="fixed bottom-12 right-6 flex flex-col gap-3 items-center z-10 transition-[margin] duration-300"
    style="margin-bottom: {safeAreaBottom}px;"
    in:fly={{ y: 100, duration: 400, delay: fabDelay, easing: cubicOut }}
    out:fly={{ y: 100, duration: 300, easing: cubicOut }}
  >
    <!-- Add Button -->
    <button
      onclick={modalStore.openAddItemModal}
      class="w-14 h-14 bg-accent-sky-strong hover:bg-accent-sky text-white rounded-full shadow-lg flex items-center justify-center transition-all hover:scale-110"
      title="항목 추가"
    >
      <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
      </svg>
    </button>

    <!-- Reset Button -->
    <button
      onclick={modalStore.openResetConfirm}
      class="w-14 h-14 bg-accent-peach-strong hover:bg-accent-peach text-ink rounded-full shadow-lg flex items-center justify-center transition-all hover:scale-110"
      title="체크 초기화"
    >
      <svg class="w-7 h-7" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
      </svg>
    </button>
  </div>
  {/if}

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
    title="체크 초기화"
    message="모든 체크를 초기화하시겠습니까?"
    confirmLabel="초기화"
    confirmClass="bg-orange-500 hover:bg-orange-600"
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
    title="카테고리 삭제"
    message={modalStore.selectedCategoryForMenu
      ? `"${modalStore.selectedCategoryForMenu.name}" 카테고리를 삭제하시겠습니까?\n항목들도 함께 삭제됩니다.`
      : ''}
    confirmLabel="삭제"
    confirmClass="bg-red-500 hover:bg-red-600"
    onConfirm={confirmDeleteCategory}
    onCancel={modalStore.closeDeleteCategoryConfirm}
  />

  <!-- Intro Animation Component -->
  <IntroAnimation />
</div>
