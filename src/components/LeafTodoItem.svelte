<script lang="ts">
  import { slide } from 'svelte/transition';
  import { cubicOut } from 'svelte/easing';
  import type { TodoItem, Tag } from '../types';
  import type { Snippet } from 'svelte';

  interface Props {
    item: TodoItem;
    itemTags?: Tag[];
    disabled?: boolean;
    longPressMs?: number;
    onToggle: (id: number) => void;
    onEdit: (id: number, text: string) => void;
    onOpenChange?: (id: number, open: boolean) => void;
    drawerContent?: Snippet<[{ item: TodoItem; closeDrawer: () => void }]>;
  }

  let {
    item,
    itemTags = [],
    disabled = false,
    longPressMs = 350,
    onToggle,
    onEdit,
    onOpenChange,
    drawerContent
  }: Props = $props();

  let displayTags = $derived(itemTags.slice(0, 2));
  let extraTagCount = $derived(Math.max(0, itemTags.length - 2));

  // Local state
  let open = $state(false);
  let timer: ReturnType<typeof setTimeout> | null = null;

  function toggleCheck() {
    if (disabled) return;
    onToggle(item.id);
  }

  function closeDrawer() {
    open = false;
    onOpenChange?.(item.id, false);
  }

  function startLongPress(_e: PointerEvent) {
    if (disabled) return;
    timer = setTimeout(() => {
      open = !open;
      onOpenChange?.(item.id, open);
    }, longPressMs);
  }

  function cancelLongPress() {
    if (timer) clearTimeout(timer);
    timer = null;
  }
</script>

<div
  class="outer"
  class:disabled
  class:done={item.done}
  role="listitem"
  onpointerdown={startLongPress}
  onpointerup={cancelLongPress}
  onpointercancel={cancelLongPress}
  onpointerleave={cancelLongPress}
>
  <!-- header row -->
  <div class="header">
    <button
      class="leafBox"
      type="button"
      onclick={(e) => { e.stopPropagation(); toggleCheck(); }}
      {disabled}
      aria-label={item.done ? 'Uncheck item' : 'Check item'}
    >
      {#if item.done}
        <svg class="check" viewBox="0 0 24 24" aria-hidden="true">
          <path d="M20 7L10 17l-5-5" />
        </svg>
      {/if}
    </button>

    <div class="main">
      <span class="text" class:done={item.done}>{item.text || '할 일을 입력해줘'}</span>
      {#if item.repeat_type !== 'none'}
        <span class="repeat-badge">
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
            <path d="M17 2.1l4 4-4 4"></path>
            <path d="M3 12.2v-2a4 4 0 0 1 4-4h12.8M7 21.9l-4-4 4-4"></path>
            <path d="M21 11.8v2a4 4 0 0 1-4 4H4.2"></path>
          </svg>
        </span>
      {/if}
      {#if displayTags.length > 0}
        <span class="tag-badges">
          {#each displayTags as tag (tag.id)}
            <span class="tag-badge">#{tag.name}</span>
          {/each}
          {#if extraTagCount > 0}
            <span class="tag-badge tag-extra">+{extraTagCount}</span>
          {/if}
        </span>
      {/if}
    </div>
  </div>

  <!-- drawer (slide only) -->
  {#if open && drawerContent}
    <div
      class="drawerArea"
      transition:slide={{ duration: 240, easing: cubicOut }}
    >
      <div class="drawerPanel">
        {@render drawerContent({ item, closeDrawer })}
      </div>
    </div>
  {/if}
</div>

<style>
  .outer {
    width: 100%;
    box-sizing: border-box;
    background: var(--color-paper);

    /* leaf shape: TL/BR angular, TR/BL rounded */
    border-radius: 0 20px 0 20px;
    border: 2px solid var(--color-ink);

    /* inset feel */
    padding: 12px;

    /* long-press & default tap response */
    touch-action: manipulation;

    /* default text size */
    font-size: 16px;
    color: var(--color-ink);
  }

  .outer.disabled {
    opacity: 0.6;
  }

  .outer.done {
    background: var(--color-canvas);
  }

  .header {
    height: 30px;
    display: flex;
    align-items: center;
    gap: 12px;
  }

  /* checkbox */
  .leafBox {
    width: 30px;
    height: 30px;
    flex: 0 0 30px;

    background: transparent;
    border: 2px solid var(--color-ink);
    border-radius: 4px;

    display: grid;
    place-items: center;
    padding: 0;
    cursor: pointer;
  }

  .leafBox:focus-visible {
    outline: 3px solid var(--color-ink);
    outline-offset: 4px;
  }

  .check {
    width: 21px;
    height: 21px;
    fill: none;
    stroke: var(--color-ink);
    stroke-width: 3.0;
    stroke-linecap: round;
    stroke-linejoin: round;
  }

  .main {
    flex: 1;
    min-width: 0;
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .repeat-badge {
    flex-shrink: 0;
    color: var(--color-accent-sky-strong);
    display: flex;
    align-items: center;
    opacity: 0.8;
  }

  .tag-badges {
    display: flex;
    align-items: center;
    gap: 6px;
    flex-shrink: 0;
  }

  .tag-badge {
    display: inline-block;
    padding: 0;
    background: none;
    font-size: 10px;
    color: var(--color-ink-muted);
    white-space: nowrap;
    max-width: 60px;
    overflow: hidden;
    text-overflow: ellipsis;
    line-height: 1.4;
  }

  .tag-extra {
    color: var(--color-ink-muted);
  }

  .text {
    display: block;
    line-height: 1.1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    opacity: 0.95;
  }

  .text.done {
    text-decoration: line-through;
    opacity: 0.5;
  }

  /* drawer inset area */
  .drawerArea {
    margin-top: 10px;
    position: relative;
    z-index: 20;
  }

  /* drawer also leaf shape */
  .drawerPanel {
    border: 2px solid var(--color-ink);
    background: var(--color-white);
    border-radius: 0 18px 0 18px;
    padding: 14px;
    box-sizing: border-box;
  }
</style>
