<script lang="ts">
  import { onMount } from 'svelte';
  import { flip } from 'svelte/animate';
  import { invoke } from '@tauri-apps/api/core';
  import type { TodoItem, Category, Template } from '../types';
  import TodoItemComponent from '../components/TodoItem.svelte';
  import AddItemInput from '../components/AddItemInput.svelte';
  import TemplateSection from '../components/TemplateSection.svelte';
  import SwipeableItem from '../components/SwipeableItem.svelte';
  import { iosFocusFix } from '$lib/iosFocusFix';

  let items = $state<TodoItem[]>([]);
  let categories = $state<Category[]>([]);
  let templates = $state<Template[]>([]);
  let selectedCategoryId = $state<number | null>(null);
  let editingCategoryId = $state<number | null>(null);
  let editingCategoryName = $state('');
  let isAddingCategory = $state(false);
  let newCategoryName = $state('');
  let showResetConfirm = $state(false);
  let showIntro = $state(false);
  let introText = $state('');
  let showCategoryMenu = $state(false);
  let selectedCategoryForMenu = $state<Category | null>(null);
  let longPressTimer: number | null = null;
  let showDeleteCategoryConfirm = $state(false);
  let isSubmittingCategory = $state(false);

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

  function handleCategoryTouchStart(category: Category) {
    longPressTimer = window.setTimeout(() => {
      selectedCategoryForMenu = category;
      showCategoryMenu = true;
    }, 500);
  }

  function handleCategoryTouchEnd() {
    if (longPressTimer) {
      clearTimeout(longPressTimer);
      longPressTimer = null;
    }
  }

  function closeCategoryMenu() {
    showCategoryMenu = false;
    selectedCategoryForMenu = null;
  }

  function handleEditFromMenu() {
    if (selectedCategoryForMenu) {
      startEditCategory(selectedCategoryForMenu);
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

  function cancelDeleteCategory() {
    showDeleteCategoryConfirm = false;
    selectedCategoryForMenu = null;
  }

  function startAddCategory() {
    isAddingCategory = true;
    newCategoryName = '';
    isSubmittingCategory = false;
  }

  async function saveNewCategory() {
    if (newCategoryName.trim()) {
      isSubmittingCategory = true;
      try {
        const newCategory = await invoke<Category>('add_category', { name: newCategoryName.trim() });
        categories = [...categories, newCategory];
        selectedCategoryId = newCategory.id;
        isAddingCategory = false;
        newCategoryName = '';
        await loadItems();
      } catch (error) {
        console.error('Failed to add category:', error);
        isSubmittingCategory = false;
      }
    }
  }

  function cancelAddCategory() {
    if (!isSubmittingCategory) {
      isAddingCategory = false;
      newCategoryName = '';
    }
  }

  function handleAddCategoryBlur() {
    if (isSubmittingCategory) return;

    // If there's a value, save it (for iOS Done button)
    // If empty, cancel
    if (newCategoryName.trim()) {
      saveNewCategory();
    } else {
      cancelAddCategory();
    }
  }

  function startEditCategory(category: Category) {
    editingCategoryId = category.id;
    editingCategoryName = category.name;
  }

  async function saveEditCategory() {
    if (editingCategoryId !== null && editingCategoryName.trim()) {
      try {
        await invoke('edit_category', { id: editingCategoryId, name: editingCategoryName.trim() });
        categories = categories.map(cat =>
          cat.id === editingCategoryId ? { ...cat, name: editingCategoryName.trim() } : cat
        );
        editingCategoryId = null;
        editingCategoryName = '';
      } catch (error) {
        console.error('Failed to edit category:', error);
        alert('카테고리 수정 실패: ' + error);
      }
    }
  }

  function cancelEditCategory() {
    editingCategoryId = null;
    editingCategoryName = '';
  }

  async function deleteCategory(id: number) {
    if (categories.length <= 1) {
      alert('최소 1개의 카테고리는 유지해야 합니다.');
      return;
    }

    if (confirm('이 카테고리를 삭제하시겠습니까? 카테고리 내 항목들은 유지됩니다.')) {
      try {
        await invoke('delete_category', { id });
        categories = categories.filter(cat => cat.id !== id);
        if (selectedCategoryId === id) {
          selectedCategoryId = categories[0].id;
          await loadItems();
        }
      } catch (error) {
        console.error('Failed to delete category:', error);
        alert('카테고리 삭제 실패: ' + error);
      }
    }
  }

  function showResetDialog() {
    showResetConfirm = true;
  }

  async function confirmReset() {
    try {
      await invoke('reset_all_items', { categoryId: selectedCategoryId });
      items = items.map(item => ({ ...item, done: false }));
      showResetConfirm = false;
    } catch (error) {
      console.error('Failed to reset items:', error);
    }
  }

  function cancelReset() {
    showResetConfirm = false;
  }

  async function loadItems() {
    try {
      items = await invoke<TodoItem[]>('get_items', { categoryId: selectedCategoryId });
    } catch (error) {
      console.error('Failed to load items:', error);
    }
  }

  async function addItem(text: string) {
    try {
      const newItem = await invoke<TodoItem>('add_item', { text, categoryId: selectedCategoryId });
      items = [...items, newItem];
    } catch (error) {
      console.error('Failed to add item:', error);
    }
  }

  function selectCategory(categoryId: number) {
    selectedCategoryId = categoryId;
    loadItems();
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

  async function loadTemplates() {
    try {
      templates = await invoke<Template[]>('get_templates');
    } catch (error) {
      console.error('Failed to load templates:', error);
    }
  }

  async function addTemplate(text: string) {
    try {
      const newTemplate = await invoke<Template>('add_template', { text });
      templates = [...templates, newTemplate];
    } catch (error) {
      console.error('Failed to add template:', error);
    }
  }

  async function editTemplate(id: number, text: string) {
    try {
      await invoke('edit_template', { id, text });
      templates = templates.map(t => (t.id === id ? { ...t, text } : t));
    } catch (error) {
      console.error('Failed to edit template:', error);
    }
  }

  async function deleteTemplate(id: number) {
    try {
      await invoke('delete_template', { id });
      templates = templates.filter(t => t.id !== id);
    } catch (error) {
      console.error('Failed to delete template:', error);
    }
  }

  async function useTemplate(templateId: number) {
    if (selectedCategoryId === null) return;
    try {
      const newItem = await invoke<TodoItem>('add_item_from_template', {
        templateId,
        categoryId: selectedCategoryId
      });
      items = [...items, newItem];
    } catch (error) {
      if (typeof error === 'string' && error.includes('이미 존재')) {
        console.log('Item already exists in the list');
        // Could show a toast notification here
      } else {
        console.error('Failed to add item from template:', error);
      }
    }
  }

  async function playIntroAnimation() {
    const fullText = 'Tickly';
    for (let i = 0; i <= fullText.length; i++) {
      introText = fullText.slice(0, i);
      await new Promise(resolve => setTimeout(resolve, 150));
    }
    await new Promise(resolve => setTimeout(resolve, 500));
    showIntro = false;
    localStorage.setItem('tickly_intro_seen', 'true');
  }

  onMount(async () => {
    // Check if first time user
    const hasSeenIntro = localStorage.getItem('tickly_intro_seen');
    if (!hasSeenIntro) {
      showIntro = true;
      playIntroAnimation();
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
    await loadTemplates();
  });
</script>

<div class="app-container bg-gray-100 flex flex-col">
  <!-- Category Tabs -->
  <div class="category-tabs bg-white border-b border-gray-200 flex-shrink-0">
    <div class="max-w-2xl mx-auto px-4">
      <div class="flex overflow-x-auto gap-2 py-3 flex-nowrap scrollbar-hide">
        {#each categories as category (category.id)}
          {#if editingCategoryId === category.id}
            <!-- Editing Mode -->
            <div class="flex items-center gap-1 px-3 py-2 bg-blue-100 rounded-full">
              <input
                use:iosFocusFix
                bind:value={editingCategoryName}
                onkeydown={(e) => {
                  if (e.key === 'Enter') saveEditCategory();
                  if (e.key === 'Escape') cancelEditCategory();
                }}
                onblur={saveEditCategory}
                class="w-24 px-2 py-1 text-sm border border-blue-500 rounded focus:outline-none focus:ring-1 focus:ring-blue-500"
                type="text"
                autofocus
              />
            </div>
          {:else}
            <!-- Normal Mode -->
            <button
              onclick={() => selectCategory(category.id)}
              ontouchstart={() => handleCategoryTouchStart(category)}
              ontouchend={handleCategoryTouchEnd}
              ontouchcancel={handleCategoryTouchEnd}
              class="px-4 py-2 rounded-full text-sm font-medium whitespace-nowrap transition-colors {
                selectedCategoryId === category.id
                  ? 'bg-blue-500 text-white'
                  : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
              }"
            >
              {category.name}
            </button>
          {/if}
        {/each}

        <!-- Add Category Button or Input -->
        {#if isAddingCategory}
          <div class="flex items-center gap-1 px-3 py-2 bg-green-100 rounded-full">
            <input
              use:iosFocusFix
              bind:value={newCategoryName}
              onkeydown={(e) => {
                if (e.key === 'Enter') saveNewCategory();
                if (e.key === 'Escape') cancelAddCategory();
              }}
              onblur={handleAddCategoryBlur}
              class="w-24 px-2 py-1 text-sm border border-green-500 rounded focus:outline-none focus:ring-1 focus:ring-green-500"
              type="text"
              placeholder="카테고리명"
              autofocus
            />
          </div>
        {:else}
          <button
            onclick={startAddCategory}
            class="px-4 py-2 rounded-full text-sm font-medium bg-green-100 text-green-700 hover:bg-green-200 whitespace-nowrap"
            title="카테고리 추가"
          >
            + 추가
          </button>
        {/if}
      </div>
    </div>
  </div>

  <!-- Main Content -->
  <main class="main-content max-w-2xl w-full mx-auto bg-white shadow-lg flex flex-col">
    <!-- Fixed Header Section -->
    <div class="fixed-header flex-shrink-0">
      <AddItemInput onAdd={addItem} />

      <TemplateSection
        {templates}
        onAddTemplate={addTemplate}
        onEditTemplate={editTemplate}
        onDeleteTemplate={deleteTemplate}
        onUseTemplate={useTemplate}
      />
    </div>

    <!-- Scrollable Todo List -->
    <div class="todo-list-scroll">
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

  <!-- Floating Reset Button -->
  <button
    onclick={showResetDialog}
    class="fixed bottom-6 right-6 w-14 h-14 bg-orange-500 hover:bg-orange-600 text-white rounded-full shadow-lg flex items-center justify-center transition-all hover:scale-110"
    title="체크 초기화"
  >
    <svg class="w-7 h-7" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
    </svg>
  </button>

  <!-- Reset Confirmation Modal -->
  {#if showResetConfirm}
    <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50" onclick={cancelReset}>
      <div class="bg-white rounded-lg p-6 max-w-sm mx-4" onclick={(e) => e.stopPropagation()}>
        <h3 class="text-lg font-semibold mb-2">체크 초기화</h3>
        <p class="text-gray-600 mb-6">모든 체크를 초기화하시겠습니까?</p>
        <div class="flex gap-3 justify-end">
          <button
            onclick={cancelReset}
            class="px-4 py-2 text-gray-700 bg-gray-100 hover:bg-gray-200 rounded-lg"
          >
            취소
          </button>
          <button
            onclick={confirmReset}
            class="px-4 py-2 text-white bg-orange-500 hover:bg-orange-600 rounded-lg"
          >
            초기화
          </button>
        </div>
      </div>
    </div>
  {/if}

  <!-- Category Menu Modal -->
  {#if showCategoryMenu && selectedCategoryForMenu}
    <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50" onclick={closeCategoryMenu}>
      <div class="bg-white rounded-lg p-6 max-w-sm mx-4 min-w-[280px]" onclick={(e) => e.stopPropagation()}>
        <h3 class="text-lg font-semibold mb-4 text-center">{selectedCategoryForMenu.name}</h3>
        <div class="flex flex-col gap-2">
          <button
            onclick={handleEditFromMenu}
            class="px-4 py-3 text-blue-600 bg-blue-50 hover:bg-blue-100 rounded-lg text-left font-medium"
          >
            이름 수정
          </button>
          <button
            onclick={handleDeleteFromMenu}
            class="px-4 py-3 text-red-600 bg-red-50 hover:bg-red-100 rounded-lg text-left font-medium"
          >
            카테고리 삭제
          </button>
          <button
            onclick={closeCategoryMenu}
            class="px-4 py-3 text-gray-700 bg-gray-100 hover:bg-gray-200 rounded-lg text-left font-medium"
          >
            취소
          </button>
        </div>
      </div>
    </div>
  {/if}

  <!-- Delete Category Confirmation Modal -->
  {#if showDeleteCategoryConfirm && selectedCategoryForMenu}
    <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50" onclick={cancelDeleteCategory}>
      <div class="bg-white rounded-lg p-6 max-w-sm mx-4" onclick={(e) => e.stopPropagation()}>
        <h3 class="text-lg font-semibold mb-2">카테고리 삭제</h3>
        <p class="text-gray-600 mb-6">
          "{selectedCategoryForMenu.name}" 카테고리를 삭제하시겠습니까?<br/>
          <span class="text-sm text-red-500">항목들도 함께 삭제됩니다.</span>
        </p>
        <div class="flex gap-3 justify-end">
          <button
            onclick={cancelDeleteCategory}
            class="px-4 py-2 text-gray-700 bg-gray-100 hover:bg-gray-200 rounded-lg"
          >
            취소
          </button>
          <button
            onclick={confirmDeleteCategory}
            class="px-4 py-2 text-white bg-red-500 hover:bg-red-600 rounded-lg"
          >
            삭제
          </button>
        </div>
      </div>
    </div>
  {/if}

  <!-- Intro Typing Animation -->
  {#if showIntro}
    <div class="fixed inset-0 bg-white z-50 flex items-center justify-center">
      <h1 class="text-5xl font-bold text-gray-900">{introText}<span class="animate-pulse">|</span></h1>
    </div>
  {/if}
</div>
