<script lang="ts">
  import type { TodoItem } from '../types';

  interface Props {
    item: TodoItem;
    onDelete: (id: number) => void;
    children: any;
  }

  let { item, onDelete, children }: Props = $props();

  let swipeState = $state<'idle' | 'swiping' | 'revealed'>('idle');
  let translateX = $state(0);
  let startX = $state(0);
  let startY = $state(0);
  let currentX = $state(0);
  let isDraggingHorizontally = $state(false);
  let shouldBlockPointer = $state(false);
  let containerElement: HTMLDivElement;

  const SWIPE_THRESHOLD = 36; // pixels to trigger reveal
  const DELETE_BUTTON_WIDTH = 44; // pixels (button + left margin)

  function handleTouchStart(e: TouchEvent) {
    const touch = e.touches[0];

    // Allow swipe from anywhere
    startX = touch.clientX;
    startY = touch.clientY;
    currentX = touch.clientX;
    isDraggingHorizontally = false;
    swipeState = 'swiping';
  }

  function handleTouchMove(e: TouchEvent) {
    if (swipeState !== 'swiping' && swipeState !== 'revealed') return;

    const touch = e.touches[0];
    currentX = touch.clientX;
    const deltaX = currentX - startX;
    const deltaY = touch.clientY - startY;

    // Determine if this is a horizontal swipe (earlier detection)
    if (!isDraggingHorizontally && (Math.abs(deltaX) > 5 || Math.abs(deltaY) > 5)) {
      isDraggingHorizontally = Math.abs(deltaX) > Math.abs(deltaY);

      // If it's vertical, reset state to allow scroll
      if (!isDraggingHorizontally) {
        swipeState = 'idle';
        return;
      }

      // Horizontal swipe detected - block pointer events to prevent long-press
      shouldBlockPointer = true;
    }

    // Only handle horizontal swipes
    if (isDraggingHorizontally) {
      // Prevent default to stop vertical scroll during horizontal swipe
      e.preventDefault();

      // Only allow leftward swipe (negative deltaX)
      // Limit to -DELETE_BUTTON_WIDTH
      translateX = Math.max(-DELETE_BUTTON_WIDTH, Math.min(0, deltaX));
    }
  }

  function handleTouchEnd() {
    if (swipeState !== 'swiping') return;

    // If swiped past threshold, reveal delete button
    if (translateX <= -SWIPE_THRESHOLD) {
      translateX = -DELETE_BUTTON_WIDTH;
      swipeState = 'revealed';
    } else {
      // Bounce back to idle
      translateX = 0;
      swipeState = 'idle';
    }

    isDraggingHorizontally = false;

    // Delay restoring pointer events to prevent accidental long-press
    setTimeout(() => {
      shouldBlockPointer = false;
    }, 500);
  }

  function handleTouchCancel() {
    translateX = 0;
    swipeState = 'idle';
    isDraggingHorizontally = false;
    shouldBlockPointer = false;
  }

  function handleDelete(e: Event) {
    e.stopPropagation();
    onDelete(item.id);
  }

  function closeSwipe(e?: Event) {
    if (swipeState === 'revealed') {
      // Prevent event propagation to avoid interfering with button clicks
      e?.stopPropagation();
      e?.preventDefault();
      translateX = 0;
      swipeState = 'idle';
    }
  }

  // Export method for parent to close swipe
  export function reset() {
    closeSwipe();
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
<div
  bind:this={containerElement}
  class="swipeable-container"
  ontouchstart={handleTouchStart}
  ontouchmove={handleTouchMove}
  ontouchend={handleTouchEnd}
  ontouchcancel={handleTouchCancel}
  onclick={closeSwipe}
>
  <!-- Swipe Content -->
  <div
    class="swipe-content"
    style="transform: translateX({translateX}px); transition: {swipeState === 'idle'
      ? 'transform 0.3s cubic-bezier(0.4, 0, 0.2, 1)'
      : 'none'}; pointer-events: {shouldBlockPointer ? 'none' : 'auto'};"
  >
    {@render children()}
  </div>

  <!-- Delete Button (revealed on swipe) -->
  {#if swipeState === 'revealed' || swipeState === 'swiping'}
    <div
      class="swipe-delete-button"
      style="opacity: {Math.min(1, Math.abs(translateX) / DELETE_BUTTON_WIDTH)};"
    >
      <button onclick={handleDelete} class="delete-button-inner" aria-label="삭제" title="삭제">
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
          />
        </svg>
      </button>
    </div>
  {/if}
</div>

<style>
  .swipeable-container {
    position: relative;
    overflow: hidden;
    touch-action: pan-y; /* Allow vertical scrolling */
  }

  .swipe-content {
    position: relative;
    background: transparent;
    will-change: transform;
  }

  .swipe-delete-button {
    position: absolute;
    top: 50%;
    right: 0;
    transform: translateY(-50%);
    width: 44px;
    height: 36px;
    display: flex;
    align-items: center;
    justify-content: flex-end;
    pointer-events: all;
  }

  .delete-button-inner {
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
    padding: 0;
    background: #ef4444; /* red-500 */
    border: none;
    cursor: pointer;
    width: 36px;
    height: 36px;
    border-radius: 0 10px 0 10px;
  }

  .delete-button-inner:active {
    background: #dc2626; /* red-600 */
  }
</style>
