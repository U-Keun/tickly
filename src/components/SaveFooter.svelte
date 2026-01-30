<script lang="ts">
  import type { Snippet } from 'svelte';
  import { i18n } from '$lib/i18n';

  interface Props {
    onSave: () => void | Promise<void>;
    disabled?: boolean;
  }

  let { onSave, disabled = false }: Props = $props();
  let isSaving = $state(false);

  async function handleClick() {
    if (disabled || isSaving) return;
    isSaving = true;
    try {
      await onSave();
    } finally {
      isSaving = false;
    }
  }
</script>

<div class="save-footer">
  <button
    class="save-btn"
    class:disabled={disabled}
    class:saving={isSaving}
    disabled={disabled || isSaving}
    onclick={handleClick}
  >
    {isSaving ? i18n.t('saving') : i18n.t('save')}
  </button>
</div>

<style>
  .save-footer {
    position: fixed;
    bottom: 0;
    left: 0;
    right: 0;
    height: 60px;
    min-height: 60px;
    padding: 0 16px;
    padding-bottom: 0;
    background: var(--color-paper);
    border-top: 1px solid var(--color-stroke);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .save-btn {
    width: 100%;
    padding: 0;
    background: var(--color-accent-sky-strong);
    color: var(--color-white);
    font-size: 15px;
    font-weight: 600;
    border: none;
    border-radius: 12px;
    cursor: pointer;
    transition: background 0.2s;
    height: 36px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .save-btn:hover:not(.disabled):not(.saving) {
    background: var(--color-accent-sky);
  }

  .save-btn.disabled,
  .save-btn.saving {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
