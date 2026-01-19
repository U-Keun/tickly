<script lang="ts">
  import { slide } from 'svelte/transition';
  import { cubicOut } from 'svelte/easing';
  import type { TodoItem } from '../types';
  import type { Snippet } from 'svelte';

  interface Props {
    item: TodoItem;
    disabled?: boolean;
    longPressMs?: number;
    onToggle: (id: number) => void;
    onEdit: (id: number, text: string) => void;
    onOpenChange?: (id: number, open: boolean) => void;
    drawerContent?: Snippet<[{ item: TodoItem; closeDrawer: () => void }]>;
  }

  let {
    item,
    disabled = false,
    longPressMs = 350,
    onToggle,
    onEdit,
    onOpenChange,
    drawerContent
  }: Props = $props();

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

  /* leaf checkbox */
  .leafBox {
    width: 30px;
    height: 30px;
    flex: 0 0 30px;

    background: transparent;
    border: 2px solid var(--color-ink);
    border-radius: 0 10px 0 10px;

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
