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
  let editable = $state(false);
  let editText = $state('');
  let timer: ReturnType<typeof setTimeout> | null = null;
  let inputElement = $state<HTMLInputElement | null>(null);

  function toggleCheck() {
    if (disabled) return;
    onToggle(item.id);
  }

  function startEdit() {
    if (disabled) return;
    editable = true;
    editText = item.text;
    setTimeout(() => inputElement?.focus(), 0);
  }

  function finishEdit() {
    if (!editable) return;
    const trimmed = editText.trim();
    if (trimmed && trimmed !== item.text) {
      onEdit(item.id, trimmed);
    }
    editable = false;
  }

  function onKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter') {
      e.preventDefault();
      (e.target as HTMLInputElement).blur();
      finishEdit();
    } else if (e.key === 'Escape') {
      e.preventDefault();
      editable = false;
    }
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
      {#if editable}
        <input
          bind:this={inputElement}
          bind:value={editText}
          onkeydown={onKeydown}
          onblur={finishEdit}
          class="textInput"
          type="text"
          {disabled}
          placeholder="할 일을 입력해줘"
        />
      {:else}
        <span class="text" class:done={item.done}>{item.text || '할 일을 입력해줘'}</span>
      {/if}
    </div>

    <button
      class="editBtn"
      type="button"
      onclick={(e) => { e.stopPropagation(); startEdit(); }}
      {disabled}
      aria-label={editable ? 'Finish edit' : 'Edit item'}
    >
      <svg class="editIcon" viewBox="0 0 24 24" fill="none" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
      </svg>
    </button>
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
    background: #faf6f1;

    /* leaf shape: TL/BR angular, TR/BL rounded */
    border-radius: 0 20px 0 20px;
    border: 2px solid #333;

    /* inset feel */
    padding: 12px;

    /* long-press & default tap response */
    touch-action: manipulation;

    /* default text size */
    font-size: 16px;
  }

  .outer.disabled {
    opacity: 0.6;
  }

  .outer.done {
    background: #f0ebe5;
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
    border: 2px solid #333;
    border-radius: 0 10px 0 10px;

    display: grid;
    place-items: center;
    padding: 0;
    cursor: pointer;
  }

  .leafBox:focus-visible {
    outline: 3px solid #000;
    outline-offset: 4px;
  }

  .check {
    width: 21px;
    height: 21px;
    fill: none;
    stroke: #333;
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

  .textInput {
    width: 100%;
    line-height: 1.1;
    font-size: 16px;
    border: 0;
    outline: none;
    background: transparent;
    padding: 0;
  }

  .textInput::placeholder {
    opacity: 0.5;
  }

  .editBtn {
    flex: 0 0 auto;
    display: inline-flex;
    align-items: center;
    justify-content: center;

    min-width: 30px;
    height: 30px;

    border: 0;
    background: transparent;
    padding: 0;
    cursor: pointer;
    color: #666;
  }

  .editBtn:focus-visible {
    outline: 3px solid #000;
    outline-offset: 4px;
  }

  .editIcon {
    width: 20px;
    height: 20px;
  }

  /* drawer inset area */
  .drawerArea {
    margin-top: 10px;
  }

  /* drawer also leaf shape */
  .drawerPanel {
    border: 2px solid #333;
    background: #fff;
    border-radius: 0 18px 0 18px;
    padding: 14px;
    box-sizing: border-box;
  }
</style>
