<script lang="ts">
  import { onMount } from 'svelte';
  import { flip } from 'svelte/animate';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import type { TodoItem, Category } from '../types';
  import TodoItemComponent from '../components/TodoItem.svelte';
  import AddItemInput from '../components/AddItemInput.svelte';
  import SwipeableItem from '../components/SwipeableItem.svelte';
  import CategoryTabs from '../components/CategoryTabs.svelte';
  import ConfirmModal from '../components/ConfirmModal.svelte';
  import CategoryMenuModal from '../components/CategoryMenuModal.svelte';
  import ResetButton from '../components/ResetButton.svelte';
  import IntroAnimation from '../components/IntroAnimation.svelte';

  // Core app state
  let items = $state<TodoItem[]>([]);
  let categories = $state<Category[]>([]);
  let selectedCategoryId = $state<number | null>(null);

  // Platform detection
  let isIOS = $state(false);

  // Modal control state
  let showResetConfirm = $state(false);
  let showCategoryMenu = $state(false);
  let showDeleteCategoryConfirm = $state(false);
  let selectedCategoryForMenu = $state<Category | null>(null);

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

    // Update native input category on iOS
    if (isIOS) {
      try {
        await invoke('set_native_selected_category', { categoryId });
      } catch (error) {
        console.error('Failed to set native selected category:', error);
      }
    }
  }

  async function handleAddCategory(name: string) {
    const newCategory = await invoke<Category>('add_category', { name });
    categories = [...categories, newCategory];
    selectedCategoryId = newCategory.id;
    await loadItems();
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
    if (!selectedCategoryForMenu) return;

    if (categories.length <= 1) {
      alert('최소 1개의 카테고리는 유지해야 합니다.');
      showDeleteCategoryConfirm = false;
      selectedCategoryForMenu = null;
      return;
    }

    try {
      await invoke('delete_category', { id: selectedCategoryForMenu.id });
      categories = categories.filter(cat => cat.id !== selectedCategoryForMenu.id);
      if (selectedCategoryId === selectedCategoryForMenu.id) {
        selectedCategoryId = categories[0].id;
        await loadItems();
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
  async function addItem(text: string) {
    try {
      const newItem = await invoke<TodoItem>('add_item', { text, categoryId: selectedCategoryId });
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

  onMount(async () => {
    // Detect platform
    try {
      const platform = await invoke<string>('get_platform');
      isIOS = platform === 'ios';
    } catch (error) {
      console.error('Failed to detect platform:', error);
    }

    // Setup event listeners for native UI (iOS only)
    if (isIOS) {
      // Listen for items added from native input
      const unlistenItemAdded = await listen('native-item-added', async () => {
        await loadItems();
      });
    }

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

    // Set initial category for native input on iOS
    if (isIOS && selectedCategoryId) {
      try {
        await invoke('set_native_selected_category', { categoryId: selectedCategoryId });
      } catch (error) {
        console.error('Failed to set initial native selected category:', error);
      }
    }
  });
</script>

<div class="app-container bg-gray-100 flex flex-col">
  <!-- Category Tabs Component -->
  <div class="{isIOS ? 'fixed top-[60px] left-0 right-0 z-10 bg-white' : ''}">
    <CategoryTabs
      bind:this={categoryTabsComponent}
      {categories}
      {selectedCategoryId}
      onSelectCategory={selectCategory}
      onAddCategory={handleAddCategory}
      onEditCategory={handleEditCategory}
      onCategoryLongPress={handleCategoryLongPress}
    />
  </div>

  <!-- Main Content -->
  <main class="main-content max-w-2xl w-full mx-auto bg-white shadow-lg flex flex-col {isIOS ? 'fixed top-[104px] left-0 right-0 bottom-0' : ''}">
    <!-- Fixed Header Section (hidden on iOS - using native UI) -->
    {#if !isIOS}
      <div class="fixed-header flex-shrink-0">
        <AddItemInput onAdd={addItem} />
      </div>
    {/if}

    <!-- Scrollable Todo List -->
    <div class="todo-list-scroll {isIOS ? 'py-4' : ''}">
      {#if items.length === 0}
        <div class="p-8 text-center text-gray-400">
          <p>아직 항목이 없습니다.</p>
          <p class="text-sm mt-1">챙겨야 할 물건을 추가해보세요!</p>
        </div>
      {:else}
        <div class="divide-y divide-gray-200">
          {#each items as item (item.id)}
            <div animate:flip={{ duration: 300 }}>
              <SwipeableItem {item} onDelete={deleteItem}>
                {#snippet children()}
                  <TodoItemComponent
                    {item}
                    onToggle={toggleItem}
                    onDelete={deleteItem}
                    onEdit={editItem}
                  />
                {/snippet}
              </SwipeableItem>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  </main>

  <!-- Reset Button Component -->
  <ResetButton onReset={() => showResetConfirm = true} />

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
