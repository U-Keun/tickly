<script lang="ts">
  import { slide } from 'svelte/transition';
  import { cubicOut } from 'svelte/easing';
  import ModalWrapper from './ModalWrapper.svelte';
  import { i18n } from '$lib/i18n';
  import {
    PRESET_APPS,
    getAppLabel,
    loadCustomApps,
    getCustomApps,
    addCustomApp,
    removeCustomApp,
    type CustomApp,
  } from '$lib/linkedApps';

  interface Props {
    show: boolean;
    selectedKey: string | null;
    onSelect: (key: string | null) => void;
    onClose: () => void;
  }

  let { show, selectedKey, onSelect, onClose }: Props = $props();

  let customApps = $state<CustomApp[]>([]);
  let showCustomForm = $state(false);
  let customAppName = $state('');
  let customAppUrl = $state('');

  $effect(() => {
    if (show) {
      loadCustomApps().then(apps => { customApps = apps; });
      showCustomForm = false;
      customAppName = '';
      customAppUrl = '';
    }
  });

  function handleSelect(key: string) {
    onSelect(key);
    onClose();
  }

  async function handleAddCustomApp() {
    const name = customAppName.trim();
    const url = customAppUrl.trim();
    if (!name || !url) return;
    const app = await addCustomApp(name, url);
    customApps = getCustomApps();
    showCustomForm = false;
    customAppName = '';
    customAppUrl = '';
    handleSelect(app.key);
  }

  async function handleRemoveCustomApp(key: string) {
    await removeCustomApp(key);
    customApps = getCustomApps();
    if (selectedKey === key) {
      onSelect(null);
    }
  }
</script>

<ModalWrapper {show} {onClose} size="sm" position="center">
  <h3 class="modal-title">{i18n.t('linkedAppConnect')}</h3>

  <div class="app-list">
    {#each PRESET_APPS as app (app.key)}
      <button
        type="button"
        class="app-item"
        class:active={selectedKey === app.key}
        onclick={() => handleSelect(app.key)}
      >
        <span class="app-icon">{app.icon}</span>
        <span class="app-name">{getAppLabel(app.key, i18n.locale)}</span>
        {#if selectedKey === app.key}
          <svg class="check-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="20 6 9 17 4 12"></polyline>
          </svg>
        {/if}
      </button>
    {/each}

    {#each customApps as app (app.key)}
      <div class="app-item custom-item" class:active={selectedKey === app.key}>
        <button
          type="button"
          class="custom-select"
          onclick={() => handleSelect(app.key)}
        >
          <span class="app-icon">🔗</span>
          <span class="app-name">{app.name}</span>
          {#if selectedKey === app.key}
            <svg class="check-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="20 6 9 17 4 12"></polyline>
            </svg>
          {/if}
        </button>
        <button
          type="button"
          class="custom-delete"
          onclick={() => handleRemoveCustomApp(app.key)}
          aria-label="Delete"
        >
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="18" y1="6" x2="6" y2="18"></line>
            <line x1="6" y1="6" x2="18" y2="18"></line>
          </svg>
        </button>
      </div>
    {/each}

    <button
      type="button"
      class="app-item custom-add-item"
      onclick={() => showCustomForm = !showCustomForm}
    >
      <span class="app-icon add-icon">+</span>
      <span class="app-name">{i18n.t('linkedAppCustomAdd')}</span>
    </button>
  </div>

  {#if showCustomForm}
    <div class="custom-form" transition:slide={{ duration: 200, easing: cubicOut }}>
      <input
        type="text"
        class="form-input"
        bind:value={customAppName}
        placeholder={i18n.t('linkedAppNamePlaceholder')}
      />
      <input
        type="text"
        class="form-input"
        bind:value={customAppUrl}
        placeholder={i18n.t('linkedAppUrlPlaceholder')}
      />
      <button
        type="button"
        class="form-add-btn"
        onclick={handleAddCustomApp}
        disabled={!customAppName.trim() || !customAppUrl.trim()}
      >
        {i18n.t('add')}
      </button>
    </div>
  {/if}

  {#if selectedKey}
    <button
      type="button"
      class="clear-btn"
      onclick={() => { onSelect(null); onClose(); }}
    >
      {i18n.t('linkedAppNone')}
    </button>
  {/if}
</ModalWrapper>

<style>
  .modal-title {
    font-size: 18px;
    font-weight: 600;
    color: var(--color-ink);
    margin: 0 0 16px 0;
  }

  .app-list {
    display: flex;
    flex-direction: column;
    gap: 2px;
    max-height: 320px;
    overflow-y: auto;
  }

  .app-item {
    display: flex;
    align-items: center;
    gap: 12px;
    width: 100%;
    padding: 10px 12px;
    background: none;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    text-align: left;
    transition: background 0.15s;
    color: var(--color-ink);
  }

  .app-item:hover {
    background: var(--color-canvas);
  }

  .app-item.active {
    background: var(--color-accent-sky);
  }

  .app-icon {
    font-size: 18px;
    width: 24px;
    text-align: center;
    flex-shrink: 0;
  }

  .add-icon {
    font-size: 20px;
    color: var(--color-ink-muted);
  }

  .app-name {
    flex: 1;
    font-size: 15px;
    color: var(--color-ink);
  }

  .check-icon {
    flex-shrink: 0;
    color: var(--color-accent-sky-strong);
  }

  .custom-item {
    padding: 0;
    display: flex;
    border-radius: 8px;
    overflow: hidden;
  }

  .custom-select {
    flex: 1;
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 10px 12px;
    background: none;
    border: none;
    cursor: pointer;
    text-align: left;
    color: var(--color-ink);
    transition: background 0.15s;
  }

  .custom-select:hover {
    background: var(--color-canvas);
  }

  .custom-delete {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    padding: 0;
    background: none;
    border: none;
    cursor: pointer;
    color: var(--color-ink-muted);
    transition: background 0.15s;
  }

  .custom-delete:hover {
    background: var(--color-mist);
    color: var(--color-ink);
  }

  .custom-add-item {
    color: var(--color-ink-muted);
    border: 1px dashed var(--color-stroke);
    margin-top: 6px;
  }

  .custom-form {
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin-top: 8px;
    padding: 12px;
    background: var(--color-canvas);
    border-radius: 8px;
  }

  .form-input {
    width: 100%;
    padding: 8px 10px;
    border: 1px solid var(--color-stroke);
    border-radius: 8px;
    font-size: 14px;
    background: var(--color-white);
    color: var(--color-ink);
    box-sizing: border-box;
  }

  .form-input:focus {
    outline: none;
    border-color: var(--color-accent-sky);
  }

  .form-input::placeholder {
    color: var(--color-ink-muted);
  }

  .form-add-btn {
    align-self: flex-end;
    padding: 8px 16px;
    border: none;
    border-radius: 8px;
    background: var(--color-accent-sky);
    color: var(--color-ink);
    font-size: 14px;
    cursor: pointer;
    transition: background 0.15s;
  }

  .form-add-btn:hover {
    background: var(--color-accent-sky-strong);
  }

  .form-add-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .clear-btn {
    display: block;
    width: 100%;
    margin-top: 12px;
    padding: 10px;
    background: var(--color-canvas);
    border: none;
    border-radius: 8px;
    font-size: 14px;
    color: var(--color-ink-muted);
    cursor: pointer;
    transition: background 0.15s;
  }

  .clear-btn:hover {
    background: var(--color-mist);
  }
</style>
