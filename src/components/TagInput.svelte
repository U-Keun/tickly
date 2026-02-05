<script lang="ts">
  import type { Tag } from '../types';
  import { i18n } from '$lib/i18n';
  import TagChip from './TagChip.svelte';

  interface Props {
    currentTags: Tag[];
    allTags: Tag[];
    onAdd: (tagName: string) => void;
    onRemove: (tagId: number) => void;
  }

  let { currentTags, allTags, onAdd, onRemove }: Props = $props();

  let inputValue = $state('');
  let showSuggestions = $state(false);

  let suggestions = $derived.by(() => {
    if (!inputValue.trim()) return [];
    const query = inputValue.trim().toLowerCase();
    const currentIds = new Set(currentTags.map(t => t.id));
    return allTags
      .filter(t => !currentIds.has(t.id) && t.name.toLowerCase().includes(query))
      .slice(0, 5);
  });

  function handleInput() {
    showSuggestions = inputValue.trim().length > 0;
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter' && inputValue.trim()) {
      e.preventDefault();
      onAdd(inputValue.trim());
      inputValue = '';
      showSuggestions = false;
    }
    if (e.key === 'Escape') {
      showSuggestions = false;
    }
  }

  function selectSuggestion(tag: Tag) {
    onAdd(tag.name);
    inputValue = '';
    showSuggestions = false;
  }
</script>

<div class="tag-input-container">
  {#if currentTags.length > 0}
    <div class="current-tags">
      {#each currentTags as tag (tag.id)}
        <TagChip
          name={tag.name}
          removable={true}
          onRemove={() => onRemove(tag.id)}
        />
      {/each}
    </div>
  {/if}

  <div class="input-wrapper">
    <input
      type="text"
      bind:value={inputValue}
      oninput={handleInput}
      onkeydown={handleKeydown}
      onfocus={() => showSuggestions = inputValue.trim().length > 0}
      onblur={() => setTimeout(() => showSuggestions = false, 200)}
      class="tag-text-input"
      placeholder={i18n.t('tagPlaceholder')}
    />

    {#if showSuggestions && suggestions.length > 0}
      <div class="suggestions-dropdown">
        {#each suggestions as suggestion (suggestion.id)}
          <button
            type="button"
            class="suggestion-item"
            onmousedown={(e) => { e.preventDefault(); selectSuggestion(suggestion); }}
          >
            {suggestion.name}
          </button>
        {/each}
      </div>
    {/if}
  </div>
</div>

<style>
  .tag-input-container {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .current-tags {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
  }

  .input-wrapper {
    position: relative;
  }

  .tag-text-input {
    width: 100%;
    padding: 8px 10px;
    border: 1px solid var(--color-stroke);
    border-radius: 8px;
    font-size: 14px;
    box-sizing: border-box;
    background: var(--color-white);
    color: var(--color-ink);
  }

  .tag-text-input:focus {
    outline: none;
    border-color: var(--color-accent-sky);
  }

  .tag-text-input::placeholder {
    color: var(--color-ink-muted);
  }

  .suggestions-dropdown {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    background: var(--color-white);
    border: 1px solid var(--color-stroke);
    border-radius: 8px;
    margin-top: 4px;
    z-index: 10;
    overflow: hidden;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }

  .suggestion-item {
    display: block;
    width: 100%;
    padding: 8px 12px;
    text-align: left;
    background: none;
    border: none;
    cursor: pointer;
    font-size: 14px;
    color: var(--color-ink);
  }

  .suggestion-item:hover {
    background: var(--color-canvas);
  }

  .suggestion-item:not(:last-child) {
    border-bottom: 1px solid var(--color-stroke);
  }
</style>
