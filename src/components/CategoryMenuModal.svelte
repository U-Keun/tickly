<script lang="ts">
  import type { Category } from '../types';
  import { i18n } from '$lib/i18n';

  interface Props {
    show: boolean;
    category: Category | null;
    onEdit: () => void;
    onDelete: () => void;
    onClose: () => void;
  }

  let { show, category, onEdit, onDelete, onClose }: Props = $props();
</script>

{#if show && category}
  <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50" onclick={onClose}>
    <div class="bg-white rounded-lg p-6 max-w-sm mx-4 min-w-[280px]" onclick={(e) => e.stopPropagation()}>
      <h3 class="text-lg font-semibold mb-4 text-center">{category.name}</h3>
      <div class="flex flex-col gap-2">
        <button
          onclick={onEdit}
          class="px-4 py-3 text-blue-600 bg-blue-50 hover:bg-blue-100 rounded-lg text-left font-medium"
        >
          {i18n.t('editName')}
        </button>
        <button
          onclick={onDelete}
          class="px-4 py-3 text-red-600 bg-red-50 hover:bg-red-100 rounded-lg text-left font-medium"
        >
          {i18n.t('categoryDelete')}
        </button>
        <button
          onclick={onClose}
          class="px-4 py-3 text-gray-700 bg-gray-100 hover:bg-gray-200 rounded-lg text-left font-medium"
        >
          {i18n.t('cancel')}
        </button>
      </div>
    </div>
  </div>
{/if}
