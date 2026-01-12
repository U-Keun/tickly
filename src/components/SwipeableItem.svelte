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
  let containerElement: HTMLDivElement;

  const SWIPE_THRESHOLD = 100; // pixels to trigger reveal
  const DELETE_BUTTON_WIDTH = 150; // pixels

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
  }

  function handleTouchCancel() {
    translateX = 0;
    swipeState = 'idle';
    isDraggingHorizontally = false;
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
      : 'none'};"
  >
    {@render children()}
  </div>

  <!-- Delete Button (revealed on swipe) -->
  {#if swipeState === 'revealed' || swipeState === 'swiping'}
    <div
      class="swipe-delete-button"
      style="opacity: {Math.min(1, Math.abs(translateX) / DELETE_BUTTON_WIDTH)};"
    >
      <button onclick={handleDelete} class="delete-button-inner">
        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
          />
        </svg>
        <span class="text-sm font-semibold">삭제</span>
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
    background: white;
    will-change: transform;
  }

  .swipe-delete-button {
    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
    width: 150px;
    background: #ef4444; /* red-500 */
    display: flex;
    align-items: center;
    justify-content: center;
    pointer-events: all;
  }

  .delete-button-inner {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.25rem;
    color: white;
    padding: 1rem;
    background: none;
    border: none;
    cursor: pointer;
    width: 100%;
    height: 100%;
  }

  .delete-button-inner:active {
    background: #dc2626; /* red-600 */
  }
</style>
