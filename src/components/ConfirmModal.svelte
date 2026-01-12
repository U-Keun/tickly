<script lang="ts">
  interface Props {
    show: boolean;
    title: string;
    message: string;
    confirmLabel?: string;
    cancelLabel?: string;
    confirmClass?: string;
    onConfirm: () => void;
    onCancel: () => void;
  }

  let {
    show,
    title,
    message,
    confirmLabel = '확인',
    cancelLabel = '취소',
    confirmClass = 'bg-blue-500 hover:bg-blue-600',
    onConfirm,
    onCancel
  }: Props = $props();
</script>

{#if show}
  <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50" onclick={onCancel}>
    <div class="bg-white rounded-lg p-6 max-w-sm mx-4" onclick={(e) => e.stopPropagation()}>
      <h3 class="text-lg font-semibold mb-2">{title}</h3>
      <p class="text-gray-600 mb-6 whitespace-pre-line">{message}</p>
      <div class="flex gap-3 justify-end">
        <button
          onclick={onCancel}
          class="px-4 py-2 text-gray-700 bg-gray-100 hover:bg-gray-200 rounded-lg"
        >
          {cancelLabel}
        </button>
        <button
          onclick={onConfirm}
          class="px-4 py-2 text-white rounded-lg {confirmClass}"
        >
          {confirmLabel}
        </button>
      </div>
    </div>
  </div>
{/if}
