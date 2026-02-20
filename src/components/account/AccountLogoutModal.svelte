<script lang="ts">
  import { i18n } from '$lib/i18n';
  import ModalWrapper from '../ModalWrapper.svelte';

  let {
    show = false,
    onClose,
    onConfirm
  } = $props<{
    show?: boolean;
    onClose: () => void;
    onConfirm: () => void | Promise<void>;
  }>();
</script>

<ModalWrapper {show} {onClose}>
  <div class="confirm-modal">
    <h3>{i18n.t('logout')}</h3>
    <p>{i18n.t('logoutConfirm')}</p>
    <div class="modal-buttons">
      <button class="cancel-btn" onclick={onClose}>
        {i18n.t('cancel')}
      </button>
      <button class="confirm-btn" onclick={onConfirm}>
        {i18n.t('logout')}
      </button>
    </div>
  </div>
</ModalWrapper>

<style>
  .confirm-modal {
    padding: 24px;
    text-align: center;
  }

  .confirm-modal h3 {
    font-size: 18px;
    font-weight: 600;
    color: var(--color-ink);
    margin-bottom: 12px;
  }

  .confirm-modal p {
    font-size: 15px;
    color: var(--color-ink-muted);
    margin-bottom: 24px;
  }

  .modal-buttons {
    display: flex;
    gap: 12px;
  }

  .modal-buttons button {
    flex: 1;
    padding: 12px;
    border-radius: 8px;
    font-size: 15px;
    font-weight: 500;
    cursor: pointer;
  }

  .cancel-btn {
    background: var(--color-mist);
    border: none;
    color: var(--color-ink);
  }

  .confirm-btn {
    background: var(--color-accent-peach-strong);
    border: none;
    color: var(--color-white);
  }
</style>
