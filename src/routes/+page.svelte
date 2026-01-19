<script lang="ts">
  import { onMount } from 'svelte';
  import { flip } from 'svelte/animate';
  import { fly } from 'svelte/transition';
  import { cubicOut } from 'svelte/easing';
  import { goto } from '$app/navigation';
  import { invoke } from '@tauri-apps/api/core';
  import type { TodoItem, Category } from '../types';
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

  // Core app state
  let items = $state<TodoItem[]>([]);
  let categories = $state<Category[]>([]);
  let selectedCategoryId = $state<number | null>(null);

  // Modal control state
  let showResetConfirm = $state(false);
  let showCategoryMenu = $state(false);
  let showDeleteCategoryConfirm = $state(false);
  let showAddItemModal = $state(false);
  let showReorderModal = $state(false);
  let showReorderCategoriesModal = $state(false);
  let selectedCategoryForMenu = $state<Category | null>(null);

  // Edit mode state
  let isEditingItem = $state(false);

  // FAB visibility (for entry animation)
  let showFab = $state(false);
  let fabDelay = $state(300);

  // Safe area bottom padding (measured via JavaScript to avoid iOS WebView bug)
  let safeAreaBottom = $state(0);

  // Reference to CategoryTabs component
  let categoryTabsComponent: any;

  async function loadCategories() {
    try {
      categories = await invoke<Category[]>('get_categories');
      if (categories.length > 0 && selectedCategoryId === null) {
        selectedCategoryId = categories[0].id;
      }
    } catch (error) {
      console.error('Failed to load categories:', error);
    }
  }

  async function loadItems() {
    try {
      items = await invoke<TodoItem[]>('get_items', { categoryId: selectedCategoryId });
    } catch (error) {
      console.error('Failed to load items:', error);
    }
  }

  // Category handlers
  async function selectCategory(categoryId: number) {
    selectedCategoryId = categoryId;
    await loadItems();
  }

  async function handleAddCategory(name: string) {
    const newCategory = await invoke<Category>('add_category', { name });
    categories = [...categories, newCategory];
    await selectCategory(newCategory.id);
  }

  async function handleEditCategory(id: number, name: string) {
    await invoke('edit_category', { id, name });
    categories = categories.map(cat =>
      cat.id === id ? { ...cat, name } : cat
    );
  }

  function handleCategoryLongPress(category: Category) {
    selectedCategoryForMenu = category;
    showCategoryMenu = true;
  }

  function handleEditFromMenu() {
    if (selectedCategoryForMenu && categoryTabsComponent) {
      categoryTabsComponent.triggerEdit(selectedCategoryForMenu);
      showCategoryMenu = false;
      selectedCategoryForMenu = null;
    }
  }

  function handleDeleteFromMenu() {
    showCategoryMenu = false;
    showDeleteCategoryConfirm = true;
  }

  async function confirmDeleteCategory() {
    const categoryToDelete = selectedCategoryForMenu;
    if (!categoryToDelete) return;

    if (categories.length <= 1) {
      alert('최소 1개의 카테고리는 유지해야 합니다.');
      showDeleteCategoryConfirm = false;
      selectedCategoryForMenu = null;
      return;
    }

    try {
      await invoke('delete_category', { id: categoryToDelete.id });
      categories = categories.filter(cat => cat.id !== categoryToDelete.id);
      if (selectedCategoryId === categoryToDelete.id) {
        await selectCategory(categories[0].id);
      }
      showDeleteCategoryConfirm = false;
      selectedCategoryForMenu = null;
    } catch (error) {
      console.error('Failed to delete category:', error);
      alert('카테고리 삭제 실패: ' + error);
    }
  }

  // Reset handler
  async function confirmReset() {
    try {
      await invoke('reset_all_items', { categoryId: selectedCategoryId });
      items = items.map(item => ({ ...item, done: false }));
      showResetConfirm = false;
    } catch (error) {
      console.error('Failed to reset items:', error);
    }
  }

  // Item handlers
  async function addItem(text: string, memo: string | null = null) {
    try {
      const newItem = await invoke<TodoItem>('add_item', { text, categoryId: selectedCategoryId });
      // If memo provided, save it immediately
      if (memo) {
        await invoke('update_item_memo', { id: newItem.id, memo });
        newItem.memo = memo;
      }
      items = [...items, newItem];
    } catch (error) {
      console.error('Failed to add item:', error);
    }
  }

  async function toggleItem(id: number) {
    try {
      await invoke('toggle_item', { id });
      await loadItems();
    } catch (error) {
      console.error('Failed to toggle item:', error);
    }
  }

  async function deleteItem(id: number) {
    try {
      await invoke('delete_item', { id });
      items = items.filter(item => item.id !== id);
    } catch (error) {
      console.error('Failed to delete item:', error);
    }
  }

  async function editItem(id: number, text: string) {
    try {
      await invoke('edit_item', { id, text });
      items = items.map(item =>
        item.id === id ? { ...item, text } : item
      );
    } catch (error) {
      console.error('Failed to edit item:', error);
    }
  }

  async function updateMemo(id: number, memo: string | null) {
    try {
      await invoke('update_item_memo', { id, memo });
      items = items.map(item =>
        item.id === id ? { ...item, memo } : item
      );
    } catch (error) {
      console.error('Failed to update memo:', error);
    }
  }

  function handleItemsReorder(updatedItems: TodoItem[]) {
    items = updatedItems;
  }

  function handleCategoriesReorder(updatedCategories: Category[]) {
    categories = updatedCategories;
  }

  function handleHomeClick() {
    if (categories.length > 0) {
      selectCategory(categories[0].id);
    }
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
      const wasReset = await invoke<boolean>('check_and_auto_reset');
      if (wasReset) {
        console.log('Auto-reset performed for new day');
      }
    } catch (error) {
      console.error('Failed to check auto-reset:', error);
    }

    await loadCategories();
    await loadItems();

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
      {categories}
      {selectedCategoryId}
      onSelectCategory={selectCategory}
      onAddCategory={handleAddCategory}
      onEditCategory={handleEditCategory}
      onCategoryLongPress={handleCategoryLongPress}
      onReorderCategories={() => showReorderCategoriesModal = true}
    />
  </div>

  <!-- Main Content -->
  <main class="main-content max-w-2xl w-full mx-auto flex flex-col">
    <!-- Scrollable Todo List -->
    <div class="todo-list-scroll">
      {#if items.length === 0}
        <div class="p-8 text-center text-ink-muted">
          <p>아직 항목이 없습니다.</p>
          <p class="text-sm mt-1">항목을 추가해보세요!</p>
        </div>
      {:else}
        <div class="item-list">
          {#each items as item (item.id)}
            <div animate:flip={{ duration: 300 }} class="item-wrapper">
              <SwipeableItem {item} onDelete={deleteItem}>
                {#snippet children()}
                  <LeafTodoItem
                    {item}
                    onToggle={toggleItem}
                    onEdit={editItem}
                  >
                    {#snippet drawerContent({ item: drawerItem, closeDrawer })}
                      <MemoDrawer
                        item={drawerItem}
                        onSaveMemo={updateMemo}
                        onEditText={editItem}
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
        onclick={() => showReorderModal = true}
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
        onclick={handleHomeClick}
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
      onclick={() => showAddItemModal = true}
      class="w-14 h-14 bg-accent-sky-strong hover:bg-accent-sky text-white rounded-full shadow-lg flex items-center justify-center transition-all hover:scale-110"
      title="항목 추가"
    >
      <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
      </svg>
    </button>

    <!-- Reset Button -->
    <button
      onclick={() => showResetConfirm = true}
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
    show={showAddItemModal}
    onAdd={addItem}
    onCancel={() => showAddItemModal = false}
  />

  <ReorderItemsModal
    show={showReorderModal}
    {items}
    onItemsReorder={handleItemsReorder}
    onClose={() => showReorderModal = false}
  />

  <ReorderCategoriesModal
    show={showReorderCategoriesModal}
    {categories}
    onCategoriesReorder={handleCategoriesReorder}
    onClose={() => showReorderCategoriesModal = false}
  />

  <!-- Reset Confirmation Modal -->
  <ConfirmModal
    show={showResetConfirm}
    title="체크 초기화"
    message="모든 체크를 초기화하시겠습니까?"
    confirmLabel="초기화"
    confirmClass="bg-orange-500 hover:bg-orange-600"
    onConfirm={confirmReset}
    onCancel={() => showResetConfirm = false}
  />

  <!-- Category Menu Modal -->
  <CategoryMenuModal
    show={showCategoryMenu}
    category={selectedCategoryForMenu}
    onEdit={handleEditFromMenu}
    onDelete={handleDeleteFromMenu}
    onClose={() => {
      showCategoryMenu = false;
      selectedCategoryForMenu = null;
    }}
  />

  <!-- Delete Category Confirmation Modal -->
  <ConfirmModal
    show={showDeleteCategoryConfirm}
    title="카테고리 삭제"
    message={selectedCategoryForMenu
      ? `"${selectedCategoryForMenu.name}" 카테고리를 삭제하시겠습니까?\n항목들도 함께 삭제됩니다.`
      : ''}
    confirmLabel="삭제"
    confirmClass="bg-red-500 hover:bg-red-600"
    onConfirm={confirmDeleteCategory}
    onCancel={() => {
      showDeleteCategoryConfirm = false;
      selectedCategoryForMenu = null;
    }}
  />

  <!-- Intro Animation Component -->
  <IntroAnimation />
</div>
