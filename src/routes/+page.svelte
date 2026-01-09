<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import type { TodoItem } from '../types';
  import TodoItemComponent from '../components/TodoItem.svelte';
  import AddItemInput from '../components/AddItemInput.svelte';

  let items = $state<TodoItem[]>([]);

  async function loadItems() {
    try {
      items = await invoke<TodoItem[]>('get_items');
    } catch (error) {
      console.error('Failed to load items:', error);
    }
  }

  async function addItem(text: string) {
    try {
      const newItem = await invoke<TodoItem>('add_item', { text });
      items = [...items, newItem];
    } catch (error) {
      console.error('Failed to add item:', error);
    }
  }

  async function toggleItem(id: number) {
    try {
      await invoke('toggle_item', { id });
      items = items.map(item =>
        item.id === id ? { ...item, done: !item.done } : item
      );
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

  onMount(() => {
    loadItems();
  });
</script>

<div class="min-h-screen bg-gray-100 flex flex-col">
  <!-- Header -->
  <header class="bg-white shadow-sm">
    <div class="max-w-2xl mx-auto px-4 py-6">
      <h1 class="text-2xl font-bold text-gray-900">Tickly</h1>
      <p class="text-sm text-gray-500 mt-1">외출 전 체크리스트</p>
    </div>
  </header>

  <!-- Main Content -->
  <main class="flex-1 max-w-2xl w-full mx-auto bg-white shadow-lg">
    <AddItemInput onAdd={addItem} />

    <!-- Todo List -->
    <div class="divide-y divide-gray-200">
      {#if items.length === 0}
        <div class="p-8 text-center text-gray-400">
          <p>아직 항목이 없습니다.</p>
          <p class="text-sm mt-1">챙겨야 할 물건을 추가해보세요!</p>
        </div>
      {:else}
        {#each items as item (item.id)}
          <TodoItemComponent
            {item}
            onToggle={toggleItem}
            onDelete={deleteItem}
          />
        {/each}
      {/if}
    </div>
  </main>
</div>
