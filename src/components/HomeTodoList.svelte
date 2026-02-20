<script lang="ts">
  import { flip } from 'svelte/animate';

  import type { Tag, TodoItem } from '../types';

  import LeafTodoItem from './LeafTodoItem.svelte';
  import MemoDrawer from './MemoDrawer.svelte';
  import SwipeableItem from './SwipeableItem.svelte';
  import { i18n } from '$lib/i18n';

  interface Props {
    displayItems: TodoItem[];
    itemTagsMap: Record<number, Tag[]>;
    onDeleteItem: (id: number) => void | Promise<void>;
    onToggleItem: (id: number) => void | Promise<void>;
    onEditItem: (id: number, text: string) => void | Promise<void>;
    onOpenEditModal: (item: TodoItem, closeDrawer: () => void) => void;
  }

  let {
    displayItems,
    itemTagsMap,
    onDeleteItem,
    onToggleItem,
    onEditItem,
    onOpenEditModal
  }: Props = $props();
</script>

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
          <SwipeableItem {item} onDelete={onDeleteItem}>
            {#snippet children()}
              <LeafTodoItem
                {item}
                itemTags={itemTagsMap[item.id] ?? []}
                onToggle={onToggleItem}
                onEdit={onEditItem}
              >
                {#snippet drawerContent({ item: drawerItem, closeDrawer })}
                  <MemoDrawer
                    item={drawerItem}
                    itemTags={itemTagsMap[drawerItem.id] ?? []}
                    onOpenEdit={(nextItem) => onOpenEditModal(nextItem, closeDrawer)}
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
