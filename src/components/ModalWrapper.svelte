<script lang="ts">
  import type { Snippet } from 'svelte';
  import { fly, fade } from 'svelte/transition';
  import { cubicOut } from 'svelte/easing';

  interface Props {
    show: boolean;
    onClose: () => void;
    size?: 'sm' | 'md';
    position?: 'center' | 'top';
    children: Snippet;
  }

  let {
    show,
    onClose,
    size = 'sm',
    position = 'center',
    children,
  }: Props = $props();

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      event.preventDefault();
      onClose();
    }
  }

  const sizeClass = {
    sm: 'max-w-sm',
    md: 'max-w-md',
  };

  const positionClass = {
    center: 'items-center',
    top: 'items-start pt-[15vh]',
  };
</script>

{#if show}
  <div
    class="modal-overlay {positionClass[position]}"
    role="dialog"
    aria-modal="true"
    tabindex="0"
    onclick={onClose}
    onkeydown={handleKeydown}
    transition:fade={{ duration: 200 }}
  >
    <div
      class="modal-content {sizeClass[size]}"
      onclick={(e) => e.stopPropagation()}
      in:fly={{ y: 50, duration: 300, easing: cubicOut }}
      out:fly={{ y: 30, duration: 200, easing: cubicOut }}
    >
      {@render children()}
    </div>
  </div>
{/if}

<style>
  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
    z-index: 50;
    overflow-y: auto;
    -webkit-overflow-scrolling: touch;
    padding-bottom: env(safe-area-inset-bottom, 0);
  }

  .modal-content {
    background: var(--color-white);
    border-radius: 16px;
    padding: 24px;
    width: 100%;
    margin: 0 16px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
    height: fit-content;
    overflow-x: hidden;
  }
</style>
