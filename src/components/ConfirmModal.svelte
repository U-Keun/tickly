<script lang="ts">
  import ModalWrapper from './ModalWrapper.svelte';

  interface Props {
    show: boolean;
    title: string;
    message: string;
    confirmLabel?: string;
    cancelLabel?: string;
    confirmStyle?: 'danger' | 'warning' | 'primary';
    onConfirm: () => void;
    onCancel: () => void;
  }

  let {
    show,
    title,
    message,
    confirmLabel = '확인',
    cancelLabel = '취소',
    confirmStyle = 'primary',
    onConfirm,
    onCancel
  }: Props = $props();
</script>

<ModalWrapper {show} onClose={onCancel}>
  <h3 class="modal-title">{title}</h3>
  <p class="modal-message">{message}</p>
  <div class="button-group">
    <button class="btn btn-cancel" onclick={onCancel}>
      {cancelLabel}
    </button>
    <button class="btn btn-confirm {confirmStyle}" onclick={onConfirm}>
      {confirmLabel}
    </button>
  </div>
</ModalWrapper>

<style>
  .modal-title {
    font-size: 18px;
    font-weight: 600;
    margin-bottom: 8px;
    color: var(--color-ink);
  }

  .modal-message {
    font-size: 15px;
    color: var(--color-ink-muted);
    margin-bottom: 24px;
    white-space: pre-line;
    line-height: 1.5;
  }

  .button-group {
    display: flex;
    gap: 12px;
    justify-content: flex-end;
  }

  .btn {
    padding: 12px 20px;
    border-radius: 10px;
    font-size: 15px;
    font-weight: 500;
    cursor: pointer;
    transition: background-color 0.2s;
    border: none;
  }

  .btn-cancel {
    background: var(--color-canvas);
    color: var(--color-ink-muted);
  }

  .btn-cancel:hover {
    background: var(--color-mist);
  }

  .btn-confirm {
    color: var(--color-white);
  }

  .btn-confirm.primary {
    background: var(--color-accent-sky-strong);
  }

  .btn-confirm.primary:hover {
    background: var(--color-accent-sky);
  }

  .btn-confirm.warning {
    background: var(--color-accent-peach-strong);
  }

  .btn-confirm.warning:hover {
    background: var(--color-accent-peach);
  }

  .btn-confirm.danger {
    background: #ef4444;
  }

  .btn-confirm.danger:hover {
    background: #dc2626;
  }
</style>
