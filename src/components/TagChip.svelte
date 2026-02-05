<script lang="ts">
  interface Props {
    name: string;
    removable?: boolean;
    onclick?: () => void;
    onRemove?: () => void;
  }

  let { name, removable = false, onclick, onRemove }: Props = $props();
</script>

<!-- svelte-ignore a11y_no_noninteractive_tabindex -->
<span
  class="tag-chip"
  class:clickable={!!onclick}
  role={onclick ? 'button' : undefined}
  tabindex={onclick ? 0 : undefined}
  onclick={(e) => {
    if (onclick) {
      e.stopPropagation();
      onclick();
    }
  }}
  onkeydown={(e) => {
    if (onclick && (e.key === 'Enter' || e.key === ' ')) {
      e.preventDefault();
      onclick();
    }
  }}
>
  <span class="tag-name">#{name}</span>
  {#if removable && onRemove}
    <button
      type="button"
      class="tag-remove"
      onclick={(e) => {
        e.stopPropagation();
        onRemove?.();
      }}
      aria-label="Remove tag"
    >
      <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
        <line x1="18" y1="6" x2="6" y2="18"></line>
        <line x1="6" y1="6" x2="18" y2="18"></line>
      </svg>
    </button>
  {/if}
</span>

<style>
  .tag-chip {
    display: inline-flex;
    align-items: center;
    gap: 2px;
    padding: 0;
    background: none;
    border: none;
    font-size: 12px;
    color: var(--color-ink-muted);
    white-space: nowrap;
    cursor: default;
    line-height: 1.4;
  }

  .tag-chip.clickable {
    cursor: pointer;
  }

  .tag-chip.clickable:hover {
    color: var(--color-ink);
  }

  .tag-name {
    max-width: 80px;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .tag-remove {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 16px;
    height: 16px;
    padding: 0;
    background: none;
    border: none;
    cursor: pointer;
    color: var(--color-ink-muted);
    border-radius: 50%;
  }

  .tag-remove:hover {
    color: var(--color-ink);
    background: var(--color-mist);
  }
</style>
